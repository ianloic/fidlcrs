use super::test_library::TestLibrary;
use crate::raw_ast::*;
use crate::tree_visitor::TreeVisitor;
use std::collections::BTreeSet;
use test_case::test_case;

use crate::source_span::SourceSpan;
use crate::tree_visitor;
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
    fn check_span(&mut self, element_type: ElementType, span: &SourceSpan<'a>) {
        if element_type == self.test_case_type {
            let mut s = span.data.trim_end();
            while s.ends_with(';') {
                s = s[..s.len() - 1].trim_end();
            }
            self.spans.insert(s.to_string());
        }
    }
}

impl<'a> TreeVisitor<'a> for SourceSpanVisitor<'a> {
    fn visit_aliasdeclaration(&mut self, node: &AliasDeclaration<'a>) {
        self.check_span(ElementType::AliasDeclaration, &node.element.span());
        tree_visitor::walk_aliasdeclaration(self, node);
    }
    fn visit_attribute(&mut self, node: &Attribute<'a>) {
        self.check_span(ElementType::Attribute, &node.element.span());
        tree_visitor::walk_attribute(self, node);
    }
    fn visit_attributearg(&mut self, node: &AttributeArg<'a>) {
        self.check_span(ElementType::AttributeArg, &node.element.span());
        tree_visitor::walk_attributearg(self, node);
    }
    fn visit_attributelist(&mut self, node: &AttributeList<'a>) {
        self.check_span(ElementType::AttributeList, &node.element.span());
        tree_visitor::walk_attributelist(self, node);
    }
    fn visit_binaryoperatorconstant(&mut self, node: &BinaryOperatorConstant<'a>) {
        self.check_span(ElementType::BinaryOperatorConstant, &node.element.span());
        tree_visitor::walk_binaryoperatorconstant(self, node);
    }
    fn visit_literal(&mut self, node: &Literal<'a>) {
        match node.kind {
            LiteralKind::Bool(_) => self.check_span(ElementType::BoolLiteral, &node.element.span()),
            LiteralKind::DocComment => {
                self.check_span(ElementType::DocCommentLiteral, &node.element.span())
            }
            LiteralKind::Numeric => {
                self.check_span(ElementType::NumericLiteral, &node.element.span())
            }
            LiteralKind::String => {
                self.check_span(ElementType::StringLiteral, &node.element.span())
            }
        }
        tree_visitor::walk_literal(self, node);
    }
    fn visit_compoundidentifier(&mut self, node: &CompoundIdentifier<'a>) {
        self.check_span(ElementType::CompoundIdentifier, &node.element.span());
        tree_visitor::walk_compoundidentifier(self, node);
    }
    fn visit_constdeclaration(&mut self, node: &ConstDeclaration<'a>) {
        self.check_span(ElementType::ConstDeclaration, &node.element.span());
        tree_visitor::walk_constdeclaration(self, node);
    }
    fn visit_file(&mut self, node: &File<'a>) {
        self.check_span(ElementType::File, &node.element.span());
        tree_visitor::walk_file(self, node);
    }
    fn visit_identifier(&mut self, node: &Identifier<'a>) {
        self.check_span(ElementType::Identifier, &node.element.span());
        tree_visitor::walk_identifier(self, node);
    }
    fn visit_identifierconstant(&mut self, node: &IdentifierConstant<'a>) {
        self.check_span(ElementType::IdentifierConstant, &node.element.span());
        tree_visitor::walk_identifierconstant(self, node);
    }
    fn visit_layoutparameter(&mut self, node: &LayoutParameter<'a>) {
        match node {
            LayoutParameter::Identifier(x) => {
                self.check_span(ElementType::IdentifierLayoutParameter, &x.element.span())
            }
            LayoutParameter::Literal(x) => {
                self.check_span(ElementType::LiteralLayoutParameter, &x.element.span())
            }
            LayoutParameter::Type(x) => {
                self.check_span(ElementType::TypeLayoutParameter, &x.element.span())
            }
            LayoutParameter::Inline(_) => {}
        }
        tree_visitor::walk_layoutparameter(self, node);
    }
    fn visit_librarydeclaration(&mut self, node: &LibraryDeclaration<'a>) {
        self.check_span(ElementType::LibraryDeclaration, &node.element.span());
        tree_visitor::walk_librarydeclaration(self, node);
    }
    fn visit_literalconstant(&mut self, node: &LiteralConstant<'a>) {
        self.check_span(ElementType::LiteralConstant, &node.element.span());
        tree_visitor::walk_literalconstant(self, node);
    }
    fn visit_modifier(&mut self, node: &Modifier<'a>) {
        self.check_span(ElementType::Modifier, &node.element.span());
        tree_visitor::walk_modifier(self, node);
    }

    fn visit_protocolcompose(&mut self, node: &ProtocolCompose<'a>) {
        self.check_span(ElementType::ProtocolCompose, &node.element.span());
        tree_visitor::walk_protocolcompose(self, node);
    }
    fn visit_protocoldeclaration(&mut self, node: &ProtocolDeclaration<'a>) {
        self.check_span(ElementType::ProtocolDeclaration, &node.element.span());
        tree_visitor::walk_protocoldeclaration(self, node);
    }
    fn visit_protocolmethod(&mut self, node: &ProtocolMethod<'a>) {
        self.check_span(ElementType::ProtocolMethod, &node.element.span());
        tree_visitor::walk_protocolmethod(self, node);
    }
    fn visit_resourcedeclaration(&mut self, node: &ResourceDeclaration<'a>) {
        self.check_span(ElementType::ResourceDeclaration, &node.element.span());
        tree_visitor::walk_resourcedeclaration(self, node);
    }
    fn visit_resourceproperty(&mut self, node: &ResourceProperty<'a>) {
        self.check_span(ElementType::ResourceProperty, &node.element.span());
        tree_visitor::walk_resourceproperty(self, node);
    }
    fn visit_servicedeclaration(&mut self, node: &ServiceDeclaration<'a>) {
        self.check_span(ElementType::ServiceDeclaration, &node.element.span());
        tree_visitor::walk_servicedeclaration(self, node);
    }
    fn visit_servicemember(&mut self, node: &ServiceMember<'a>) {
        self.check_span(ElementType::ServiceMember, &node.element.span());
        tree_visitor::walk_servicemember(self, node);
    }
    fn visit_typedeclaration(&mut self, node: &TypeDeclaration<'a>) {
        self.check_span(ElementType::TypeDeclaration, &node.element.span());
        tree_visitor::walk_typedeclaration(self, node);
    }
    fn visit_typeconstructor(&mut self, node: &TypeConstructor<'a>) {
        self.check_span(ElementType::TypeConstructor, &node.element.span());
        tree_visitor::walk_typeconstructor(self, node);
    }
    fn visit_usingdeclaration(&mut self, node: &UsingDeclaration<'a>) {
        self.check_span(ElementType::UsingDeclaration, &node.element.span());
        tree_visitor::walk_usingdeclaration(self, node);
    }
    fn visit_structdeclaration(&mut self, node: &StructDeclaration<'a>) {
        self.check_span(ElementType::StructLayout, &node.element.span());
        tree_visitor::walk_structdeclaration(self, node);
    }
    fn visit_structmember(&mut self, node: &StructMember<'a>) {
        self.check_span(ElementType::StructLayoutMember, &node.element.span());
        tree_visitor::walk_structmember(self, node);
    }
    fn visit_enumdeclaration(&mut self, node: &EnumDeclaration<'a>) {
        self.check_span(ElementType::ValueLayout, &node.element.span());
        tree_visitor::walk_enumdeclaration(self, node);
    }
    fn visit_bitsdeclaration(&mut self, node: &BitsDeclaration<'a>) {
        self.check_span(ElementType::ValueLayout, &node.element.span());
        tree_visitor::walk_bitsdeclaration(self, node);
    }
    fn visit_uniondeclaration(&mut self, node: &UnionDeclaration<'a>) {
        self.check_span(ElementType::OrdinaledLayout, &node.element.span());
        tree_visitor::walk_uniondeclaration(self, node);
    }
    fn visit_tabledeclaration(&mut self, node: &TableDeclaration<'a>) {
        self.check_span(ElementType::OrdinaledLayout, &node.element.span());
        tree_visitor::walk_tabledeclaration(self, node);
    }
    fn visit_unionmember(&mut self, node: &UnionMember<'a>) {
        self.check_span(ElementType::OrdinaledLayoutMember, &node.element.span());
        tree_visitor::walk_unionmember(self, node);
    }
    fn visit_tablemember(&mut self, node: &TableMember<'a>) {
        self.check_span(ElementType::OrdinaledLayoutMember, &node.element.span());
        tree_visitor::walk_tablemember(self, node);
    }
    fn visit_enummember(&mut self, node: &EnumMember<'a>) {
        self.check_span(ElementType::ValueLayoutMember, &node.element.span());
        tree_visitor::walk_enummember(self, node);
    }
    fn visit_bitsmember(&mut self, node: &BitsMember<'a>) {
        self.check_span(ElementType::ValueLayoutMember, &node.element.span());
        tree_visitor::walk_bitsmember(self, node);
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

fn check_spans(
    element_type: ElementType,
    pad_left: &str,
    pad_right: &str,
    exclude: &[ElementType],
    sources: &[&str],
) {
    if exclude.contains(&element_type) {
        return;
    }

    let mut library = TestLibrary::new();
    let mut expected_spans = BTreeSet::new();

    for (i, source) in sources.iter().enumerate() {
        let marked_source = source
            .replace("«", &format!("{}«", pad_left))
            .replace("»", &format!("»{}", pad_right));
        let (clean, spans) = extract_spans(&marked_source);
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

    let diff: Vec<_> = expected_spans
        .symmetric_difference(&visitor.spans)
        .collect();
    if !diff.is_empty() {
        panic!(
            "Spans mismatch for type {:?}\nExpected: {:#?}\nActual: {:#?}\nDiff: {:#?}",
            element_type, expected_spans, visitor.spans, diff
        );
    }
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_alias_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::AliasDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_attribute(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::Attribute,
        pad_left,
        pad_right,
        exclude,
        &[
            r#"library x; «@foo("foo")» «@bar» const MY_BOOL bool = false;"#,
            r#"library x;
          «@foo("foo")»
          «@bar»
          const MY_BOOL bool = false;
         "#,
            r#"library x;
          protocol Foo {
            Bar(«@foo» struct {});
          };
         "#,
        ],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_attribute_arg(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::AttributeArg,
        pad_left,
        pad_right,
        exclude,
        &[
            r#"library x; @attr(«"foo"») const MY_BOOL bool = false;"#,
            r#"library x; @attr(«a="foo"»,«b="bar"») const MY_BOOL bool = false;"#,
            r#"library x;
          const MY_BOOL bool = false;
          @attr(«a=true»,«b=MY_BOOL»,«c="foo"»)
          const MY_OTHER_BOOL bool = false;
         "#,
        ],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_attribute_list(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::AttributeList,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_binary_operator_constant(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::BinaryOperatorConstant,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_bool_literal(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::BoolLiteral,
        pad_left,
        pad_right,
        exclude,
        &[
            r#"library x; const x bool = «true»;"#,
            r#"library x; @attr(«true») const x bool = «true»;"#,
            r#"library x; const x bool = «false»;"#,
            r#"library x; @attr(«false») const x bool = «false»;"#,
        ],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_compound_identifier(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::CompoundIdentifier,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_const_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ConstDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_doc_comment_literal(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::DocCommentLiteral,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_identifier(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::Identifier,
        pad_left,
        pad_right,
        exclude,
        &[
            r#"library «x»;
          type «MyEnum» = strict enum {
            «A» = 1;
          };
         "#,
            r#"library «x»;
          type «MyStruct» = resource struct {
            «boolval» «bool»;
            «boolval» «resource»;
            «boolval» «flexible»;
            «boolval» «struct»;
          };
         "#,
            r#"library «x»;
          type «MyUnion» = flexible union {
            1: «intval» «int64»;
          };
         "#,
        ],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_identifier_constant(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::IdentifierConstant,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_identifier_layout_parameter(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::IdentifierLayoutParameter,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_inline_layout_reference(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::InlineLayoutReference,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_library_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::LibraryDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_literal_constant(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::LiteralConstant,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_literal_layout_parameter(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::LiteralLayoutParameter,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_modifier(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::Modifier,
        pad_left,
        pad_right,
        exclude,
        &[
            r#"library x; type MyBits = «flexible» bits { MY_VALUE = 1; };"#,
            r#"library x; type MyBits = «strict» bits : uint32 { MY_VALUE = 1; };"#,
            r#"library x; type MyEnum = «flexible» enum : uint32 { MY_VALUE = 1; };"#,
            r#"library x; type MyEnum = «strict» enum { MY_VALUE = 1; };"#,
            r#"library x; type MyStruct = «resource» struct {};"#,
            r#"library x; type MyTable = «resource» table { 1: my_member bool; };"#,
            r#"library x; type MyUnion = «resource» union { 1: my_member bool; };"#,
            r#"library x; type MyUnion = «flexible» union { 1: my_member bool; };"#,
            r#"library x; type MyUnion = «strict» union { 1: my_member bool; };"#,
            r#"library x; type MyUnion = «resource» «strict» union { 1: my_member bool; };"#,
            r#"library x; @attr type MyEnum = «flexible» enum : uint32 { MY_VALUE = 1; };"#,
            r#"library x; @attr type MyStruct = «resource» struct {};"#,
            r#"library x; @attr type MyUnion = «resource» «strict» union { 1: my_member bool; };"#,
            r#"library x; type MyUnion = «resource» «flexible» union { 1: my_member resource; };"#,
            r#"library x; type MyUnion = «strict» «resource» union { 1: my_member flexible; };"#,
            r#"library x; type MyUnion = «flexible» «resource» union { 1: my_member strict; };"#,
            r#"library x; «ajar» protocol MyProtocol {};"#,
            r#"library x; «closed» protocol MyProtocol {};"#,
            r#"library x; «open» protocol MyProtocol {};"#,
            r#"library x; @attr «open» protocol MyProtocol {};"#,
            r#"library x; «open» protocol MyProtocol { «flexible» MyMethod(); };"#,
            r#"library x; «open» protocol MyProtocol { «strict» MyMethod(); };"#,
            r#"library x; «open» protocol MyProtocol { @attr «strict» MyMethod(); };"#,
            r#"library x; type MyUnion = «flexible(added=2)» union {};"#,
            r#"library x; type MyUnion = «strict(removed=2)» «flexible(added=2)» «resource(added=3)» union {};"#,
            r#"library x; «open(removed=2)» «ajar(added=2)» protocol MyProtocol { @attr «strict(added=2)» MyMethod(); };"#,
            r#"library x; «open» protocol MyProtocol { «flexible» flexible(); strict(); };"#,
            r#"library x; «open» protocol MyProtocol { «strict» strict(); flexible(); };"#,
            r#"library x; «open» protocol MyProtocol { @attr «flexible» flexible(); @attr strict(); };"#,
        ],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_named_layout_reference(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::NamedLayoutReference,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_numeric_literal(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::NumericLiteral,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_ordinaled_layout(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::OrdinaledLayout,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_ordinaled_layout_member(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::OrdinaledLayoutMember,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_protocol_compose(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ProtocolCompose,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_protocol_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ProtocolDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_protocol_method(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ProtocolMethod,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_resource_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ResourceDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_resource_property(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ResourceProperty,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_service_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ServiceDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_service_member(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ServiceMember,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_string_literal(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::StringLiteral,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_struct_layout(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::StructLayout,
        pad_left,
        pad_right,
        exclude,
        &[r#"library x;
          type S = «resource struct {
            intval int64;
          }»;
         "#],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_struct_layout_member(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::StructLayoutMember,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_type_constructor(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::TypeConstructor,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_type_declaration(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::TypeDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_type_layout_parameter(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::TypeLayoutParameter,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_using(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::UsingDeclaration,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_value_layout(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ValueLayout,
        pad_left,
        pad_right,
        exclude,
        &[r#"library x;
          type B = «bits {
            A = 1;
          }»;
          type E = «strict enum {
            A = 1;
          }»;
         "#],
    );
}

#[test_case("", "", &[] ; "good_no_padding")]
#[test_case(" ", "", &[] ; "good_left_padding")]
#[test_case("", " ", &[ElementType::DocCommentLiteral] ; "good_right_padding")]
#[test_case(" ", " ", &[ElementType::DocCommentLiteral] ; "good_left_right_padding")]
fn test_span_value_layout_member(pad_left: &str, pad_right: &str, exclude: &[ElementType]) {
    check_spans(
        ElementType::ValueLayoutMember,
        pad_left,
        pad_right,
        exclude,
        &[],
    );
}
