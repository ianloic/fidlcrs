use crate::raw_ast::*;

pub trait TreeVisitor<'a> {
    fn visit_sourceelement(&mut self, node: &SourceElement<'a>) {
        walk_sourceelement(self, node);
    }
    fn visit_identifier(&mut self, node: &Identifier<'a>) {
        walk_identifier(self, node);
    }
    fn visit_compoundidentifier(&mut self, node: &CompoundIdentifier<'a>) {
        walk_compoundidentifier(self, node);
    }
    fn visit_librarydeclaration(&mut self, node: &LibraryDeclaration<'a>) {
        walk_librarydeclaration(self, node);
    }
    fn visit_attributelist(&mut self, node: &AttributeList<'a>) {
        walk_attributelist(self, node);
    }
    fn visit_attribute(&mut self, node: &Attribute<'a>) {
        walk_attribute(self, node);
    }
    fn visit_modifier(&mut self, node: &Modifier<'a>) {
        walk_modifier(self, node);
    }
    fn visit_attributearg(&mut self, node: &AttributeArg<'a>) {
        walk_attributearg(self, node);
    }
    fn visit_identifierconstant(&mut self, node: &IdentifierConstant<'a>) {
        walk_identifierconstant(self, node);
    }
    fn visit_literalconstant(&mut self, node: &LiteralConstant<'a>) {
        walk_literalconstant(self, node);
    }
    fn visit_binaryoperatorconstant(&mut self, node: &BinaryOperatorConstant<'a>) {
        walk_binaryoperatorconstant(self, node);
    }
    fn visit_literal(&mut self, node: &Literal<'a>) {
        walk_literal(self, node);
    }
    fn visit_constdeclaration(&mut self, node: &ConstDeclaration<'a>) {
        walk_constdeclaration(self, node);
    }
    fn visit_typeconstructor(&mut self, node: &TypeConstructor<'a>) {
        walk_typeconstructor(self, node);
    }
    fn visit_file(&mut self, node: &File<'a>) {
        walk_file(self, node);
    }
    fn visit_usingdeclaration(&mut self, node: &UsingDeclaration<'a>) {
        walk_usingdeclaration(self, node);
    }
    fn visit_resourceproperty(&mut self, node: &ResourceProperty<'a>) {
        walk_resourceproperty(self, node);
    }
    fn visit_resourcedeclaration(&mut self, node: &ResourceDeclaration<'a>) {
        walk_resourcedeclaration(self, node);
    }
    fn visit_typedeclaration(&mut self, node: &TypeDeclaration<'a>) {
        walk_typedeclaration(self, node);
    }
    fn visit_structdeclaration(&mut self, node: &StructDeclaration<'a>) {
        walk_structdeclaration(self, node);
    }
    fn visit_structmember(&mut self, node: &StructMember<'a>) {
        walk_structmember(self, node);
    }
    fn visit_enumdeclaration(&mut self, node: &EnumDeclaration<'a>) {
        walk_enumdeclaration(self, node);
    }
    fn visit_enummember(&mut self, node: &EnumMember<'a>) {
        walk_enummember(self, node);
    }
    fn visit_bitsdeclaration(&mut self, node: &BitsDeclaration<'a>) {
        walk_bitsdeclaration(self, node);
    }
    fn visit_bitsmember(&mut self, node: &BitsMember<'a>) {
        walk_bitsmember(self, node);
    }
    fn visit_uniondeclaration(&mut self, node: &UnionDeclaration<'a>) {
        walk_uniondeclaration(self, node);
    }
    fn visit_unionmember(&mut self, node: &UnionMember<'a>) {
        walk_unionmember(self, node);
    }
    fn visit_tabledeclaration(&mut self, node: &TableDeclaration<'a>) {
        walk_tabledeclaration(self, node);
    }
    fn visit_tablemember(&mut self, node: &TableMember<'a>) {
        walk_tablemember(self, node);
    }
    fn visit_protocoldeclaration(&mut self, node: &ProtocolDeclaration<'a>) {
        walk_protocoldeclaration(self, node);
    }
    fn visit_protocolcompose(&mut self, node: &ProtocolCompose<'a>) {
        walk_protocolcompose(self, node);
    }
    fn visit_protocolmethod(&mut self, node: &ProtocolMethod<'a>) {
        walk_protocolmethod(self, node);
    }
    fn visit_servicedeclaration(&mut self, node: &ServiceDeclaration<'a>) {
        walk_servicedeclaration(self, node);
    }
    fn visit_servicemember(&mut self, node: &ServiceMember<'a>) {
        walk_servicemember(self, node);
    }
    fn visit_aliasdeclaration(&mut self, node: &AliasDeclaration<'a>) {
        walk_aliasdeclaration(self, node);
    }
    fn visit_constant(&mut self, node: &Constant<'a>) {
        walk_constant(self, node);
    }
    fn visit_layoutparameter(&mut self, node: &LayoutParameter<'a>) {
        walk_layoutparameter(self, node);
    }
    fn visit_layout(&mut self, node: &Layout<'a>) {
        walk_layout(self, node);
    }
}

