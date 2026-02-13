use crate::lexer::Lexer;
use crate::raw_ast::*;
use crate::reporter::Reporter;
use crate::token::{Token, TokenKind, TokenSubkind};

#[allow(dead_code)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    reporter: &'a Reporter<'a>,
    last_token: Token<'a>,
    previous_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>, reporter: &'a Reporter<'a>) -> Self {
        let last_token = lexer.lex();
        Self {
            lexer,
            reporter,
            last_token,
            previous_token: None,
        }
    }

    pub fn parse_file(&mut self) -> Option<File<'a>> {
        let start_pos = self.last_token.clone();
        if self.last_token.kind == TokenKind::StartOfFile {
            self.last_token = self.lexer.lex();
        }

        let mut attributes = self.maybe_parse_attribute_list();

        let library_decl = if self.last_token.subkind == TokenSubkind::Library {
            self.parse_library_declaration(attributes.take())
        } else {
            None
        };

        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut struct_decls = vec![];
        let mut enum_decls = vec![];
        let mut bits_decls = vec![];
        let mut union_decls = vec![];
        let mut table_decls = vec![];
        let mut protocol_decls = vec![];

        loop {
            let mut attributes = self.maybe_parse_attribute_list();
            if self.last_token.kind == TokenKind::EndOfFile {
                break;
            }

            if self.last_token.subkind == TokenSubkind::Const {
                if let Some(decl) = self.parse_const_declaration(attributes.take()) {
                    const_decls.push(decl);
                } else {
                    // Error recovery: consume token if parsing failed
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Type {
                if let Some(decl) = self.parse_type_declaration(attributes.take()) {
                    type_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Struct {
                if let Some(decl) = self.parse_struct_declaration(attributes.take(), false) {
                    struct_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Enum {
                if let Some(decl) = self.parse_enum_declaration(attributes.take(), None) {
                    enum_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Bits {
                if let Some(decl) = self.parse_bits_declaration(attributes.take(), None) {
                    bits_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Union {
                // Default to Flexible if not specified by Strict/Flexible keywords
                if let Some(decl) =
                    self.parse_union_declaration(attributes.take(), Strictness::Flexible, false)
                {
                    union_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Table {
                if let Some(decl) = self.parse_table_declaration(attributes.take(), false) {
                    table_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Protocol
                || self.last_token.subkind == TokenSubkind::Closed
                || self.last_token.subkind == TokenSubkind::Open
                || self.last_token.subkind == TokenSubkind::Ajar
            {
                if let Some(decl) = self.parse_protocol_declaration(attributes.take()) {
                    protocol_decls.push(decl);
                } else {
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Resource {
                self.consume_token_with_subkind(TokenSubkind::Resource);
                if self.last_token.subkind == TokenSubkind::Struct {
                    if let Some(decl) = self.parse_struct_declaration(attributes.take(), true) {
                        struct_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else if self.last_token.subkind == TokenSubkind::Union {
                    let strictness = self.parse_strictness();
                    if let Some(decl) =
                        self.parse_union_declaration(attributes.take(), strictness, true)
                    {
                        union_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else if self.last_token.subkind == TokenSubkind::Table {
                    if let Some(decl) = self.parse_table_declaration(attributes.take(), true) {
                        table_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else {
                    // If 'resource' is followed by something unexpected
                    self.last_token = self.lexer.lex();
                }
            } else if self.last_token.subkind == TokenSubkind::Strict
                || self.last_token.subkind == TokenSubkind::Flexible
            {
                let strictness = self.parse_strictness();
                let is_resource = if self.last_token.subkind == TokenSubkind::Resource {
                    self.consume_token_with_subkind(TokenSubkind::Resource);
                    true
                } else {
                    false
                };

                if self.last_token.subkind == TokenSubkind::Union {
                    if let Some(decl) =
                        self.parse_union_declaration(attributes.take(), strictness, is_resource)
                    {
                        union_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else if self.last_token.subkind == TokenSubkind::Enum {
                    if let Some(decl) =
                        self.parse_enum_declaration(attributes.take(), Some(strictness))
                    {
                        enum_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else if self.last_token.subkind == TokenSubkind::Bits {
                    if let Some(decl) =
                        self.parse_bits_declaration(attributes.take(), Some(strictness))
                    {
                        bits_decls.push(decl);
                    } else {
                        self.last_token = self.lexer.lex();
                    }
                } else {
                    // If 'strict'/'flexible' is followed by something unexpected
                    self.last_token = self.lexer.lex();
                }
            } else {
                // If attributes were parsed but not consumed by a declaration,
                // or if an unexpected token is encountered.
                if attributes.is_some() {
                    // TODO: Report error for dangling attributes
                }
                self.last_token = self.lexer.lex();
            }
        }

        let end_pos = self.last_token.clone();
        Some(File {
            element: SourceElement::new(start_pos, end_pos),
            library_decl: library_decl.map(Box::new),
            const_decls,
            type_decls,
            struct_decls,
            enum_decls,
            bits_decls,
            union_decls,
            table_decls,
            protocol_decls,
            tokens: vec![], // Assuming token list is not needed internally
        })
    }

    pub fn parse_library_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<LibraryDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        self.consume_token_with_subkind(TokenSubkind::Library)?;
        let path = self.parse_compound_identifier()?;
        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(LibraryDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            path,
        })
    }

    pub fn parse_compound_identifier(&mut self) -> Option<CompoundIdentifier<'a>> {
        let start = self.last_token.clone();
        let mut components = vec![];

        components.push(self.parse_identifier()?);

        while self.last_token.kind == TokenKind::Dot {
            self.consume_token(TokenKind::Dot)?;
            components.push(self.parse_identifier()?);
        }

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(CompoundIdentifier {
            element: SourceElement::new(start, end),
            components,
        })
    }

    pub fn maybe_parse_attribute_list(&mut self) -> Option<AttributeList<'a>> {
        // Doc comments
        // At attributes
        // For now return None
        if self.last_token.kind == TokenKind::At || self.last_token.kind == TokenKind::DocComment {
            self.parse_attribute_list()
        } else {
            None
        }
    }

    pub fn parse_attribute_list(&mut self) -> Option<AttributeList<'a>> {
        let start = self.last_token.clone();
        let mut attributes = vec![];

        loop {
            if self.last_token.kind == TokenKind::DocComment {
                let token = self.consume_token(TokenKind::DocComment)?;
                // Doc comments are attributes with provenance DocComment
                let name = Identifier {
                    element: SourceElement::new(token.clone(), token.clone()), // data() will be "/// comment".
                };

                attributes.push(Attribute {
                    element: SourceElement::new(token.clone(), token.clone()),
                    name,
                    args: vec![AttributeArg {
                        element: SourceElement::new(token.clone(), token.clone()),
                        name: None,
                        value: Constant::Literal(LiteralConstant {
                            element: SourceElement::new(token.clone(), token.clone()),
                            literal: Literal {
                                element: SourceElement::new(token.clone(), token.clone()),
                                kind: LiteralKind::DocComment,
                                value: token.span.data.to_string(),
                            },
                        }),
                    }],
                    provenance: AttributeProvenance::DocComment,
                });
            } else if self.last_token.kind == TokenKind::At {
                let start_attr = self.last_token.clone();
                self.consume_token(TokenKind::At)?;
                let name = self.parse_identifier()?;
                let mut args = vec![];

                if self.last_token.kind == TokenKind::LeftParen {
                    self.consume_token(TokenKind::LeftParen)?;
                    // check for closing paren immediately
                    while self.last_token.kind != TokenKind::RightParen
                        && self.last_token.kind != TokenKind::EndOfFile
                    {
                        // Parse arg
                        let start_arg = self.last_token.clone();
                        let first = self.parse_constant()?;

                        let arg = if self.last_token.kind == TokenKind::Equal {
                            // The first constant was actually a name identifier
                            if let Constant::Identifier(id_const) = first {
                                // Convert IdentifierConstant to Identifier (for name)
                                // We take the first component of compound identifier
                                let id = id_const.identifier.components.into_iter().next().unwrap();
                                self.consume_token(TokenKind::Equal)?;
                                let val = self.parse_constant()?;
                                AttributeArg {
                                    element: SourceElement::new(
                                        start_arg.clone(),
                                        self.previous_token.as_ref().unwrap().clone(),
                                    ),
                                    name: Some(id),
                                    value: val,
                                }
                            } else {
                                return None;
                            }
                        } else {
                            // Positional arg
                            AttributeArg {
                                element: SourceElement::new(
                                    start_arg.clone(),
                                    self.previous_token.as_ref().unwrap().clone(),
                                ),
                                name: None,
                                value: first,
                            }
                        };
                        args.push(arg);

                        if self.last_token.kind == TokenKind::Comma {
                            self.consume_token(TokenKind::Comma)?;
                        } else {
                            break;
                        }
                    }
                    self.consume_token(TokenKind::RightParen)?;
                }

                let end_attr = self.previous_token.as_ref().unwrap().clone();
                attributes.push(Attribute {
                    element: SourceElement::new(start_attr, end_attr),
                    name,
                    args,
                    provenance: AttributeProvenance::Default,
                });
            } else {
                break;
            }
        }

        if attributes.is_empty() {
            return None;
        }

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(AttributeList {
            element: SourceElement::new(start, end),
            attributes,
        })
    }

    pub fn parse_const_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<ConstDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        self.consume_token_with_subkind(TokenSubkind::Const)?;
        let name = self.parse_identifier()?;
        let type_ctor = self.parse_type_constructor()?;
        self.consume_token(TokenKind::Equal)?;
        let value = self.parse_constant()?;
        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(ConstDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            type_ctor,
            value,
        })
    }

    pub fn parse_struct_member(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<StructMember<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        let name = self.parse_identifier()?;
        let type_ctor = self.parse_type_constructor()?;

        let default_value = if self.last_token.kind == TokenKind::Equal {
            self.consume_token(TokenKind::Equal)?;
            Some(self.parse_constant()?)
        } else {
            None
        };

        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(StructMember {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            type_ctor,
            name,
            default_value,
        })
    }

    pub fn parse_union_member(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<UnionMember<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        let ordinal = if self.last_token.kind == TokenKind::NumericLiteral {
            Some(self.parse_literal()?)
        } else {
            None
        };

        if ordinal.is_some() {
            self.consume_token(TokenKind::Colon)?;
        }

        if self.last_token.subkind == TokenSubkind::Reserved {
            self.consume_token_with_subkind(TokenSubkind::Reserved)?;
            self.consume_token(TokenKind::Semicolon)?;
            let end = self.previous_token.as_ref().unwrap().clone();
            return Some(UnionMember {
                element: SourceElement::new(start, end),
                attributes: attributes.map(Box::new),
                ordinal,
                name: None,
                type_ctor: None,
            });
        }

        let name = self.parse_identifier()?;
        let type_ctor = self.parse_type_constructor()?;
        self.consume_token(TokenKind::Semicolon)?;

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(UnionMember {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            ordinal,
            name: Some(name),
            type_ctor: Some(type_ctor),
        })
    }

    pub fn parse_table_member(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<TableMember<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        let ordinal = self.parse_literal()?;
        self.consume_token(TokenKind::Colon)?;

        if self.last_token.subkind == TokenSubkind::Reserved {
            self.consume_token_with_subkind(TokenSubkind::Reserved)?;
            self.consume_token(TokenKind::Semicolon)?;
            let end = self.previous_token.as_ref().unwrap().clone();
            return Some(TableMember {
                element: SourceElement::new(start, end),
                attributes: attributes.map(Box::new),
                ordinal,
                name: None,
                type_ctor: None,
            });
        }

        let name = self.parse_identifier()?;
        let type_ctor = self.parse_type_constructor()?;
        self.consume_token(TokenKind::Semicolon)?;

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(TableMember {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            ordinal,
            name: Some(name),
            type_ctor: Some(type_ctor),
        })
    }

    pub fn parse_type_constructor(&mut self) -> Option<TypeConstructor<'a>> {
        let start = self.last_token.clone();
        let layout = self.parse_layout_parameter()?;
        let mut parameters = vec![];
        let mut constraints = vec![];

        if self.last_token.kind == TokenKind::LeftAngle {
            self.consume_token(TokenKind::LeftAngle)?;
            // Parse params
            while self.last_token.kind != TokenKind::RightAngle
                && self.last_token.kind != TokenKind::EndOfFile
            {
                // Try parsing type param
                // If fails, try parsing constraint
                // Only implementing simple case for now
                if let Some(param) = self.parse_type_constructor() {
                    parameters.push(param);
                } else {
                    break;
                }
                if self.last_token.kind == TokenKind::Comma {
                    self.consume_token(TokenKind::Comma)?;
                } else {
                    break;
                }
            }
            self.consume_token(TokenKind::RightAngle)?;
        }

        if self.last_token.kind == TokenKind::Colon {
            self.consume_token(TokenKind::Colon)?;
            if self.last_token.kind == TokenKind::LeftAngle {
                self.consume_token(TokenKind::LeftAngle)?;
                loop {
                    if let Some(constant) = self.parse_constant() {
                        constraints.push(constant);
                    }
                    if self.last_token.kind == TokenKind::Comma {
                        self.consume_token(TokenKind::Comma)?;
                    } else {
                        break;
                    }
                }
                self.consume_token(TokenKind::RightAngle)?;
            } else {
                if let Some(constant) = self.parse_constant() {
                    constraints.push(constant);
                }
            }
        }

        let nullable = if self.last_token.kind == TokenKind::Question {
            self.consume_token(TokenKind::Question)?;
            true
        } else {
            false
        };

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(TypeConstructor {
            element: SourceElement::new(start, end),
            layout,
            parameters,
            constraints,
            nullable,
        })
    }

    pub fn parse_layout_parameter(&mut self) -> Option<LayoutParameter<'a>> {
        // Identifier or Literal or Type?
        // Check for identifier
        if self.last_token.kind == TokenKind::Identifier {
            // Could be compound identifier
            // But layout parameter is usually Type or Id.
            // Let's assume compound identifier for now
            let id = self.parse_compound_identifier()?;
            return Some(LayoutParameter::Identifier(id));
        } else if self.last_token.kind == TokenKind::NumericLiteral
            || self.last_token.kind == TokenKind::StringLiteral
            || self.last_token.subkind == TokenSubkind::True
            || self.last_token.subkind == TokenSubkind::False
        {
            let literal = self.parse_literal()?;
            return Some(LayoutParameter::Literal(LiteralConstant {
                element: literal.element.clone(),
                literal,
            }));
        }
        None
    }

    pub fn parse_constant(&mut self) -> Option<Constant<'a>> {
        let start = self.last_token.clone();
        if self.last_token.kind == TokenKind::Identifier {
            // Check if it is True/False subkind, treating as literal?
            // Actually, if lexer tokenizes as Identifier with Subkind::True,
            // we should consume it HERE or in parse_literal?
            if self.last_token.subkind == TokenSubkind::True
                || self.last_token.subkind == TokenSubkind::False
            {
                let literal = self.parse_literal()?;
                Some(Constant::Literal(LiteralConstant {
                    element: literal.element.clone(),
                    literal,
                }))
            } else {
                let id = self.parse_compound_identifier()?;
                Some(Constant::Identifier(IdentifierConstant {
                    element: SourceElement::new(
                        start,
                        self.previous_token.as_ref().unwrap().clone(),
                    ),
                    identifier: id,
                }))
            }
        } else if self.last_token.kind == TokenKind::StringLiteral
            || self.last_token.kind == TokenKind::NumericLiteral
        {
            let literal = self.parse_literal()?;
            Some(Constant::Literal(LiteralConstant {
                element: literal.element.clone(),
                literal,
            }))
        } else {
            None
        }
    }

    pub fn parse_literal(&mut self) -> Option<Literal<'a>> {
        let start = self.last_token.clone();
        let kind = match self.last_token.kind {
            TokenKind::StringLiteral => LiteralKind::String,
            TokenKind::NumericLiteral => LiteralKind::Numeric,
            _ => {
                if self.last_token.subkind == TokenSubkind::True {
                    LiteralKind::Bool(true)
                } else if self.last_token.subkind == TokenSubkind::False {
                    LiteralKind::Bool(false)
                } else {
                    return None;
                }
            }
        };

        let token = if kind == LiteralKind::Bool(true) || kind == LiteralKind::Bool(false) {
            // Consume identifier or specific subkind
            // If kind is not specific, consume_token(TokenKind::Identifier)?
            // Or consume_token_with_subkind?
            // But consume_token_with_subkind takes specific subkind.
            if self.last_token.subkind == TokenSubkind::True {
                self.consume_token_with_subkind(TokenSubkind::True)?
            } else {
                self.consume_token_with_subkind(TokenSubkind::False)?
            }
        } else {
            self.consume_token(self.last_token.kind)?
        };

        let end = token.clone();

        Some(Literal {
            element: SourceElement::new(start, end),
            kind,
            value: token.span.data.to_string(),
        })
    }

    pub fn parse_struct_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
        is_resource: bool,
    ) -> Option<StructDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        self.consume_token_with_subkind(TokenSubkind::Struct)?;

        let name = if self.last_token.kind == TokenKind::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let mut members = vec![];
        self.consume_token(TokenKind::LeftCurly)?;

        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let member_attrs = self.maybe_parse_attribute_list();
            if let Some(member) = self.parse_struct_member(member_attrs) {
                members.push(member);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;

        // Semicolon is NOT consumed here to allow reuse in TypeDeclaration.

        let end = self.previous_token.as_ref().unwrap().clone();

        Some(StructDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            is_resource,
            name,
            members,
        })
    }

    pub fn parse_enum_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
        strictness: Option<Strictness>,
    ) -> Option<EnumDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        self.consume_token_with_subkind(TokenSubkind::Enum)?;

        let name = if self.last_token.kind == TokenKind::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let subtype = if self.last_token.kind == TokenKind::Colon {
            self.consume_token(TokenKind::Colon)?;
            Some(self.parse_type_constructor()?)
        } else {
            None
        };

        let mut members = vec![];
        self.consume_token(TokenKind::LeftCurly)?;
        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let member_attrs = self.maybe_parse_attribute_list();
            if let Some(member) = self.parse_enum_member(member_attrs) {
                members.push(member);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;

        // Semicolon is NOT consumed here.

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(EnumDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            subtype,
            strictness,
            members,
        })
    }

    pub fn parse_enum_member(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<EnumMember<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        let name = self.parse_identifier()?;
        self.consume_token(TokenKind::Equal)?;
        let value = self.parse_constant()?;
        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(EnumMember {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            value,
        })
    }

    pub fn parse_bits_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
        strictness: Option<Strictness>,
    ) -> Option<BitsDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        self.consume_token_with_subkind(TokenSubkind::Bits)?;

        let name = if self.last_token.kind == TokenKind::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let subtype = if self.last_token.kind == TokenKind::Colon {
            self.consume_token(TokenKind::Colon)?;
            Some(self.parse_type_constructor()?)
        } else {
            None
        };

        let mut members = vec![];
        self.consume_token(TokenKind::LeftCurly)?;
        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let member_attrs = self.maybe_parse_attribute_list();
            if let Some(member) = self.parse_bits_member(member_attrs) {
                members.push(member);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;

        // Semicolon is NOT consumed here.

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(BitsDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            subtype,
            strictness,
            members,
        })
    }

    pub fn parse_bits_member(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<BitsMember<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        let name = self.parse_identifier()?;
        self.consume_token(TokenKind::Equal)?;
        let value = self.parse_constant()?;
        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(BitsMember {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            value,
        })
    }

    pub fn parse_union_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
        strictness: Strictness,
        is_resource: bool,
    ) -> Option<UnionDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        self.consume_token_with_subkind(TokenSubkind::Union)?;

        let name = if self.last_token.kind == TokenKind::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let mut members = vec![];
        self.consume_token(TokenKind::LeftCurly)?;
        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let member_attrs = self.maybe_parse_attribute_list();
            if let Some(member) = self.parse_union_member(member_attrs) {
                members.push(member);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;

        // Semicolon is NOT consumed here.

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(UnionDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            strictness,
            is_resource,
            members,
        })
    }

    pub fn parse_table_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
        is_resource: bool,
    ) -> Option<TableDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        self.consume_token_with_subkind(TokenSubkind::Table)?;

        let name = if self.last_token.kind == TokenKind::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let mut members = vec![];
        self.consume_token(TokenKind::LeftCurly)?;
        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let member_attrs = self.maybe_parse_attribute_list();
            if let Some(member) = self.parse_table_member(member_attrs) {
                members.push(member);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;

        // Semicolon is NOT consumed here.

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(TableDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            is_resource,
            members,
        })
    }

    pub fn parse_protocol_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<ProtocolDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        let mut _openness = None;
        if self.last_token.subkind == TokenSubkind::Closed
            || self.last_token.subkind == TokenSubkind::Open
            || self.last_token.subkind == TokenSubkind::Ajar
        {
            _openness = Some(self.last_token.clone());
            self.consume_token(TokenKind::Identifier)?;
        }

        self.consume_token_with_subkind(TokenSubkind::Protocol)?;
        let name = self.parse_identifier()?;

        let mut methods = vec![];
        self.consume_token(TokenKind::LeftCurly)?;

        while self.last_token.kind != TokenKind::RightCurly
            && self.last_token.kind != TokenKind::EndOfFile
        {
            let attrs = self.maybe_parse_attribute_list();
            if self.last_token.subkind == TokenSubkind::Compose {
                // Compose is mostly ignored for now by our AST simplification
                self.consume_token_with_subkind(TokenSubkind::Compose)?;
                self.parse_compound_identifier()?;
                self.consume_token(TokenKind::Semicolon)?;
            } else if let Some(method) = self.parse_protocol_method(attrs) {
                methods.push(method);
            } else {
                break;
            }
        }
        self.consume_token(TokenKind::RightCurly)?;
        self.consume_token(TokenKind::Semicolon)?;

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(ProtocolDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            methods,
        })
    }

    pub fn parse_protocol_method(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<ProtocolMethod<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());

        let mut _strictness = None;
        if self.last_token.subkind == TokenSubkind::Strict
            || self.last_token.subkind == TokenSubkind::Flexible
        {
            _strictness = Some(self.last_token.clone());
            self.consume_token(TokenKind::Identifier)?;
        }

        let is_event = if self.last_token.kind == TokenKind::Arrow {
            self.consume_token(TokenKind::Arrow)?;
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;

        let mut has_request = false;
        let mut request_payload = None;

        if !is_event {
            has_request = true;
            // Expect parameter list for requests
            self.consume_token(TokenKind::LeftParen)?;
            if self.last_token.kind != TokenKind::RightParen {
                // Actually FIDL methods specify parameters inside paren: Method(payload PayloadType); or Method(struct { ... });
                // For simplicity in JSON IR, if there's an identifier, treat parameter as payload type.
                if self.last_token.subkind == TokenSubkind::Struct {
                    if let Some(s) = self.parse_struct_declaration(None, false) {
                        request_payload = Some(Layout::Struct(s));
                    }
                } else if self.last_token.subkind == TokenSubkind::Table {
                    if let Some(t) = self.parse_table_declaration(None, false) {
                        request_payload = Some(Layout::Table(t));
                    }
                } else if self.last_token.subkind == TokenSubkind::Union {
                    if let Some(u) = self.parse_union_declaration(None, Strictness::Flexible, false)
                    {
                        request_payload = Some(Layout::Union(u));
                    }
                } else {
                    if let Some(tc) = self.parse_type_constructor() {
                        request_payload = Some(Layout::TypeConstructor(tc));
                    }
                }

                while self.last_token.kind != TokenKind::RightParen
                    && self.last_token.kind != TokenKind::EndOfFile
                {
                    self.consume_token(self.last_token.kind.clone())?; // Skip others
                }
            }
            self.consume_token(TokenKind::RightParen)?;
        }

        let mut has_response = false;
        let mut response_payload = None;
        let mut has_error = false;
        let mut error_payload = None;

        if is_event || self.last_token.kind == TokenKind::Arrow {
            if !is_event {
                self.consume_token(TokenKind::Arrow)?;
            }
            has_response = true;

            self.consume_token(TokenKind::LeftParen)?;
            if self.last_token.kind != TokenKind::RightParen {
                if self.last_token.subkind == TokenSubkind::Struct {
                    if let Some(s) = self.parse_struct_declaration(None, false) {
                        response_payload = Some(Layout::Struct(s));
                    }
                } else if self.last_token.subkind == TokenSubkind::Table {
                    if let Some(t) = self.parse_table_declaration(None, false) {
                        response_payload = Some(Layout::Table(t));
                    }
                } else if self.last_token.subkind == TokenSubkind::Union {
                    if let Some(u) = self.parse_union_declaration(None, Strictness::Flexible, false)
                    {
                        response_payload = Some(Layout::Union(u));
                    }
                } else {
                    if let Some(tc) = self.parse_type_constructor() {
                        response_payload = Some(Layout::TypeConstructor(tc));
                    }
                }

                while self.last_token.kind != TokenKind::RightParen
                    && self.last_token.kind != TokenKind::EndOfFile
                {
                    self.consume_token(self.last_token.kind.clone())?; // Skip others
                }
            }
            self.consume_token(TokenKind::RightParen)?;

            if self.last_token.subkind == TokenSubkind::Error {
                self.consume_token_with_subkind(TokenSubkind::Error)?;
                has_error = true;
                if let Some(tc) = self.parse_type_constructor() {
                    error_payload = Some(Layout::TypeConstructor(tc));
                }
            }
        }

        self.consume_token(TokenKind::Semicolon)?;

        let end = self.previous_token.as_ref().unwrap().clone();
        Some(ProtocolMethod {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            has_request,
            request_payload,
            has_response,
            response_payload,
            has_error,
            error_payload,
        })
    }

    fn parse_strictness(&mut self) -> Strictness {
        if self.last_token.subkind == TokenSubkind::Strict {
            self.consume_token_with_subkind(TokenSubkind::Strict);
            Strictness::Strict
        } else if self.last_token.subkind == TokenSubkind::Flexible {
            self.consume_token_with_subkind(TokenSubkind::Flexible);
            Strictness::Flexible
        } else {
            Strictness::Flexible
        }
    }

    pub fn parse_type_declaration(
        &mut self,
        attributes: Option<AttributeList<'a>>,
    ) -> Option<TypeDeclaration<'a>> {
        let start = attributes
            .as_ref()
            .map(|a| a.element.start_token.clone())
            .unwrap_or_else(|| self.last_token.clone());
        self.consume_token_with_subkind(TokenSubkind::Type)?;
        let name = self.parse_identifier()?;
        self.consume_token(TokenKind::Equal)?;
        // Parse modifiers
        let mut strictness = None;
        if self.last_token.subkind == TokenSubkind::Strict {
            self.consume_token_with_subkind(TokenSubkind::Strict);
            strictness = Some(Strictness::Strict);
        } else if self.last_token.subkind == TokenSubkind::Flexible {
            self.consume_token_with_subkind(TokenSubkind::Flexible);
            strictness = Some(Strictness::Flexible);
        }

        let is_resource = if self.last_token.subkind == TokenSubkind::Resource {
            self.consume_token_with_subkind(TokenSubkind::Resource);
            true
        } else {
            false
        };

        let layout = if self.last_token.subkind == TokenSubkind::Struct {
            Layout::Struct(self.parse_struct_declaration(None, is_resource)?)
        } else if self.last_token.subkind == TokenSubkind::Union {
            Layout::Union(self.parse_union_declaration(
                None,
                strictness.unwrap_or(Strictness::Strict),
                is_resource,
            )?)
        } else if self.last_token.subkind == TokenSubkind::Table {
            Layout::Table(self.parse_table_declaration(None, is_resource)?)
        } else if self.last_token.subkind == TokenSubkind::Enum {
            Layout::Enum(self.parse_enum_declaration(None, strictness)?)
        } else if self.last_token.subkind == TokenSubkind::Bits {
            Layout::Bits(self.parse_bits_declaration(None, strictness)?)
        } else {
            Layout::TypeConstructor(self.parse_type_constructor()?)
        };

        self.consume_token(TokenKind::Semicolon)?;
        let end = self.previous_token.as_ref().unwrap().clone();

        Some(TypeDeclaration {
            element: SourceElement::new(start, end),
            attributes: attributes.map(Box::new),
            name,
            layout,
        })
    }

    fn consume_token_with_subkind(&mut self, subkind: TokenSubkind) -> Option<Token<'a>> {
        if self.last_token.subkind == subkind {
            let token = self.last_token.clone();
            self.previous_token = Some(token.clone());
            self.last_token = self.lexer.lex();
            Some(token)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> &Token<'a> {
        &self.last_token
    }

    pub fn consume_token(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        if self.last_token.kind == kind {
            let token = self.last_token.clone();
            self.previous_token = Some(token.clone());
            self.last_token = self.lexer.lex();
            Some(token)
        } else {
            // TODO: Report error
            None
        }
    }

    #[allow(dead_code)]
    pub fn parse_identifier(&mut self) -> Option<Identifier<'a>> {
        let start = self.last_token.clone();
        let _token = self.consume_token(TokenKind::Identifier)?;
        let end = self.previous_token.as_ref()?.clone();

        Some(Identifier {
            element: SourceElement::new(start, end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::reporter::Reporter;
    use crate::source_file::SourceFile;

    #[test]
    fn test_parse_identifier() {
        let source = SourceFile::new("test.fidl".to_string(), "foobar".to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);
        parser
            .consume_token(TokenKind::StartOfFile)
            .expect("failed to consume StartOfFile");

        let id = parser
            .parse_identifier()
            .expect("failed to parse identifier");
        assert_eq!(id.data(), "foobar");
    }

    #[test]
    fn test_parse_library_decl() {
        let source = SourceFile::new("test.fidl".to_string(), "library foo.bar;".to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let lib = parser
            .parse_library_declaration(None)
            .expect("parse library");
        assert_eq!(lib.path.components.len(), 2);
        assert_eq!(lib.path.components[0].data(), "foo");
        assert_eq!(lib.path.components[1].data(), "bar");
    }

    #[test]
    fn test_parse_const_decl() {
        let source = SourceFile::new("test.fidl".to_string(), "const FOO uint8 = 5;".to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser.parse_const_declaration(None).expect("parse const");
        assert_eq!(decl.name.data(), "FOO");
    }

    #[test]
    fn test_parse_struct_decl() {
        let source = SourceFile::new(
            "test.fidl".to_string(),
            "struct Foo { bar uint8; };".to_string(),
        );
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser
            .parse_struct_declaration(None, false)
            .expect("parse struct");
        assert_eq!(decl.name.as_ref().unwrap().data(), "Foo");
        assert_eq!(decl.members.len(), 1);
        assert_eq!(decl.members[0].name.data(), "bar");
    }

    #[test]
    fn test_parse_file_full() {
        let content = r#"
library example;

const FOO uint8 = 5;

struct MyStruct {
    member uint8;
};
"#;
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        let file = parser.parse_file().expect("parse file");
        assert!(file.library_decl.is_some());
        assert_eq!(file.const_decls.len(), 1);
        assert_eq!(file.struct_decls.len(), 1);
        assert_eq!(file.const_decls[0].name.data(), "FOO");
        assert_eq!(
            file.struct_decls[0].name.as_ref().unwrap().data(),
            "MyStruct"
        );
    }

    #[test]
    fn test_parse_enum_decl() {
        let content = "enum Color : uint32 { RED = 1; BLUE = 2; };";
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser
            .parse_enum_declaration(None, None)
            .expect("Failed to parse enum");
        assert_eq!(decl.name.as_ref().unwrap().data(), "Color");
        assert!(decl.subtype.is_some());
        assert_eq!(decl.members.len(), 2);
        assert_eq!(decl.members[0].name.data(), "RED");
    }

    #[test]
    fn test_parse_bits_decl() {
        let content = "bits Flags : uint8 { A = 1; B = 2; };";
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser
            .parse_bits_declaration(None, None)
            .expect("parse bits");
        assert_eq!(decl.name.as_ref().unwrap().data(), "Flags");
        assert_eq!(decl.members.len(), 2);
    }

    #[test]
    fn test_parse_union_decl() {
        let content = "union MyUnion { 1: foo uint8; 2: bar string; };";
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser
            .parse_union_declaration(None, Strictness::Flexible, false)
            .expect("parse union");
        assert_eq!(decl.name.as_ref().unwrap().data(), "MyUnion");
        assert_eq!(decl.members.len(), 2);
        assert!(decl.members[0].ordinal.is_some());
    }

    #[test]
    fn test_parse_table_decl() {
        let content = "table MyTable { 1: foo uint8; 2: reserved; };";
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        let decl = parser
            .parse_table_declaration(None, false)
            .expect("parse table");
        assert_eq!(decl.name.as_ref().unwrap().data(), "MyTable");
        assert_eq!(decl.members.len(), 2);
        assert!(decl.members[1].name.is_none()); // Reserved
    }

    #[test]
    fn test_parse_type_decl() {
        let content = "type MyStruct = struct { member uint8; };";
        let source = SourceFile::new("test.fidl".to_string(), content.to_string());
        let reporter = Reporter::new();
        let mut lexer = Lexer::new(&source, &reporter);
        let mut parser = Parser::new(&mut lexer, &reporter);

        parser.consume_token(TokenKind::StartOfFile).unwrap();

        if parser.peek().kind == TokenKind::EndOfFile {
            panic!(
                "Parser got EndOfFile immediately! Source data: '{}'",
                source.data()
            );
        }

        let decl = parser.parse_type_declaration(None);
        if decl.is_none() {
            panic!("parse type failed. Current token: {:?}", parser.peek());
        }
        let decl = decl.unwrap();
        assert_eq!(decl.name.data(), "MyStruct");
        if let Layout::Struct(s) = decl.layout {
            assert!(s.name.is_none());
            assert_eq!(s.members.len(), 1);
        } else {
            panic!("expected struct layout");
        }
    }
}
