use serde::Serialize;
use std::path::PathBuf;

/// A position in the *original* source file. Column counting assumes one
/// byte == one column, which holds for the ASCII-only linuxdoom sources this
/// parser targets but would be wrong for multi-byte UTF-8 input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub line: u32,
    pub col: u32,
    pub byte: usize,
}

impl Pos {
    pub fn start() -> Self {
        Pos {
            line: 1,
            col: 1,
            byte: 0,
        }
    }

    /// Advance this position by the raw text `s` (which must be the exact
    /// original bytes starting at this position).
    pub fn advance(mut self, s: &str) -> Self {
        for b in s.bytes() {
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.byte += 1;
        }
        self
    }
}

/// A contiguous, exact slice of the original source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
    pub text: String,
}

/// One classified piece of the raw source: a comment, a string/char literal,
/// a (backslash-continuation-joined) preprocessor line, or a run of
/// everything else ("code"). Every byte of the file ends up in exactly one
/// RawToken, so concatenating their `.text()` reproduces the file exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawToken {
    LineComment(Span),
    BlockComment(Span),
    StringLit(Span),
    CharLit(Span),
    PreprocLine(Span),
    Code(Span),
}

impl RawToken {
    pub fn span(&self) -> &Span {
        match self {
            RawToken::LineComment(s)
            | RawToken::BlockComment(s)
            | RawToken::StringLit(s)
            | RawToken::CharLit(s)
            | RawToken::PreprocLine(s)
            | RawToken::Code(s) => s,
        }
    }

    pub fn text(&self) -> &str {
        &self.span().text
    }
}

/// Step 1's brace grouping: a flat (non-recursive) split of a token stream
/// into depth-0 runs and matched `{ ... }` groups. A `Group`'s `inner` is
/// stored opaque/unparsed even if it contains further nested braces -
/// callers that need to look one level deeper call `group_braces` again on
/// `inner`.
#[derive(Debug, Clone)]
pub enum Chunk {
    Flat(Vec<RawToken>),
    Group {
        open: Span,
        close: Span,
        inner: Vec<RawToken>,
    },
}

impl Chunk {
    /// Exact original text this chunk covers.
    pub fn render(&self) -> String {
        match self {
            Chunk::Flat(toks) => toks.iter().map(|t| t.text()).collect(),
            Chunk::Group { open, inner, close } => {
                let mut s = open.text.clone();
                for t in inner {
                    s.push_str(t.text());
                }
                s.push_str(&close.text);
                s
            }
        }
    }
}

pub fn render_chunks(chunks: &[Chunk]) -> String {
    chunks.iter().map(Chunk::render).collect()
}

pub fn render_tokens(tokens: &[RawToken]) -> String {
    tokens.iter().map(RawToken::text).collect()
}

/// Same as `render_tokens`, but drops comments. Used wherever comment text
/// is rendered only to be split on a delimiter (`;` for struct fields, `,`
/// for enum variants) - a comment containing that delimiter (e.g. `// Use
/// button, or something` before a `,`-terminated enum variant) would
/// otherwise fracture into bogus extra pieces.
pub fn render_tokens_no_comments(tokens: &[RawToken]) -> String {
    tokens
        .iter()
        .filter(|t| !matches!(t, RawToken::LineComment(_) | RawToken::BlockComment(_)))
        .map(RawToken::text)
        .collect()
}

/// Splits `s` on top-level occurrences of `sep`, treating `(...)`, `[...]`
/// and `{...}` as opaque (never splitting inside them). Used for enum
/// variant lists, struct field lists, and braced-initializer element lists
/// once they're back down to plain text.
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                cur.push(c);
            }
            c if c == sep && depth <= 0 => {
                out.push(std::mem::take(&mut cur));
            }
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

#[derive(Debug, Clone, Serialize)]
pub enum Comment {
    Line(String),
    Block(String),
}

impl Comment {
    pub fn text(&self) -> &str {
        match self {
            Comment::Line(s) | Comment::Block(s) => s,
        }
    }
}

/// Comments attached to an item.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Trivia {
    pub leading: Vec<Comment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecordKind {
    Struct,
    Union,
}