pub fn walk_sourceelement<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &SourceElement<'a>) {
}

pub fn walk_identifier<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Identifier<'a>) {
    visitor.visit_sourceelement(&node.element);
}

pub fn walk_compoundidentifier<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &CompoundIdentifier<'a>) {
    visitor.visit_sourceelement(&node.element);
    for item in &node.components {
        visitor.visit_identifier(item);
    }
}

pub fn walk_librarydeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &LibraryDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_compoundidentifier(&node.path);
}

pub fn walk_attributelist<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &AttributeList<'a>) {
    visitor.visit_sourceelement(&node.element);
    for item in &node.attributes {
        visitor.visit_attribute(item);
    }
}

pub fn walk_attribute<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Attribute<'a>) {
    visitor.visit_sourceelement(&node.element);
    visitor.visit_identifier(&node.name);
    for item in &node.args {
        visitor.visit_attributearg(item);
    }
}

pub fn walk_modifier<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Modifier<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(item);
    }
}

pub fn walk_attributearg<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &AttributeArg<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    visitor.visit_constant(&node.value);
}

pub fn walk_identifierconstant<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &IdentifierConstant<'a>) {
    visitor.visit_sourceelement(&node.element);
    visitor.visit_compoundidentifier(&node.identifier);
}

pub fn walk_literalconstant<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &LiteralConstant<'a>) {
    visitor.visit_sourceelement(&node.element);
    visitor.visit_literal(&node.literal);
}

pub fn walk_binaryoperatorconstant<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &BinaryOperatorConstant<'a>) {
    visitor.visit_sourceelement(&node.element);
    visitor.visit_constant(&*node.left);
    visitor.visit_constant(&*node.right);
}

pub fn walk_literal<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Literal<'a>) {
    visitor.visit_sourceelement(&node.element);
}

pub fn walk_constdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ConstDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    visitor.visit_typeconstructor(&node.type_ctor);
    visitor.visit_constant(&node.value);
}

pub fn walk_typeconstructor<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &TypeConstructor<'a>) {
    visitor.visit_sourceelement(&node.element);
    visitor.visit_layoutparameter(&node.layout);
    for item in &node.parameters {
        visitor.visit_typeconstructor(item);
    }
    for item in &node.constraints {
        visitor.visit_constant(item);
    }
}

pub fn walk_file<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &File<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.library_decl {
        visitor.visit_librarydeclaration(&**item);
    }
    for item in &node.const_decls {
        visitor.visit_constdeclaration(item);
    }
    for item in &node.alias_decls {
        visitor.visit_aliasdeclaration(item);
    }
    for item in &node.using_decls {
        visitor.visit_usingdeclaration(item);
    }
    for item in &node.type_decls {
        visitor.visit_typedeclaration(item);
    }
    for item in &node.struct_decls {
        visitor.visit_structdeclaration(item);
    }
    for item in &node.enum_decls {
        visitor.visit_enumdeclaration(item);
    }
    for item in &node.bits_decls {
        visitor.visit_bitsdeclaration(item);
    }
    for item in &node.union_decls {
        visitor.visit_uniondeclaration(item);
    }
    for item in &node.table_decls {
        visitor.visit_tabledeclaration(item);
    }
    for item in &node.protocol_decls {
        visitor.visit_protocoldeclaration(item);
    }
    for item in &node.service_decls {
        visitor.visit_servicedeclaration(item);
    }
    for item in &node.resource_decls {
        visitor.visit_resourcedeclaration(item);
    }
}

pub fn walk_usingdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &UsingDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_compoundidentifier(&node.using_path);
    if let Some(item) = &node.maybe_alias {
        visitor.visit_identifier(item);
    }
}

pub fn walk_resourceproperty<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ResourceProperty<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_typeconstructor(&node.type_ctor);
    visitor.visit_identifier(&node.name);
}

pub fn walk_resourcedeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ResourceDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    if let Some(item) = &node.type_ctor {
        visitor.visit_typeconstructor(item);
    }
    for item in &node.properties {
        visitor.visit_resourceproperty(item);
    }
}

pub fn walk_typedeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &TypeDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    visitor.visit_layout(&node.layout);
}

pub fn walk_structdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &StructDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    for item in &node.members {
        visitor.visit_structmember(item);
    }
}

