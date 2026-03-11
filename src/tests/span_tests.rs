use super::test_library::TestLibrary;
use crate::tree_visitor::TreeVisitor;
use crate::raw_ast::*;
use std::collections::BTreeSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    AliasDeclaration,
    Attribute,
    AttributeArg,
    AttributeList,
    BinaryOperatorConstant,
    BoolLiteral,
    CompoundIdentifier,
    ConstDeclaration,
    DocCommentLiteral,
    File,
    Identifier,
    IdentifierConstant,
    IdentifierLayoutParameter,
    InlineLayoutReference,
    LayoutParameterList,
    LibraryDeclaration,
    LiteralConstant,
    LiteralLayoutParameter,
    Modifier,
    ModifierList,
    NamedLayoutReference,
    NumericLiteral,
    Ordinal64,
    OrdinaledLayout,
    OrdinaledLayoutMember,
    ParameterList,
    ProtocolCompose,
    ProtocolDeclaration,
    ProtocolMethod,
    ResourceDeclaration,
    ResourceProperty,
    ServiceMember,
    ServiceDeclaration,
    StringLiteral,
    StructLayout,
    StructLayoutMember,
    TypeConstraints,
    TypeDeclaration,
    TypeConstructor,
    TypeLayoutParameter,
    UsingDeclaration,
    ValueLayout,
    ValueLayoutMember,
}

struct SourceSpanVisitor<'a> {
    test_case_type: ElementType,
    spans: BTreeSet<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> SourceSpanVisitor<'a> {
    fn check_span(&mut self, element_type: ElementType, span: &crate::source_span::SourceSpan<'a>) {
        if element_type == self.test_case_type {
            self.spans.insert(span.data.to_string());
        }
    }
}

