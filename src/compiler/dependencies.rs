use super::RawDecl;
use crate::diagnostics::Error;
use crate::flat_ast;
use crate::flat_ast::*;
use crate::raw_ast;
use crate::reporter::Reporter;
use std::collections::{HashMap, HashSet};
pub(crate) fn collect_deps_from_constant(
    constant: &raw_ast::Constant<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
) {
    match constant {
        raw_ast::Constant::Identifier(id) => {
            let name = id.identifier.to_string();
            if !name.contains('/') && !name.contains('.') {
                deps.push(format!("{}/{}", library_name, name));
            } else if !name.contains('/') {
                if let Some((type_name, _)) = name.rsplit_once('.') {
                    deps.push(format!("{}/{}", library_name, type_name));
                }
            } else {
                deps.push(name);
            }
        }
        raw_ast::Constant::BinaryOperator(binop) => {
            collect_deps_from_constant(&binop.left, library_name, deps);
            collect_deps_from_constant(&binop.right, library_name, deps);
        }
        _ => {}
    }
}

pub(crate) fn collect_deps_from_attributes(
    attributes: Option<&raw_ast::AttributeList<'_>>,
    library_name: &str,
    deps: &mut Vec<String>,
) {
    if let Some(attrs) = attributes {
        for attr in &attrs.attributes {
            for arg in &attr.args {
                collect_deps_from_constant(&arg.value, library_name, deps);
            }
        }
    }
}