pub fn walk_structmember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &StructMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_typeconstructor(&node.type_ctor);
    visitor.visit_identifier(&node.name);
    if let Some(item) = &node.default_value {
        visitor.visit_constant(item);
    }
}

pub fn walk_enumdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &EnumDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    if let Some(item) = &node.subtype {
        visitor.visit_typeconstructor(item);
    }
    for item in &node.members {
        visitor.visit_enummember(item);
    }
}

pub fn walk_enummember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &EnumMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    visitor.visit_constant(&node.value);
}

pub fn walk_bitsdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &BitsDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    if let Some(item) = &node.subtype {
        visitor.visit_typeconstructor(item);
    }
    for item in &node.members {
        visitor.visit_bitsmember(item);
    }
}

pub fn walk_bitsmember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &BitsMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    visitor.visit_constant(&node.value);
}

pub fn walk_uniondeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &UnionDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    for item in &node.members {
        visitor.visit_unionmember(item);
    }
}

pub fn walk_unionmember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &UnionMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    if let Some(item) = &node.ordinal {
        visitor.visit_literal(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    if let Some(item) = &node.type_ctor {
        visitor.visit_typeconstructor(item);
    }
    if let Some(item) = &node.default_value {
        visitor.visit_constant(item);
    }
}

pub fn walk_tabledeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &TableDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    for item in &node.members {
        visitor.visit_tablemember(item);
    }
}

pub fn walk_tablemember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &TableMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    if let Some(item) = &node.ordinal {
        visitor.visit_literal(item);
    }
    if let Some(item) = &node.name {
        visitor.visit_identifier(item);
    }
    if let Some(item) = &node.type_ctor {
        visitor.visit_typeconstructor(item);
    }
}

pub fn walk_protocoldeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ProtocolDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    visitor.visit_identifier(&node.name);
    for item in &node.composed_protocols {
        visitor.visit_protocolcompose(item);
    }
    for item in &node.methods {
        visitor.visit_protocolmethod(item);
    }
}

pub fn walk_protocolcompose<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ProtocolCompose<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_compoundidentifier(&node.protocol_name);
}

pub fn walk_protocolmethod<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ProtocolMethod<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    for item in &node.modifiers {
        visitor.visit_modifier(item);
    }
    visitor.visit_identifier(&node.name);
    if let Some(item) = &node.request_payload {
        visitor.visit_layout(item);
    }
    if let Some(item) = &node.response_payload {
        visitor.visit_layout(item);
    }
    if let Some(item) = &node.error_payload {
        visitor.visit_layout(item);
    }
    if let Some(item) = &node.response_param_element {
        visitor.visit_sourceelement(item);
    }
}

pub fn walk_servicedeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ServiceDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    for item in &node.members {
        visitor.visit_servicemember(item);
    }
}

pub fn walk_servicemember<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &ServiceMember<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_typeconstructor(&node.type_ctor);
    visitor.visit_identifier(&node.name);
}

pub fn walk_aliasdeclaration<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &AliasDeclaration<'a>) {
    visitor.visit_sourceelement(&node.element);
    if let Some(item) = &node.attributes {
        visitor.visit_attributelist(&**item);
    }
    visitor.visit_identifier(&node.name);
    visitor.visit_typeconstructor(&node.type_ctor);
}

pub fn walk_constant<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Constant<'a>) {
    match node {
        Constant::Identifier(item) => visitor.visit_identifierconstant(item),
        Constant::Literal(item) => visitor.visit_literalconstant(item),
        Constant::BinaryOperator(item) => visitor.visit_binaryoperatorconstant(item),
    }
}

pub fn walk_layoutparameter<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &LayoutParameter<'a>) {
    match node {
        LayoutParameter::Identifier(item) => visitor.visit_compoundidentifier(item),
        LayoutParameter::Literal(item) => visitor.visit_literalconstant(item),
        LayoutParameter::Type(item) => visitor.visit_typeconstructor(&**item),
        LayoutParameter::Inline(item) => visitor.visit_layout(&**item),
    }
}

pub fn walk_layout<'a, V: TreeVisitor<'a> + ?Sized>(visitor: &mut V, node: &Layout<'a>) {
    match node {
        Layout::Struct(item) => visitor.visit_structdeclaration(item),
        Layout::Enum(item) => visitor.visit_enumdeclaration(item),
        Layout::Bits(item) => visitor.visit_bitsdeclaration(item),
        Layout::Union(item) => visitor.visit_uniondeclaration(item),
        Layout::Table(item) => visitor.visit_tabledeclaration(item),
        Layout::TypeConstructor(item) => visitor.visit_typeconstructor(item),
    }
}
