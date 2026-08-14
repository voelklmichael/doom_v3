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
        Pos { line: 1, col: 1, byte: 0 }
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

#[derive(Debug, Clone)]
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

/// Comments attached to an item. `trailing` is unused by v1 (see record.rs
/// module docs) but kept for future same-line-comment attachment.
#[derive(Debug, Clone, Default)]
pub struct Trivia {
    pub leading: Vec<Comment>,
    pub trailing: Option<Comment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordKind {
    Struct,
    Union,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub ty: String,
    pub name: String,
    pub array_dims: Vec<Option<String>>,
    pub bitfield: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RecordDecl {
    pub kind: RecordKind,
    pub tag: Option<String>,
    pub fields: Vec<Field>,
    /// Declarator names following the closing brace, e.g. `} foo, *foo_p;`.
    pub names: Vec<String>,
    /// If this came from `typedef struct { ... } name;`, the typedef name.
    pub typedef_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub tag: Option<String>,
    pub variants: Vec<(String, Option<String>)>,
    pub names: Vec<String>,
    pub typedef_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypedefDecl {
    pub underlying: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Init {
    Braced(String),
    Expr(String),
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub storage: Vec<String>,
    pub ty: String,
    pub name: String,
    pub array_dims: Vec<Option<String>>,
    pub initializer: Option<Init>,
}

#[derive(Debug, Clone)]
pub struct FnSig {
    pub storage: Vec<String>,
    pub ret_ty: String,
    pub name: String,
    pub params_raw: String,
}

#[derive(Debug, Clone)]
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
    /// Fallback for anything v1 doesn't structurally recognize yet. `raw`
    /// (on the containing `Item`) already holds the exact text.
    Raw,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    /// Exact original text of this item, excluding its leading/trailing
    /// trivia. This is the round-trip source of truth - the structured
    /// fields in `kind` are best-effort derived data on top of it.
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub items: Vec<(Item, Trivia)>,
}

impl File {
    /// Reconstructs the exact original file text from the AST.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for (item, trivia) in &self.items {
            for c in &trivia.leading {
                out.push_str(c.text());
            }
            out.push_str(&item.raw);
            if let Some(c) = &trivia.trailing {
                out.push_str(c.text());
            }
        }
        out
    }
}
