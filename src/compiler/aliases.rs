use crate::compiler::Compiler;
use crate::flat_ast::AliasDeclaration;
use crate::raw_ast;

impl<'node, 'src> Compiler<'node, 'src> {
    pub fn compile_alias(
        &mut self,
        decl: &'node raw_ast::AliasDeclaration<'src>,
        library_name: &str,
    ) -> AliasDeclaration {
        AliasDeclaration::new(
            format!("{}/{}", library_name, decl.name.data()).into(),
            self.get_location(&decl.name.element),
            self.is_deprecated(decl.attributes.as_deref()),
            self.compile_attribute_list(&decl.attributes),
            self.compile_partial_type_ctor(&decl.type_ctor, library_name),
            self.resolve_type(&decl.type_ctor, library_name, None),
        )
    }
}
