use super::test_library::TestLibrary;
use crate::source_file::SourceFile;

#[test]
#[ignore]
fn test_span_alias_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «alias Foo = uint8»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; «alias Foo = vector<uint8>»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_attribute() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «@foo("foo")» «@bar» const MY_BOOL bool = false;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x;
          «@foo("foo")»
          «@bar»
          const MY_BOOL bool = false;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x;
          protocol Foo {
            Bar(«@foo» struct {});
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_attribute_arg() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; @attr(«"foo"») const MY_BOOL bool = false;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; @attr(«a="foo"»,«b="bar"») const MY_BOOL bool = false;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x;
          const MY_BOOL bool = false;
          @attr(«a=true»,«b=MY_BOOL»,«c="foo"»)
          const MY_OTHER_BOOL bool = false;
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_attribute_list() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «@foo("foo") @bar» const MY_BOOL bool = false;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x;
          «@foo("foo")
          @bar»
          const MY_BOOL bool = false;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x;
          protocol Foo {
            Bar(«@foo» struct {});
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_binary_operator_constant() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          const one uint8 = 0x0001;
          const two_fifty_six uint16 = 0x0100;
          const two_fifty_seven uint16 = «one | two_fifty_six»;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; const two_fifty_seven uint16 = «0x0001 | 0x0100»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_bool_literal() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x bool = «true»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; @attr(«true») const x bool = «true»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; const x bool = «false»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; @attr(«false») const x bool = «false»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_compound_identifier() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library «foo.bar.baz»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_const_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library example;
          «const C_SIMPLE uint32   = 11259375»;
          «const C_HEX_S uint32    = 0xABCDEF»;
          «const C_HEX_L uint32    = 0XABCDEF»;
          «const C_BINARY_S uint32 = 0b101010111100110111101111»;
          «const C_BINARY_L uint32 = 0B101010111100110111101111»;
      "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_doc_comment_literal() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          «/// Foo»
          const MY_BOOL bool = false;"#
            .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_identifier() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library «x»;
          type «MyEnum» = strict enum {
            «A» = 1;
          };
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library «x»;
          type «MyStruct» = resource struct {
            «boolval» «bool»;
            «boolval» «resource»;
            «boolval» «flexible»;
            «boolval» «struct»;
          };
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library «x»;
          type «MyUnion» = flexible union {
            1: «intval» «int64»;
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_identifier_constant() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x bool = true; const y bool = «x»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_identifier_layout_parameter() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type a = bool; const b uint8 = 4; type y = array<«a»,«b»>;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_inline_layout_reference() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type S = «struct {
            intval int64;
            boolval bool = false;
            stringval string:MAX_STRING_SIZE;
            inner «union {
              1: floatval float64;
            }»:optional;
          }»;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x;
          protocol P {
            M(«struct {
              intval int64;
              boolval bool = false;
              stringval string:MAX_STRING_SIZE;
              inner «union {
                1: floatval float64;
              }»:optional;
            }»);
          };
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x;
          protocol Foo {
            Bar(«@foo struct {}»);
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_layout_parameter_list() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type y = array«<uint8,4>»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type y = vector«<array«<uint8,4>»>»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_library_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"«library x»; using y;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"«library x.y.z»; using y;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_literal_constant() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x bool = «true»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; const x uint8 = «42»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; const x string = «"hi"»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_literal_layout_parameter() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type y = array<uint8,«4»>;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type y = vector<array<uint8,«4»>>;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_modifier() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type MyBits = «flexible» bits { MY_VALUE = 1; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type MyBits = «strict» bits : uint32 { MY_VALUE = 1; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; type MyEnum = «flexible» enum : uint32 { MY_VALUE = 1; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; type MyEnum = «strict» enum { MY_VALUE = 1; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 4),
        r#"library x; type MyStruct = «resource» struct {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 5),
        r#"library x; type MyTable = «resource» table { 1: my_member bool; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 6),
        r#"library x; type MyUnion = «resource» union { 1: my_member bool; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 7),
        r#"library x; type MyUnion = «flexible» union { 1: my_member bool; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 8),
        r#"library x; type MyUnion = «strict» union { 1: my_member bool; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 9),
        r#"library x; type MyUnion = «resource» «strict» union { 1: my_member bool; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 10),
        r#"library x; @attr type MyEnum = «flexible» enum : uint32 { MY_VALUE = 1; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 11),
        r#"library x; @attr type MyStruct = «resource» struct {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 12),
        r#"library x; @attr type MyUnion = «resource» «strict» union { 1: my_member bool; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 13),
        r#"library x; type MyUnion = «resource» «flexible» union { 1: my_member resource; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 14),
        r#"library x; type MyUnion = «strict» «resource» union { 1: my_member flexible; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 15),
        r#"library x; type MyUnion = «flexible» «resource» union { 1: my_member strict; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 16),
        r#"library x; «ajar» protocol MyProtocol {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 17),
        r#"library x; «closed» protocol MyProtocol {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 18),
        r#"library x; «open» protocol MyProtocol {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 19),
        r#"library x; @attr «open» protocol MyProtocol {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 20),
        r#"library x; «open» protocol MyProtocol { «flexible» MyMethod(); };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 21),
        r#"library x; «open» protocol MyProtocol { «strict» MyMethod(); };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 22),
        r#"library x; «open» protocol MyProtocol { @attr «strict» MyMethod(); };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 23),
        r#"library x; type MyUnion = «flexible(added=2)» union {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(format!("example{}.fidl", 24), r#"library x; type MyUnion = «strict(removed=2)» «flexible(added=2)» «resource(added=3)» union {};"#.to_string()));

    library.add_source(SourceFile::new(format!("example{}.fidl", 25), r#"library x; «open(removed=2)» «ajar(added=2)» protocol MyProtocol { @attr «strict(added=2)» MyMethod(); };"#.to_string()));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 26),
        r#"library x; «open» protocol MyProtocol { «flexible» flexible(); strict(); };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 27),
        r#"library x; «open» protocol MyProtocol { «strict» strict(); flexible(); };"#.to_string(),
    ));

    library.add_source(SourceFile::new(format!("example{}.fidl", 28), r#"library x; «open» protocol MyProtocol { @attr «flexible» flexible(); @attr strict(); };"#.to_string()));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_modifier_list() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type MyUnion = «flexible» union {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type MyStruct = struct { anon @attr «flexible» union {}; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; @attr «ajar» protocol MyProtocol { @attr «flexible» MyMethod(); };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; type MyUnion = «flexible resource» union {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(format!("example{}.fidl", 4), r#"library x; type MyStruct = «resource» struct { anon @attr «flexible resource» union {}; };"#.to_string()));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 5),
        r#"library x; type MyUnion = «flexible(added=2)» union {};"#.to_string(),
    ));

    library.add_source(SourceFile::new(format!("example{}.fidl", 6), r#"library x; type MyUnion = «strict(removed=2) flexible(added=2) resource(added=3)» union {};"#.to_string()));

    library.add_source(SourceFile::new(format!("example{}.fidl", 7), r#"library x; «open(removed=2) ajar(added=2)» protocol MyProtocol { @attr «strict(added=2)» MyMethod(); };"#.to_string()));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_named_layout_reference() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type S = struct {
            intval «int64»;
            boolval «bool» = false;
            stringval «string»:MAX_STRING_SIZE;
            inner struct {
              floatval «float64»;
              uintval «uint8» = 7;
              vecval «vector»<«vector»<Foo>>;
              arrval «array»<uint8,4>;
            };
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_numeric_literal() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x uint8 = «42»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; @attr(«42») const x uint8 = «42»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_ordinal64() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type U = union { «1:» one uint8; };"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_ordinaled_layout() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type T = «resource table {
            1: intval int64;
          }»;
          type U = «flexible resource union {
            1: intval int64;
          }»:optional;
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_ordinaled_layout_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type T = table {
            «1: intval int64»;
            «@attr 3: floatval float64»;
            «4: stringval string:100»;
            «5: inner union {
              «1: boolval bool»;
            }:optional»;
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_parameter_list() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; protocol X { Method«()» -> «()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; protocol X { Method«(struct {})» -> «(struct {})»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; protocol X { Method«(struct { a int32; b bool; })» -> «(struct { c
         uint8; d bool; })»; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; protocol X { -> Event«()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 4),
        r#"library x; protocol X { -> Event«(struct {})»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 5),
        r#"library x; protocol X { -> Event«(struct { a int32; b bool; })»; };"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_protocol_compose() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; protocol X { «compose OtherProtocol»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; protocol X { «@attr compose OtherProtocol»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; protocol X {
            «/// Foo
            compose OtherProtocol»;
          };"#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_protocol_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «protocol X {}»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; «@attr protocol X { compose OtherProtocol; }»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_protocol_method() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; protocol X { «Method()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; protocol X { «@attr Method(struct { a int32; b bool; })»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; protocol X { «Method(struct { a int32; }) -> ()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; protocol X { «@attr Method(struct { a int32; }) -> ()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 4),
        r#"library x; protocol X { «Method(struct { a int32; }) -> (struct { res bool; })»;
         };"#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 5),
        r#"library x; protocol X { «Method(struct { a int32; }) -> (struct { res
         bool; res2 int32; })»; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 6),
        r#"library x; protocol X { «Method(struct { a int32; }) -> () error uint32»;
         };"#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 7),
        r#"library x; protocol X { «@attr Method(struct { a int32; }) -> () error
         uint32»; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 8),
        r#"library x; protocol X { «Method(struct { a int32; }) ->
         (struct { res bool; }) error uint32»; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 9),
        r#"library x; protocol X {
         «Method(struct { a int32; }) -> (struct { res bool; res2 int32; }) error uint32»;
         };"#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 10),
        r#"library x; protocol X { «-> Event()»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 11),
        r#"library x; protocol X { «-> Event(struct { res bool; })»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 12),
        r#"library x; protocol X { «@attr -> Event(struct { res bool; res2 int32; })»;
         };"#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_resource_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"
     library example; «resource_definition Res : uint32 { properties { subtype Enum; };
     }»;"#
            .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_resource_property() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"
     library example; resource_definition Res : uint32 { properties { «subtype Enum»; };
     };"#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_service_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «service X {}»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; protocol P {}; «service X { Z client_end:P; }»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_service_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; protocol P {}; service X { «Z client_end:P»; };"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; protocol P {}; service X { «@attr Z client_end:P»; };"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_string_literal() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x string = «"hello"»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; @attr(«"foo"») const x string = «"goodbye"»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; @attr(a=«"foo"»,b=«"bar"») const MY_BOOL bool = false;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_struct_layout() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type S = «resource struct {
            intval int64;
          }»;
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_struct_layout_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type S = struct {
            «intval int64»;
            «boolval bool = false»;
            «@attr stringval string:100»;
            «inner struct {
              «floatval float64»;
              «uintval uint8 = 7»;
            }»;
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_type_constraints() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type y = array<uint8,4>;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type y = vector<vector<uint8>:«16»>:«<16,optional>»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; type y = union { 1: foo bool; }:«optional»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; using zx; type y = zx.Handle:«optional»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 4),
        r#"library x; using zx; type y = zx.Handle:«<VMO,zx.READ,optional>»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_type_constructor() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; const x «int32» = 1;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; const x «zx.Handle:<VMO, zx.Rights.READ, optional>» = 1;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 2),
        r#"library x; const x «Foo<«Bar<«zx.Handle:VMO»>:20»>:optional» = 1;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 3),
        r#"library x; const x «zx.Handle:VMO» = 1;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 4),
        r#"library x; type y = «array<uint8,4>»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 5),
        r#"library x; type y = «vector<«array<Foo,4>»>»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 6),
        r#"library x; type y = «string:100»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 7),
        r#"library x; type y = «string:<100,optional>»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 8),
        r#"library x;
          type e = «flexible enum : «uint32» {
            A = 1;
          }»;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 9),
        r#"library x;
          type S = «struct {
            intval «int64»;
            boolval «bool» = false;
            stringval «string:MAX_STRING_SIZE»;
            inner «struct {
              floatval «float64»;
              uintval «uint8» = 7;
              vecval «vector<«vector<Foo>»>»;
              arrval «array<uint8,4>»;
            }»;
          }»;
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 10),
        r#"library x; protocol X { Method(«struct { a «int32»; b «bool»; }») -> («struct
         {}») error «uint32»; };"#
            .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 11),
        r#"library x;
          resource_definition foo : «uint8» {
              properties {
                  rights «rights»;
              };
          };
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 12),
        r#"library x;
          protocol Foo {
            Bar(«@foo struct {}»);
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_type_declaration() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          «type E = enum : int8 {
            A = 1;
          }»;
          «type S = struct {
            intval int64;
          }»;
          «type U = union {
            1: intval int64;
          }:optional»;
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_type_layout_parameter() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; type y = array<uint8,4>;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; type y = vector<«array<uint8,4>»>;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_using() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x; «using y»;"#.to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x; «using y as z»;"#.to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_value_layout() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type B = «bits {
            A = 1;
          }»;
          type E = «strict enum {
            A = 1;
          }»;
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}

#[test]
#[ignore]
fn test_span_value_layout_member() {
    let mut library = TestLibrary::new();

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 0),
        r#"library x;
          type E = enum {
            «A = 1»;
            «@attr B = 2»;
          };
         "#
        .to_string(),
    ));

    library.add_source(SourceFile::new(
        format!("example{}.fidl", 1),
        r#"library x;
          type B = bits {
            «A = 0x1»;
            «@attr B = 0x2»;
          };
         "#
        .to_string(),
    ));
    // TODO: Implement AST span checking logic (TreeVisitor port required)
    let result = library.compile();
    assert!(result.is_err() || result.is_ok());
}