/// A declarator's type, recursively structured instead of kept as raw text.
/// No qualifiers (`const`/`volatile`) live here - those are storage-class
/// keywords, kept in whichever `storage: Vec<String>` field sits alongside
/// a `Type` (this parser doesn't distinguish `const` applying to the
/// pointer vs. the pointee - `const char *` and `char * const` both just
/// contribute "const" to the same flat `storage` list, same as before this
/// type existed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Type {
    /// A base type name, e.g. `"int"`, `"mobj_t"`, `"struct foo"` (the
    /// short label used for an anonymous nested record field's outer type -
    /// see `Field::nested`), or - as a last-resort fallback - an entire
    /// unparsed blob of text this parser couldn't make sense of (e.g. an
    /// opaque nested-group field's raw `{ ... }` body).
    Named(String),
    /// One level of pointer indirection, e.g. `char *` is
    /// `Pointer(Named("char"))`; `char **` is `Pointer(Pointer(Named("char")))`.
    Pointer(Box<Type>),
    /// An array of `dim` (or unsized, if `None`, e.g. `int xs[]`) elements
    /// of the inner type. For a multi-dimensional array (`int m[3][4]`),
    /// the outermost `Array` corresponds to the *first* bracket - `m` is an
    /// array of 3 (array of 4 int), i.e.
    /// `Array(Array(Named("int"), Some("4")), Some("3"))`.
    Array(Box<Type>, Option<String>),
    /// A function-pointer type, e.g. `void (*)(int, int)` or a named
    /// typedef's underlying type (`p_local.h`'s `traverser_t`). `params`
    /// holds each parameter's type only (any parameter *names* inside the
    /// pointer's own parens, e.g. `traverser_t`'s `(intercept_t *in)`, are
    /// discarded - out of scope here, unlike `FnSig::params`, which is a
    /// real function's own parameter list and does keep names).
    FunctionPointer { ret: Box<Type>, params: Vec<Type> },
}