pub(crate) fn get_dependencies<'node, 'src>(
    decl: &RawDecl<'node, 'src>,
    library_name: &str,
    _decl_kinds: &HashMap<crate::names::OwnedQualifiedName, &'static str>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) -> Vec<String> {
    let mut deps = vec![];

    collect_deps_from_attributes(decl.attributes(), library_name, &mut deps);

    match decl {
        RawDecl::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Enum(e) => {
            if let Some(ref subtype) = e.subtype {
                collect_deps_from_ctor(
                    subtype,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
            for member in &e.members {
                collect_deps_from_constant(&member.value, library_name, &mut deps);
            }
        }
        RawDecl::Bits(b) => {
            if let Some(ref subtype) = b.subtype {
                collect_deps_from_ctor(
                    subtype,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
            for member in &b.members {
                collect_deps_from_constant(&member.value, library_name, &mut deps);
            }
        }
        RawDecl::Union(u) => {
            for member in &u.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Table(t) => {
            for member in &t.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Type(t) => {
            if let Some(s) = option_layout_as_struct(&t.layout) {
                for member in &s.members {
                    collect_deps_from_ctor(
                        &member.type_ctor,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            } else if let Some(e) = option_layout_as_enum(&t.layout) {
                if let Some(ref subtype) = e.subtype {
                    collect_deps_from_ctor(
                        subtype,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                for member in &e.members {
                    collect_deps_from_constant(&member.value, library_name, &mut deps);
                }
            } else if let Some(b) = option_layout_as_bits(&t.layout) {
                if let Some(ref subtype) = b.subtype {
                    collect_deps_from_ctor(
                        subtype,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                for member in &b.members {
                    collect_deps_from_constant(&member.value, library_name, &mut deps);
                }
            } else if let Some(u) = option_layout_as_union(&t.layout) {
                for member in &u.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(
                            ctor,
                            library_name,
                            &mut deps,
                            skip_optional,
                            inline_names,
                        );
                    }
                }
            } else if let Some(ta) = option_layout_as_table(&t.layout) {
                for member in &ta.members {
                    if let Some(ref ctor) = member.type_ctor {
                        collect_deps_from_ctor(
                            ctor,
                            library_name,
                            &mut deps,
                            skip_optional,
                            inline_names,
                        );
                    }
                }
            } else if let raw_ast::Layout::TypeConstructor(ref tc) = t.layout {
                collect_deps_from_ctor(tc, library_name, &mut deps, skip_optional, inline_names);
            }
        }
        RawDecl::Protocol(p) => {
            for m in &p.methods {
                let _method_name_camel = format!(
                    "{}{}",
                    m.name.data().chars().next().unwrap().to_uppercase(),
                    &m.name.data()[1..]
                );
                if let Some(req) = &m.request_payload {
                    collect_deps_from_layout(
                        req,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                if let Some(res) = &m.response_payload {
                    collect_deps_from_layout(
                        res,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
                if let Some(ref err) = m.error_payload {
                    collect_deps_from_layout(
                        err,
                        library_name,
                        &mut deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        RawDecl::Service(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Resource(r) => {
            if let Some(type_ctor) = &r.type_ctor {
                collect_deps_from_ctor(
                    type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
            for prop in &r.properties {
                collect_deps_from_ctor(
                    &prop.type_ctor,
                    library_name,
                    &mut deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        RawDecl::Const(c) => {
            collect_deps_from_ctor(
                &c.type_ctor,
                library_name,
                &mut deps,
                skip_optional,
                inline_names,
            );
        }
        RawDecl::Alias(a) => {
            collect_deps_from_ctor(
                &a.type_ctor,
                library_name,
                &mut deps,
                skip_optional,
                inline_names,
            );
        }
    }

    deps
}

pub(crate) fn option_layout_as_struct<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::StructDeclaration<'a>> {
    if let raw_ast::Layout::Struct(s) = layout {
        Some(s)
    } else {
        None
    }
}

pub(crate) fn option_layout_as_enum<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::EnumDeclaration<'a>> {
    if let raw_ast::Layout::Enum(e) = layout {
        Some(e)
    } else {
        None
    }
}

pub(crate) fn option_layout_as_bits<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::BitsDeclaration<'a>> {
    if let raw_ast::Layout::Bits(b) = layout {
        Some(b)
    } else {
        None
    }
}

pub(crate) fn option_layout_as_union<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::UnionDeclaration<'a>> {
    if let raw_ast::Layout::Union(u) = layout {
        Some(u)
    } else {
        None
    }
}

pub(crate) fn option_layout_as_table<'a>(
    layout: &'a raw_ast::Layout<'a>,
) -> Option<&'a raw_ast::TableDeclaration<'a>> {
    if let raw_ast::Layout::Table(t) = layout {
        Some(t)
    } else {
        None
    }
}

pub(crate) fn collect_deps_from_ctor(
    ctor: &raw_ast::TypeConstructor<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) {
    if skip_optional {
        // Nullable types (e.g., box, optional unions) are placed behind pointers or
        // envelopes and don't strict layout dependency cycles.
        if ctor.nullable {
            return;
        }

        // Check if it has an `:optional` constraint
        for constraint in &ctor.constraints {
            if let raw_ast::Constant::Identifier(id_const) = constraint
                && id_const.identifier.to_string() == "optional"
            {
                return;
            }
        }
    }

    if let raw_ast::LayoutParameter::Identifier(ref id) = ctor.layout {
        let name = id.to_string();
        match name.as_str() {
            "bool" | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32" | "int64"
            | "uint64" | "string" => {}
            "box" | "client_end" | "server_end" => {
                if skip_optional {
                    return;
                }
            }
            _ => {
                deps.push(format!("{}/{}", library_name, name));
            }
        }
    } else if let raw_ast::LayoutParameter::Inline(_) = ctor.layout
        && let Some(name) =
            inline_names.get(&(ctor.element.start_token.span.data.as_ptr() as usize))
    {
        deps.push(name.clone());
    }

    for param in &ctor.parameters {
        collect_deps_from_ctor(param, library_name, deps, skip_optional, inline_names);
    }
}

pub(crate) fn collect_deps_from_layout(
    layout: &raw_ast::Layout<'_>,
    library_name: &str,
    deps: &mut Vec<String>,
    skip_optional: bool,
    inline_names: &HashMap<usize, String>,
) {
    match layout {
        raw_ast::Layout::TypeConstructor(tc) => {
            collect_deps_from_ctor(tc, library_name, deps, skip_optional, inline_names);
        }
        raw_ast::Layout::Struct(s) => {
            for member in &s.members {
                collect_deps_from_ctor(
                    &member.type_ctor,
                    library_name,
                    deps,
                    skip_optional,
                    inline_names,
                );
            }
        }
        raw_ast::Layout::Union(u) => {
            for member in &u.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        raw_ast::Layout::Table(t) => {
            for member in &t.members {
                if let Some(type_ctor) = &member.type_ctor {
                    collect_deps_from_ctor(
                        type_ctor,
                        library_name,
                        deps,
                        skip_optional,
                        inline_names,
                    );
                }
            }
        }
        _ => {}
    }
}

impl<'node, 'src> super::Compiler<'node, 'src> {
    pub fn topological_sort(&self, skip_optional: bool) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut sorted = vec![];
        let mut temp_path = vec![]; // for cycle detection

        let mut keys: Vec<String> = self.raw_decls.keys().map(|k| k.to_string()).collect();
        keys.sort();

        #[allow(clippy::too_many_arguments)]
        pub(crate) fn visit<'a, 'b>(
            name: &str,
            decls: &HashMap<crate::names::OwnedQualifiedName, RawDecl<'a, 'b>>,
            library_name: &crate::names::OwnedLibraryName,
            visited: &mut HashSet<String>,
            temp_path: &mut Vec<String>,
            sorted: &mut Vec<String>,
            decl_kinds: &HashMap<crate::names::OwnedQualifiedName, &'static str>,
            skip_optional: bool,
            inline_names: &HashMap<usize, String>,
            reporter: &Reporter<'b>,
        ) {
            if visited.contains(name) {
                return;
            }
            if let Some(idx) = temp_path.iter().position(|x| x == name) {
                let cycle_names = &temp_path[idx..];
                let mut cycle_str = String::new();
                for cname in cycle_names {
                    let ckind = decl_kinds.get::<str>(cname.as_ref()).unwrap_or(&"unknown");
                    let cname_fqn = crate::names::OwnedQualifiedName::parse(cname);
                    let short_name = cname_fqn.declaration();
                    cycle_str.push_str(&format!("{} '{}' -> ", ckind, short_name));
                }
                let kind = decl_kinds.get::<str>(name.as_ref()).unwrap_or(&"unknown");
                let name_fqn = crate::names::OwnedQualifiedName::parse(name);
                let short_name = name_fqn.declaration();
                cycle_str.push_str(&format!("{} '{}'", kind, short_name));

                let span = if let Some(decl) = decls.get::<str>(name.as_ref()) {
                    decl.element().span()
                } else {
                    let first_decl = decls.values().next().unwrap();
                    first_decl.element().span()
                };

                reporter.fail(Error::ErrIncludeCycle, span, &[&cycle_str]);
                return;
            }
            temp_path.push(name.to_string());

            if let Some(decl) = decls.get::<str>(name.as_ref()) {
                let deps =
                    get_dependencies(decl, &library_name.to_string(), decl_kinds, skip_optional, inline_names);
                // Sort dependencies by name to ensure deterministic order if needed, but they are in AST order
                for dep in deps {
                    visit(
                        &dep,
                        decls,
                        library_name,
                        visited,
                        temp_path,
                        sorted,
                        decl_kinds,
                        skip_optional,
                        inline_names,
                        reporter,
                    );
                }
            }

            temp_path.pop();
            visited.insert(name.to_string());
            if decls.contains_key::<str>(name.as_ref()) {
                sorted.push(name.to_string());
            }
        }

        for name in keys {
            visit(
                &name,
                &self.raw_decls,
                &self.library_name,
                &mut visited,
                &mut temp_path,
                &mut sorted,
                &self.decl_kinds,
                skip_optional,
                &self.inline_names,
                self.reporter,
            );
        }

        sorted
    }

    pub(crate) fn recompute_declaration_order(&mut self) {
        pub(crate) fn get_type_dependencies(ty: &Type) -> Vec<String> {
            let mut deps = Vec::new();
            if ty.nullable() {
                return deps;
            }
            if let Some(id) = ty.identifier() {
                deps.push(id.clone());
            }
            if let Some(inner) = ty.element_type() {
                deps.extend(get_type_dependencies(inner));
            }
            // DO NOT process ty.protocol() because Handles are out-of-line pointers
            // and do not induce topological sorting edges.
            deps
        }
        pub(crate) fn get_constant_deps(c: &flat_ast::Constant) -> Vec<String> {
            let mut deps: Vec<String> = Vec::new();
            if let Some(id) = &c.identifier {
                deps.push(id.clone());
            }
            // For binary expressions etc., we might need more but json_generator
            // constant struct only exposes the top-level identifier! Or wait, if it's
            // a binary expression, the `identifier` field is missing, wait!
            // Wait, binary_operator might have identifiers? In JSON `identifier` is scalar?
            // Actually, in C++, it evaluates `left_operand` and `right_operand`
            // but in JSON, we can just not worry about binary_operator because `UINT32` is identifier.
            // Wait, if binary_operator has no identifier, it might break?
            deps
        }

        let mut all_names = Vec::new();
        let keys: Vec<crate::names::OwnedQualifiedName> = self.raw_decls.keys().cloned().collect();
        for k in &keys {
            all_names.push(k.to_string());
        }
        for decl in &self.union_declarations {
            if !all_names.contains(&decl.name) {
                all_names.push(decl.name.clone());
            }
        }
        for decl in &self.struct_declarations {
            if !all_names.contains(&decl.name) {
                all_names.push(decl.name.clone());
            }
        }
        for decl in &self.table_declarations {
            if !all_names.contains(&decl.name) {
                all_names.push(decl.name.clone());
            }
        }

        all_names.sort();

        let mut deps: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for name in &all_names {
            let mut d = Vec::new();

            if let Some(obj) = self.bits_declarations.iter().find(|x| &x.name == name) {
                d.extend(get_type_dependencies(&obj.type_));
                for m in &obj.members {
                    d.extend(get_constant_deps(&m.value));
                }
            } else if let Some(obj) = self.const_declarations.iter().find(|x| &x.name == name) {
                d.extend(get_type_dependencies(&obj.type_));
                d.extend(get_constant_deps(&obj.value));
            } else if let Some(obj) = self.enum_declarations.iter().find(|x| &x.name == name) {
                for m in &obj.members {
                    d.extend(get_constant_deps(&m.value));
                }
            } else if let Some(obj) = self.struct_declarations.iter().find(|x| &x.name == name) {
                for m in &obj.members {
                    d.extend(get_type_dependencies(&m.type_));
                    if let Some(def_val) = &m.maybe_default_value {
                        d.extend(get_constant_deps(def_val));
                    }
                }
            } else if let Some(obj) = self.table_declarations.iter().find(|x| &x.name == name) {
                for m in &obj.members {
                    if let Some(ref ty) = m.type_ {
                        d.extend(get_type_dependencies(ty));
                    }
                }
            } else if let Some(obj) = self.union_declarations.iter().find(|x| &x.name == name) {
                for m in &obj.members {
                    if let Some(ref ty) = m.type_ {
                        d.extend(get_type_dependencies(ty));
                    }
                }
            } else if let Some(obj) = self.alias_declarations.iter().find(|x| &x.name == name) {
                d.push(obj.partial_type_ctor.name.clone());
            } else if let Some(obj) = self.new_type_declarations.iter().find(|x| &x.name == name) {
                if let Some(alias) = &obj.type_.experimental_maybe_from_alias {
                    d.push(alias.name.clone());
                } else if let Some(id) = obj.type_.identifier() {
                    d.push(id.clone());
                }
            } else if let Some(obj) = self.service_declarations.iter().find(|x| &x.name == name) {
                for m in &obj.members {
                    d.extend(get_type_dependencies(&m.type_));
                }
            } else if let Some(obj) = self
                .experimental_resource_declarations
                .iter()
                .find(|x| &x.name == name)
            {
                d.extend(get_type_dependencies(&obj.type_));
                for prop in &obj.properties {
                    d.extend(get_type_dependencies(&prop.type_));
                }
            } else if let Some(obj) = self.protocol_declarations.iter().find(|x| &x.name == name) {
                for comp in &obj.composed_protocols {
                    d.push(comp.name.clone());
                }
                for m in &obj.methods {
                    if let Some(req) = &m.maybe_request_payload {
                        d.extend(get_type_dependencies(req));
                    }
                    if m.has_error {
                        let name_fqn = crate::names::OwnedQualifiedName::parse(name);
                        let decl_name_short = name_fqn.declaration();
                        let union_name = format!(
                            "{}/{}_{}_Result",
                            self.library_name, decl_name_short, m.name
                        );
                        d.push(union_name);
                    } else if let Some(res) = &m.maybe_response_payload {
                        d.extend(get_type_dependencies(res));
                    }
                }
            }

            // To prevent binary operator identifiers from failing the traversal (if we fall back to raw_decls ast lookup)
            // Actually it's okay because we extracted what JSON exposed. Wait, if it missed `BitsType.THIRD_VALUE` which is binary operator!
            // `CalcDependencies` recurses on BinaryOperator!
            // Wait, `get_dependencies` DOES recurse `RawDecl::Const` and gets all operands!
            // Instead of solely relying on JSON, we can UNION the dependencies with what `get_dependencies` extracts!
            if let Some(raw) = self.raw_decls.get::<str>(name.as_ref()) {
                d.extend(get_dependencies(
                    raw,
                    &self.library_name.to_string(),
                    &self.decl_kinds,
                    true, // skip_optional = true!
                    &self.inline_names,
                ));
            }

            deps.insert(name.clone(), d);
        }

        let mut visited = std::collections::HashSet::new();
        let mut order = Vec::new();

        pub(crate) fn visit<'a>(
            name: &'a str,
            deps: &'a std::collections::HashMap<String, Vec<String>>,
            visited: &mut std::collections::HashSet<String>,
            order: &mut Vec<String>,
            library_name: &str,
        ) {
            if visited.contains(name) {
                return;
            }
            visited.insert(name.to_string());
            if let Some(d) = deps.get(name) {
                for dep in d {
                    visit(dep, deps, visited, order, library_name);
                }
                if name.starts_with(&format!("{}/", library_name)) {
                    order.push(name.to_string());
                }
            }
        }

        for name in &all_names {
            visit(name, &deps, &mut visited, &mut order, &self.library_name.to_string());
        }

        self.declaration_order = order;
    }
}
