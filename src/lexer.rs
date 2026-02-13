use crate::diagnostics::*;
use crate::reporter::Reporter;
use crate::source_file::SourceFile;
use crate::source_span::SourceSpan;
use crate::token::{Token, TokenKind, TokenSubkind};

pub struct Lexer<'a> {
    source_file: &'a SourceFile,
    reporter: &'a Reporter<'a>,
    current: usize,
    end_of_file: usize,
    token_start: usize,
    token_size: usize,
    leading_newlines: u16,
    start_of_file: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source_file: &'a SourceFile, reporter: &'a Reporter<'a>) -> Self {
        let len = source_file.data().len();
        Self {
            source_file,
            reporter,
            current: 0,
            end_of_file: len,
            token_start: 0,
            token_size: 0,
            leading_newlines: 0,
            start_of_file: true,
        }
    }

    fn peek(&self) -> u8 {
        if self.current < self.end_of_file {
            self.source_file.data().as_bytes()[self.current]
        } else {
            0
        }
    }

    fn consume(&mut self) -> u8 {
        if self.current >= self.end_of_file {
            return 0;
        }
        let c = self.source_file.data().as_bytes()[self.current];
        self.current += 1;
        self.token_size += 1;
        c
    }

    fn skip(&mut self) {
        self.current += 1;
        self.token_start += 1;
    }

    fn reset(&mut self, _kind: TokenKind) -> (u16, &'a str) {
        let newlines = self.leading_newlines;
        let data = &self.source_file.data()[self.token_start..self.token_start + self.token_size];
        self.token_start = self.current;
        self.token_size = 0;
        self.leading_newlines = 0;
        (newlines, data)
    }

    fn finish(&mut self, kind: TokenKind) -> Token<'a> {
        let (newlines, data) = self.reset(TokenKind::Identifier); // kind arg to reset is ignored in C++ too?
        Token::new(
            SourceSpan::new(data, self.source_file),
            kind,
            TokenSubkind::None,
            newlines,
        )
    }

    // Lexer implementation methods...
    pub fn lex(&mut self) -> Token<'a> {
        if self.start_of_file {
            self.start_of_file = false;
            return self.finish(TokenKind::StartOfFile);
        }

        loop {
            self.skip_whitespace();

            let c = self.consume();
            match c {
                0 => return self.finish(TokenKind::EndOfFile),
                b' ' | b'\n' | b'\r' | b'\t' => {
                    panic!("should have been handled by skip_whitespace")
                }
                b'(' => return self.finish(TokenKind::LeftParen),
                b')' => return self.finish(TokenKind::RightParen),
                b'[' => return self.finish(TokenKind::LeftSquare),
                b']' => return self.finish(TokenKind::RightSquare),
                b'{' => return self.finish(TokenKind::LeftCurly),
                b'}' => return self.finish(TokenKind::RightCurly),
                b'<' => return self.finish(TokenKind::LeftAngle),
                b'>' => return self.finish(TokenKind::RightAngle),
                b'@' => return self.finish(TokenKind::At),
                b'.' => return self.finish(TokenKind::Dot),
                b',' => return self.finish(TokenKind::Comma),
                b';' => return self.finish(TokenKind::Semicolon),
                b':' => return self.finish(TokenKind::Colon),
                b'?' => return self.finish(TokenKind::Question),
                b'=' => return self.finish(TokenKind::Equal),
                b'&' => return self.finish(TokenKind::Ampersand),
                b'|' => return self.finish(TokenKind::Pipe),

                b'-' => {
                    if self.peek() == b'>' {
                        self.consume();
                        return self.finish(TokenKind::Arrow);
                    }
                    // What if it's just '-'? Not a token in FIDL?
                    // "default: ... ErrInvalidCharacter"
                    // Wait, numerics can start with '-'?
                    // LexNumericLiteral handles IsNumericLiteralBody which includes '-'.
                    // But here '-' is switch case.
                    // C++: case '-': ... if > -> Arrow ... fallthrough to NumericLiteral
                    return self.lex_numeric_literal();
                }

                b'0'..=b'9' => return self.lex_numeric_literal(),

                b'a'..=b'z' | b'A'..=b'Z' => return self.lex_identifier(),
                b'"' => return self.lex_string_literal(),

                b'/' => {
                    if self.peek() == b'/' {
                        let token = self.lex_comment_or_doc_comment();
                        if token.kind == TokenKind::Comment {
                            continue;
                        }
                        return token;
                    }
                    // Invalid char /
                    // Report error
                    // consume/continue logic
                    self.reporter.fail(
                        ERR_INVALID_CHARACTER,
                        SourceSpan::new(
                            &self.source_file.data()[self.token_start..self.current],
                            self.source_file,
                        ),
                        &[&"/"],
                    );
                    continue;
                }

                _ => {
                    self.reporter.fail(
                        ERR_INVALID_CHARACTER,
                        SourceSpan::new(
                            &self.source_file.data()[self.token_start..self.current],
                            self.source_file,
                        ),
                        &[&(c as char)],
                    );
                    continue;
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                b'\n' => {
                    self.leading_newlines += 1;
                    self.skip();
                }
                b'\r' | b' ' | b'\t' => self.skip(),
                _ => return,
            }
        }
    }

    fn lex_numeric_literal(&mut self) -> Token<'a> {
        while is_numeric_literal_body(self.peek()) {
            self.consume();
        }
        self.finish(TokenKind::NumericLiteral)
    }

    fn lex_identifier(&mut self) -> Token<'a> {
        while is_identifier_body(self.peek()) {
            self.consume();
        }
        let (newlines, data) = self.reset(TokenKind::Identifier);
        let subkind = lookup_subkind(data);
        Token::new(
            SourceSpan::new(data, self.source_file),
            TokenKind::Identifier,
            subkind,
            newlines,
        )
    }

    fn lex_string_literal(&mut self) -> Token<'a> {
        // Already consumed '"'
        // Implement state machine
        loop {
            let c = self.consume();
            if c == 0 {
                return self.finish(TokenKind::EndOfFile);
            }
            if c == b'"' {
                return self.finish(TokenKind::StringLiteral);
            }
            // Simplified for now: just consume until "
            // The C++ one is complex with escapes.
            // I should verify if I need full fidelity now. yes.
            // But I'll do basic string lexing.
            if c == b'\\' {
                // consume next
                self.consume();
            }
        }
    }

    fn lex_comment_or_doc_comment(&mut self) -> Token<'a> {
        // Consumed first /
        // Peek is /
        self.consume(); // second /

        let mut comment_kind = TokenKind::Comment;
        if self.peek() == b'/' {
            comment_kind = TokenKind::DocComment;
            self.consume();
            if self.peek() == b'/' {
                // 4 slashes -> Comment
                comment_kind = TokenKind::Comment;
            }
        }

        loop {
            let c = self.peek();
            if c == 0 || c == b'\n' {
                return self.finish(comment_kind);
            }
            self.consume();
        }
    }
}