impl<'a> TreeVisitor<'a> for SourceSpanVisitor<'a> {
    fn visit_aliasdeclaration(&mut self, node: &AliasDeclaration<'a>) {
        self.check_span(ElementType::AliasDeclaration, &node.element.span());
        crate::tree_visitor::walk_aliasdeclaration(self, node);
    }
    fn visit_attribute(&mut self, node: &Attribute<'a>) {
        self.check_span(ElementType::Attribute, &node.element.span());
        crate::tree_visitor::walk_attribute(self, node);
    }
    fn visit_attributearg(&mut self, node: &AttributeArg<'a>) {
        self.check_span(ElementType::AttributeArg, &node.element.span());
        crate::tree_visitor::walk_attributearg(self, node);
    }
    fn visit_attributelist(&mut self, node: &AttributeList<'a>) {
        self.check_span(ElementType::AttributeList, &node.element.span());
        crate::tree_visitor::walk_attributelist(self, node);
    }
    fn visit_binaryoperatorconstant(&mut self, node: &BinaryOperatorConstant<'a>) {
        self.check_span(ElementType::BinaryOperatorConstant, &node.element.span());
        crate::tree_visitor::walk_binaryoperatorconstant(self, node);
    }
    fn visit_literal(&mut self, node: &Literal<'a>) {
        match node.kind {
            LiteralKind::Bool(_) => self.check_span(ElementType::BoolLiteral, &node.element.span()),
            LiteralKind::DocComment => self.check_span(ElementType::DocCommentLiteral, &node.element.span()),
            LiteralKind::Numeric => self.check_span(ElementType::NumericLiteral, &node.element.span()),
            LiteralKind::String => self.check_span(ElementType::StringLiteral, &node.element.span()),
        }
        crate::tree_visitor::walk_literal(self, node);
    }
    fn visit_compoundidentifier(&mut self, node: &CompoundIdentifier<'a>) {
        self.check_span(ElementType::CompoundIdentifier, &node.element.span());
        crate::tree_visitor::walk_compoundidentifier(self, node);
    }
    fn visit_constdeclaration(&mut self, node: &ConstDeclaration<'a>) {
        self.check_span(ElementType::ConstDeclaration, &node.element.span());
        crate::tree_visitor::walk_constdeclaration(self, node);
    }
    fn visit_file(&mut self, node: &File<'a>) {
        self.check_span(ElementType::File, &node.element.span());
        crate::tree_visitor::walk_file(self, node);
    }
    fn visit_identifier(&mut self, node: &Identifier<'a>) {
        self.check_span(ElementType::Identifier, &node.element.span());
        crate::tree_visitor::walk_identifier(self, node);
    }
    fn visit_identifierconstant(&mut self, node: &IdentifierConstant<'a>) {
        self.check_span(ElementType::IdentifierConstant, &node.element.span());
        crate::tree_visitor::walk_identifierconstant(self, node);
    }
    fn visit_layoutparameter(&mut self, node: &LayoutParameter<'a>) {
        match node {
            LayoutParameter::Identifier(x) => self.check_span(ElementType::IdentifierLayoutParameter, &x.element.span()),
            LayoutParameter::Literal(x) => self.check_span(ElementType::LiteralLayoutParameter, &x.element.span()),
            LayoutParameter::Type(x) => self.check_span(ElementType::TypeLayoutParameter, &x.element.span()),
            LayoutParameter::Inline(_) => {},
        }
        crate::tree_visitor::walk_layoutparameter(self, node);
    }
    fn visit_librarydeclaration(&mut self, node: &LibraryDeclaration<'a>) {
        self.check_span(ElementType::LibraryDeclaration, &node.element.span());
        crate::tree_visitor::walk_librarydeclaration(self, node);
    }
    fn visit_literalconstant(&mut self, node: &LiteralConstant<'a>) {
        self.check_span(ElementType::LiteralConstant, &node.element.span());
        crate::tree_visitor::walk_literalconstant(self, node);
    }
    fn visit_modifier(&mut self, node: &Modifier<'a>) {
        self.check_span(ElementType::Modifier, &node.element.span());
        crate::tree_visitor::walk_modifier(self, node);
    }

    fn visit_protocolcompose(&mut self, node: &ProtocolCompose<'a>) {
        self.check_span(ElementType::ProtocolCompose, &node.element.span());
        crate::tree_visitor::walk_protocolcompose(self, node);
    }
    fn visit_protocoldeclaration(&mut self, node: &ProtocolDeclaration<'a>) {
        self.check_span(ElementType::ProtocolDeclaration, &node.element.span());
        crate::tree_visitor::walk_protocoldeclaration(self, node);
    }
    fn visit_protocolmethod(&mut self, node: &ProtocolMethod<'a>) {
        self.check_span(ElementType::ProtocolMethod, &node.element.span());
        crate::tree_visitor::walk_protocolmethod(self, node);
    }
    fn visit_resourcedeclaration(&mut self, node: &ResourceDeclaration<'a>) {
        self.check_span(ElementType::ResourceDeclaration, &node.element.span());
        crate::tree_visitor::walk_resourcedeclaration(self, node);
    }
    fn visit_resourceproperty(&mut self, node: &ResourceProperty<'a>) {
        self.check_span(ElementType::ResourceProperty, &node.element.span());
        crate::tree_visitor::walk_resourceproperty(self, node);
    }
    fn visit_servicedeclaration(&mut self, node: &ServiceDeclaration<'a>) {
        self.check_span(ElementType::ServiceDeclaration, &node.element.span());
        crate::tree_visitor::walk_servicedeclaration(self, node);
    }
    fn visit_servicemember(&mut self, node: &ServiceMember<'a>) {
        self.check_span(ElementType::ServiceMember, &node.element.span());
        crate::tree_visitor::walk_servicemember(self, node);
    }
    fn visit_typedeclaration(&mut self, node: &TypeDeclaration<'a>) {
        self.check_span(ElementType::TypeDeclaration, &node.element.span());
        crate::tree_visitor::walk_typedeclaration(self, node);
    }
    fn visit_typeconstructor(&mut self, node: &TypeConstructor<'a>) {
        self.check_span(ElementType::TypeConstructor, &node.element.span());
        crate::tree_visitor::walk_typeconstructor(self, node);
    }
    fn visit_usingdeclaration(&mut self, node: &UsingDeclaration<'a>) {
        self.check_span(ElementType::UsingDeclaration, &node.element.span());
        crate::tree_visitor::walk_usingdeclaration(self, node);
    }
    fn visit_structdeclaration(&mut self, node: &StructDeclaration<'a>) {
        self.check_span(ElementType::StructLayout, &node.element.span());
        crate::tree_visitor::walk_structdeclaration(self, node);
    }
    fn visit_structmember(&mut self, node: &StructMember<'a>) {
        self.check_span(ElementType::StructLayoutMember, &node.element.span());
        crate::tree_visitor::walk_structmember(self, node);
    }
    fn visit_enumdeclaration(&mut self, node: &EnumDeclaration<'a>) {
        self.check_span(ElementType::ValueLayout, &node.element.span());
        crate::tree_visitor::walk_enumdeclaration(self, node);
    }
    fn visit_bitsdeclaration(&mut self, node: &BitsDeclaration<'a>) {
        self.check_span(ElementType::ValueLayout, &node.element.span());
        crate::tree_visitor::walk_bitsdeclaration(self, node);
    }
    fn visit_uniondeclaration(&mut self, node: &UnionDeclaration<'a>) {
        self.check_span(ElementType::OrdinaledLayout, &node.element.span());
        crate::tree_visitor::walk_uniondeclaration(self, node);
    }
    fn visit_tabledeclaration(&mut self, node: &TableDeclaration<'a>) {
        self.check_span(ElementType::OrdinaledLayout, &node.element.span());
        crate::tree_visitor::walk_tabledeclaration(self, node);
    }
    fn visit_unionmember(&mut self, node: &UnionMember<'a>) {
        self.check_span(ElementType::OrdinaledLayoutMember, &node.element.span());
        crate::tree_visitor::walk_unionmember(self, node);
    }
    fn visit_tablemember(&mut self, node: &TableMember<'a>) {
        self.check_span(ElementType::OrdinaledLayoutMember, &node.element.span());
        crate::tree_visitor::walk_tablemember(self, node);
    }
    fn visit_enummember(&mut self, node: &EnumMember<'a>) {
        self.check_span(ElementType::ValueLayoutMember, &node.element.span());
        crate::tree_visitor::walk_enummember(self, node);
    }
    fn visit_bitsmember(&mut self, node: &BitsMember<'a>) {
        self.check_span(ElementType::ValueLayoutMember, &node.element.span());
        crate::tree_visitor::walk_bitsmember(self, node);
    }
}

