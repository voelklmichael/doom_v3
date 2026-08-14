//! Step 1a: a minimal hand-rolled scanner that partitions a whole source
//! file into `RawToken`s (comments, string/char literals, preprocessor
//! lines, and everything else as "code"). This has to happen *before* brace
//! counting because braces can legally appear inside block comments
//! (confirmed in r_draw.c, s_sound.c, v_video.c, i_sound.c) and inside
//! string/char literals - a naive brace counter would miscount on those
//! real files.
//!
//! Backslash-newline line splicing is handled transparently: `peek`/`bump`
//! skip over `\` followed by a newline as if it weren't there, so a
//! backslash-continued macro is scanned as one logical line. The spliced
//! bytes are never dropped - spans always slice the original source, so
//! concatenating every token's text reproduces the file exactly.

use super::ast::{Pos, RawToken, Span};

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: Pos,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a str) -> Self {
        Cursor { bytes: src.as_bytes(), pos: Pos::start() }
    }

    fn eof(&self) -> bool {
        self.pos.byte >= self.bytes.len()
    }

    fn raw_at(&self, byte: usize) -> Option<u8> {
        self.bytes.get(byte).copied()
    }

    /// Consume exactly one raw byte (no splice-skipping), updating pos.
    fn advance_raw(&mut self) -> u8 {
        let b = self.bytes[self.pos.byte];
        if b == b'\n' {
            self.pos.line += 1;
            self.pos.col = 1;
        } else {
            self.pos.col += 1;
        }
        self.pos.byte += 1;
        b
    }

    /// Skip any number of backslash-newline splices at the current position.
    fn skip_splices(&mut self) {
        loop {
            if self.raw_at(self.pos.byte) != Some(b'\\') {
                return;
            }
            let nl_len = match self.raw_at(self.pos.byte + 1) {
                Some(b'\n') => 1,
                Some(b'\r') if self.raw_at(self.pos.byte + 2) == Some(b'\n') => 2,
                _ => return,
            };
            self.advance_raw(); // backslash
            for _ in 0..nl_len {
                self.advance_raw();
            }
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_splices();
        self.raw_at(self.pos.byte)
    }

    /// Peek the logical byte after the current logical byte, without
    /// consuming either.
    fn peek2(&mut self) -> Option<u8> {
        self.skip_splices();
        let save = self.pos;
        if self.eof() {
            return None;
        }
        self.advance_raw();
        self.skip_splices();
        let r = self.raw_at(self.pos.byte);
        self.pos = save;
        r
    }

    fn bump(&mut self) -> Option<u8> {
        self.skip_splices();
        if self.eof() {
            return None;
        }
        Some(self.advance_raw())
    }
}

struct Scanner<'a> {
    src: &'a str,
    cur: Cursor<'a>,
    /// True if only horizontal whitespace has been seen since the last
    /// newline - used to detect a `#` that starts a preprocessor line.
    bol: bool,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a str) -> Self {
        Scanner { src, cur: Cursor::new(src), bol: true }
    }

    fn slice(&self, start: usize, end: usize) -> String {
        self.src[start..end].to_string()
    }

    fn pos(&self) -> Pos {
        self.cur.pos
    }

    fn peek(&mut self) -> Option<u8> {
        self.cur.peek()
    }

    fn peek2(&mut self) -> Option<u8> {
        self.cur.peek2()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.cur.bump();
        if let Some(b) = b {
            if b == b'\n' {
                self.bol = true;
            } else if b != b' ' && b != b'\t' && b != b'\r' {
                self.bol = false;
            }
        }
        b
    }

    fn bump_through_eol(&mut self) {
        while let Some(c) = self.peek() {
            if c == b'\n' {
                break;
            }
            self.bump();
        }
        if self.peek() == Some(b'\n') {
            self.bump();
        }
    }

    fn scan(mut self) -> Vec<RawToken> {
        let mut tokens = Vec::new();
        let mut code_start: Option<Pos> = None;

        macro_rules! flush_code {
            () => {
                if let Some(start) = code_start.take() {
                    let end = self.pos();
                    if end.byte > start.byte {
                        tokens.push(RawToken::Code(Span {
                            start,
                            end,
                            text: self.slice(start.byte, end.byte),
                        }));
                    }
                }
            };
        }

        while let Some(c) = self.peek() {
            if c == b'#' && self.bol {
                flush_code!();
                let start = self.pos();
                self.bump_through_eol();
                let end = self.pos();
                tokens.push(RawToken::PreprocLine(Span { start, end, text: self.slice(start.byte, end.byte) }));
                continue;
            }
            if c == b'/' && self.peek2() == Some(b'/') {
                flush_code!();
                let start = self.pos();
                self.bump_through_eol();
                let end = self.pos();
                tokens.push(RawToken::LineComment(Span { start, end, text: self.slice(start.byte, end.byte) }));
                continue;
            }
            if c == b'/' && self.peek2() == Some(b'*') {
                flush_code!();
                let start = self.pos();
                self.bump();
                self.bump();
                loop {
                    match self.peek() {
                        None => break,
                        Some(b'*') if self.peek2() == Some(b'/') => {
                            self.bump();
                            self.bump();
                            break;
                        }
                        Some(_) => {
                            self.bump();
                        }
                    }
                }
                let end = self.pos();
                tokens.push(RawToken::BlockComment(Span { start, end, text: self.slice(start.byte, end.byte) }));
                continue;
            }
            if c == b'"' {
                flush_code!();
                let start = self.pos();
                self.bump();
                loop {
                    match self.peek() {
                        None => break,
                        Some(b'\\') => {
                            self.bump();
                            self.bump();
                        }
                        Some(b'"') => {
                            self.bump();
                            break;
                        }
                        Some(b'\n') => break, // unterminated; bail defensively
                        Some(_) => {
                            self.bump();
                        }
                    }
                }
                let end = self.pos();
                tokens.push(RawToken::StringLit(Span { start, end, text: self.slice(start.byte, end.byte) }));
                continue;
            }
            if c == b'\'' {
                flush_code!();
                let start = self.pos();
                self.bump();
                loop {
                    match self.peek() {
                        None => break,
                        Some(b'\\') => {
                            self.bump();
                            self.bump();
                        }
                        Some(b'\'') => {
                            self.bump();
                            break;
                        }
                        Some(b'\n') => break,
                        Some(_) => {
                            self.bump();
                        }
                    }
                }
                let end = self.pos();
                tokens.push(RawToken::CharLit(Span { start, end, text: self.slice(start.byte, end.byte) }));
                continue;
            }
            // Otherwise: part of a Code run.
            if code_start.is_none() {
                code_start = Some(self.pos());
            }
            self.bump();
        }
        flush_code!();
        tokens
    }
}

pub fn scan(src: &str) -> Vec<RawToken> {
    Scanner::new(src).scan()
}

#[cfg(test)]
mod tests;