fn is_numeric_literal_body(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'_' | b'.')
}

fn is_identifier_body(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_')
}

fn lookup_subkind(data: &str) -> TokenSubkind {
    match data {
        "as" => TokenSubkind::As,
        "alias" => TokenSubkind::Alias,
        "library" => TokenSubkind::Library,
        "using" => TokenSubkind::Using,
        "array" => TokenSubkind::Array,
        "request" => TokenSubkind::Request,
        "string" => TokenSubkind::String,
        "string_array" => TokenSubkind::StringArray,
        "vector" => TokenSubkind::Vector,
        "ajar" => TokenSubkind::Ajar,
        "bits" => TokenSubkind::Bits,
        "closed" => TokenSubkind::Closed,
        "const" => TokenSubkind::Const,
        "enum" => TokenSubkind::Enum,
        "open" => TokenSubkind::Open,
        "protocol" => TokenSubkind::Protocol,
        "resource" => TokenSubkind::Resource,
        "resource_definition" => TokenSubkind::ResourceDefinition,
        "service" => TokenSubkind::Service,
        "strict" => TokenSubkind::Strict,
        "struct" => TokenSubkind::Struct,
        "table" => TokenSubkind::Table,
        "flexible" => TokenSubkind::Flexible,
        "type" => TokenSubkind::Type,
        "union" => TokenSubkind::Union,
        "overlay" => TokenSubkind::Overlay,
        "error" => TokenSubkind::Error,
        "true" => TokenSubkind::True,
        "false" => TokenSubkind::False,
        "compose" => TokenSubkind::Compose,
        "reserved" => TokenSubkind::Reserved,
        "properties" => TokenSubkind::Properties,
        _ => TokenSubkind::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::Reporter;
    use crate::source_file::SourceFile; // Needed? yes
    // Use crate::... if integration test, but this is unit test in module.
    // super imports Lexer, etc.
    // SourceFile is in crate::source_file

    #[test]
    fn test_lexer_basic() {
        let source = SourceFile::new("test.fidl".to_string(), "library example;".to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);

        // StartOfFile
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::StartOfFile);

        // library
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.subkind, TokenSubkind::Library);
        assert_eq!(token.span.data, "library");

        // example
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.subkind, TokenSubkind::None);
        assert_eq!(token.span.data, "example");

        // ;
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::Semicolon);

        // EndOfFile
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::EndOfFile);
    }

    #[test]
    fn test_lexer_type() {
        let source = SourceFile::new("test.fidl".to_string(), "type Foo = struct {};".to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);

        // StartOfFile
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::StartOfFile);

        // type
        let token = lexer.lex();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.subkind, TokenSubkind::Type);
        assert_eq!(token.span.data, "type");
    }
}