fn extract_spans(source: &str) -> (String, Vec<String>) {
    let mut clean = String::new();
    let mut spans = Vec::new();
    let mut stack = Vec::new();
    
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '«' {
            stack.push(clean.len());
        } else if chars[i] == '»' {
            if let Some(start) = stack.pop() {
                spans.push(clean.chars().skip(start).collect());
            } else {
                panic!("Unmatched closing guillemet");
            }
        } else {
            clean.push(chars[i]);
        }
        i += 1;
    }
    assert!(stack.is_empty(), "Unmatched opening guillemet");
    (clean, spans)
}

fn check_spans(element_type: ElementType, sources: &[&str]) {
    let mut library = TestLibrary::new();
    let mut expected_spans = BTreeSet::new();

    for (i, source) in sources.iter().enumerate() {
        let (clean, spans) = extract_spans(source);
        for span in spans {
            expected_spans.insert(span);
        }
        library.add_source_file(&format!("example{}.fidl", i), &clean);
    }
    
    let asts = match library.parse() {
        Ok(asts) => asts,
        Err(e) => panic!("Compilation failed: {}", e),
    };
    
    let mut visitor = SourceSpanVisitor {
        test_case_type: element_type,
        spans: BTreeSet::new(),
        _phantom: std::marker::PhantomData,
    };
    
    for ast in &asts {
        visitor.visit_file(ast);
    }
    
    let diff: Vec<_> = expected_spans.symmetric_difference(&visitor.spans).collect();
    if !diff.is_empty() {
        panic!("Spans mismatch for type {:?}\nExpected: {:#?}\nActual: {:#?}\nDiff: {:#?}", 
               element_type, expected_spans, visitor.spans, diff);
    }
}

#[test]
fn test_span_alias_declaration() {
    check_spans(ElementType::AliasDeclaration, &[
        
    ]);
}

#[test]
fn test_span_attribute() {
    check_spans(ElementType::Attribute, &[
        
    ]);
}

#[test]
fn test_span_attribute_arg() {
    check_spans(ElementType::AttributeArg, &[
        
    ]);
}

#[test]
fn test_span_attribute_list() {
    check_spans(ElementType::AttributeList, &[
        
    ]);
}