#[derive(Debug, Clone, Serialize)]
pub struct Field {
    pub ty: Type,
    pub name: String,
    /// Storage-class/qualifier keywords preceding the type (`const`,
    /// `volatile`, ...) - almost always empty for a struct/union field, but
    /// kept for parity with `ConstDecl`/`FnSig`/`Param` rather than
    /// silently dropped or glued into `ty`'s text the way it used to be.
    pub storage: Vec<String>,
    pub bitfield: Option<String>,
    /// Set when this field is itself an anonymous nested struct/union (a
    /// literal `{ ... }` body with no `typedef`, e.g. `union { ... } d;`
    /// inside another struct) - recursively parsed rather than kept as one
    /// opaque field. `ty`/`name` still describe the *outer* declarator
    /// (`d`); `ty` is `Type::Named("union")` (or `"struct foo"` with a tag)
    /// rather than the full nested body, whose structure lives here instead.
    pub nested: Option<Box<RecordDecl>>,
    /// Doc comment(s) on their own line(s) immediately preceding this field.
    pub trivia: Trivia,
    /// A comment on the same line as (and after) this field's `;`, e.g.
    /// `int health; // hit points`.
    pub trailing_comment: Option<Comment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordDecl {
    pub kind: RecordKind,
    pub tag: Option<String>,
    pub fields: Vec<Field>,
    /// Declarator names following the closing brace, e.g. `} foo, *foo_p;`.
    pub names: Vec<String>,
    /// If this came from `typedef struct { ... } name;`, the typedef name.
    pub typedef_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
    /// Doc comment(s) on their own line(s) immediately preceding this variant.
    pub trivia: Trivia,
    /// A comment on the same line as (and after) this variant's `,`, e.g.
    /// `INVULN, // invulnerability`.
    pub trailing_comment: Option<Comment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnumDecl {
    pub tag: Option<String>,
    pub variants: Vec<EnumVariant>,
    pub names: Vec<String>,
    pub typedef_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypedefDecl {
    pub underlying: Type,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum Init {
    /// A `{ ... }` initializer, split on top-level `,` into its elements -
    /// each either a nested `Braced` sub-list (e.g. one row of a
    /// `mobjinfo[]`/`states[]`-style table) or a scalar `Expr`. No operator
    /// grammar: an element's own text (`(30*TICRATE)`, `"a"`, `&foo`, ...)
    /// is kept as raw text either way - only the comma-separated *shape* of
    /// the initializer is structured.
    Braced(Vec<Init>),
    Expr(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstDecl {
    pub storage: Vec<String>,
    pub ty: Type,
    pub name: String,
    pub initializer: Option<Init>,
}

/// One entry in a function signature's parameter list, e.g. `int x` or
/// `char *s`. `name` is empty for an anonymous parameter (a bare type with
/// no identifier, e.g. an old K&R-style forward declaration's `int foo(int,
/// char*);`) - in that case `ty` is `Type::Named` holding the parameter's
/// entire text verbatim rather than attempting to guess where a type ends
/// and a name begins.
#[derive(Debug, Clone, Serialize)]
pub struct Param {
    pub ty: Type,
    pub name: String,
    pub storage: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FnSig {
    pub storage: Vec<String>,
    pub ret_ty: Type,
    pub name: String,
    /// The parameter list, split on top-level `,`. Empty for both `()`
    /// (old-style "unspecified parameters") and `(void)` (explicitly no
    /// parameters) - that distinction isn't preserved structurally, though
    /// it's still visible in the containing `Item.raw`.
    pub params: Vec<Param>,
    /// True if the parameter list ends in `...` (e.g. `I_Error`'s
    /// `(char *error, ...)`).
    pub variadic: bool,
}

/// One `#if`/`#ifdef`/`#ifndef` branch (or a following `#elif`): the
/// directive that opened it, plus the (recursively folded) items in its body.
#[derive(Debug, Clone, Serialize)]
pub struct CondBranch {
    pub directive: super::preproc::Directive,
    pub body: Vec<(Item, Trivia)>,
}

/// A folded `#if...#endif` run. `branches[0]` is the opening `#if`/`#ifdef`/
/// `#ifndef`; any further entries are `#elif`s. `else_body` is the `#else`
/// body, if present. The matching `#endif` itself isn't modeled separately -
/// it's implicit in where the group ends, and its exact text (like
/// everything else in the group) lives in the containing `Item.raw`.
#[derive(Debug, Clone, Serialize)]
pub struct CondGroup {
    pub branches: Vec<CondBranch>,
    pub else_body: Option<Vec<(Item, Trivia)>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ItemKind {
    Preproc(super::preproc::Directive),
    Record(RecordDecl),
    Enum(EnumDecl),
    Typedef(TypedefDecl),
    Const(ConstDecl),
    FunctionDecl(FnSig),
    /// Signature + fully opaque, never-descended-into body text (including
    /// the surrounding braces).
    FunctionDef(FnSig, String),
    /// A folded `#if`/`#ifdef`/`#ifndef` ... `#endif` run. See `cond.rs`.
    Conditional(CondGroup),
    /// Fallback for anything v1 doesn't structurally recognize yet. `raw`
    /// (on the containing `Item`) already holds the exact text.
    Raw,
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub kind: ItemKind,
    /// Exact original text of this item, excluding its leading/trailing
    /// trivia. This is the round-trip source of truth - the structured
    /// fields in `kind` are best-effort derived data on top of it.
    pub raw: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub path: PathBuf,
    pub items: Vec<(Item, Trivia)>,
}

impl File {
    /// Reconstructs the exact original file text from the AST.
    pub fn render(&self) -> String {
        render_items(&self.items)
    }
}

/// Reconstructs the exact original text spanned by `items` (leading trivia +
/// raw text of each, in order). Used both by `File::render` and by `cond.rs`
/// to rebuild a folded conditional group's own `raw` text.
pub fn render_items(items: &[(Item, Trivia)]) -> String {
    let mut out = String::new();
    for (item, trivia) in items {
        for c in &trivia.leading {
            out.push_str(c.text());
        }
        out.push_str(&item.raw);
    }
    out
}