#[test]
fn test_span_binary_operator_constant() {
    check_spans(ElementType::BinaryOperatorConstant, &[
        
    ]);
}

#[test]
fn test_span_bool_literal() {
    check_spans(ElementType::BoolLiteral, &[
        
    ]);
}

#[test]
fn test_span_compound_identifier() {
    check_spans(ElementType::CompoundIdentifier, &[
        
    ]);
}

#[test]
fn test_span_const_declaration() {
    check_spans(ElementType::ConstDeclaration, &[
        
    ]);
}

#[test]
fn test_span_doc_comment_literal() {
    check_spans(ElementType::DocCommentLiteral, &[
        
    ]);
}

#[test]
fn test_span_identifier() {
    check_spans(ElementType::Identifier, &[
        
    ]);
}

#[test]
fn test_span_identifier_constant() {
    check_spans(ElementType::IdentifierConstant, &[
        
    ]);
}

#[test]
fn test_span_identifier_layout_parameter() {
    check_spans(ElementType::IdentifierLayoutParameter, &[
        
    ]);
}

#[test]
fn test_span_inline_layout_reference() {
    check_spans(ElementType::InlineLayoutReference, &[
        
    ]);
}

#[test]
fn test_span_library_declaration() {
    check_spans(ElementType::LibraryDeclaration, &[
        
    ]);
}

#[test]
fn test_span_literal_constant() {
    check_spans(ElementType::LiteralConstant, &[
        
    ]);
}

#[test]
fn test_span_literal_layout_parameter() {
    check_spans(ElementType::LiteralLayoutParameter, &[
        
    ]);
}

#[test]
fn test_span_modifier() {
    check_spans(ElementType::Modifier, &[
        
    ]);
}

#[test]
fn test_span_named_layout_reference() {
    check_spans(ElementType::NamedLayoutReference, &[
        
    ]);
}

#[test]
fn test_span_numeric_literal() {
    check_spans(ElementType::NumericLiteral, &[
        
    ]);
}

#[test]
fn test_span_ordinaled_layout() {
    check_spans(ElementType::OrdinaledLayout, &[
        
    ]);
}

#[test]
fn test_span_ordinaled_layout_member() {
    check_spans(ElementType::OrdinaledLayoutMember, &[
        
    ]);
}

#[test]
fn test_span_protocol_compose() {
    check_spans(ElementType::ProtocolCompose, &[
        
    ]);
}

#[test]
fn test_span_protocol_declaration() {
    check_spans(ElementType::ProtocolDeclaration, &[
        
    ]);
}

#[test]
fn test_span_protocol_method() {
    check_spans(ElementType::ProtocolMethod, &[
        
    ]);
}

#[test]
fn test_span_resource_declaration() {
    check_spans(ElementType::ResourceDeclaration, &[
        
    ]);
}

#[test]
fn test_span_resource_property() {
    check_spans(ElementType::ResourceProperty, &[
        
    ]);
}

#[test]
fn test_span_service_declaration() {
    check_spans(ElementType::ServiceDeclaration, &[
        
    ]);
}

#[test]
fn test_span_service_member() {
    check_spans(ElementType::ServiceMember, &[
        
    ]);
}

#[test]
fn test_span_string_literal() {
    check_spans(ElementType::StringLiteral, &[
        
    ]);
}

#[test]
fn test_span_struct_layout() {
    check_spans(ElementType::StructLayout, &[
        
    ]);
}

#[test]
fn test_span_struct_layout_member() {
    check_spans(ElementType::StructLayoutMember, &[
        
    ]);
}

#[test]
fn test_span_type_constructor() {
    check_spans(ElementType::TypeConstructor, &[
        
    ]);
}

#[test]
fn test_span_type_declaration() {
    check_spans(ElementType::TypeDeclaration, &[
        
    ]);
}

#[test]
fn test_span_type_layout_parameter() {
    check_spans(ElementType::TypeLayoutParameter, &[
        
    ]);
}

#[test]
fn test_span_using() {
    check_spans(ElementType::UsingDeclaration, &[
        
    ]);
}

#[test]
fn test_span_value_layout() {
    check_spans(ElementType::ValueLayout, &[
        
    ]);
}

#[test]
fn test_span_value_layout_member() {
    check_spans(ElementType::ValueLayoutMember, &[
        
    ]);
}
