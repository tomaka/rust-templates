use std::collections::HashMap;
use std::collections::hash_map::Entry;
use syntex_syntax::ast;
use syntex_syntax::parse;
use syntex_syntax::print::pprust::path_to_string;
use syntex_syntax::print::pprust::ty_to_string;
use syntex_syntax::visit;

use template;

/// Takes a template's content and outputs a String containing the final output.
pub fn analyze(prefix: &str, elements: &[template::Element]) -> String {
    let mut idents = HashMap::new();
    let mut codegen = String::new();
    for element in elements.iter() {
        match element {
            &template::Element::Static(ref s) => {
                codegen.push_str(&format!("out.push_str(r##\"{}\"##);", s));      // TODO: should Rust-escape `s` maybe?
            },
            &template::Element::RustExpr(ref expr) => {
                for (ident, ty) in indiv_analyze(expr) {
                    match idents.entry(ident) {
                        Entry::Vacant(e) => {
                            e.insert(ty);
                        },
                        Entry::Occupied(mut e) => {
                            if e.get().is_none() {
                                e.insert(ty);
                            } else if ty.is_some() {
                                assert_eq!(e.get(), &ty);
                            }
                        },
                    }
                }

                codegen.push_str(&format!("out.push_str(&escape(to_string({})));", expr));
            },
        }
    }

    format!(r#"
struct {prefix};
impl {prefix} {{
    pub fn build<{template_params_reqs}>(input: {prefix}Main<{template_params}>) -> String {{
        let mut out = String::new();
        fn to_string<S: ::std::fmt::Display>(s: S) -> String {{ s.to_string() }}
        fn escape(s: String) -> String {{ s.replace("<", "&lt;").replace(">", "gt;").replace("\"", "&quot;") }}       /* TODO: escape correctly */
        {read_from_input}
        {codegen}
        out
    }}
}}

struct {prefix}Main<{template_params}> {{
    {variables}
}}
"#, codegen = codegen, prefix = prefix,
    variables = idents.keys().map(|v| format!("{}: {}", v, v)).collect::<Vec<_>>().join(", "),
    template_params = idents.keys().cloned().collect::<Vec<_>>().join(", "),
    template_params_reqs = idents.iter().map(|(i, ty)| if let Some(ty) = ty.as_ref() { format!("{}: Into<{}>", i, ty) } else { format!("{}: Into<String>", i) }).collect::<Vec<_>>().join(", "),
    read_from_input = idents.keys().map(|i| format!("let {} = input.{}.into();", i, i)).collect::<Vec<_>>().concat())
}

// Returns the list of idents used in the expression, and their type if known. 
fn indiv_analyze(input: &str) -> HashMap<String, Option<String>> {
    // TODO: correct span
    let session = parse::ParseSess::new();
    let mut parser = parse::new_parser_from_source_str(&session, vec![], "".to_owned(),
                                                       input.to_owned());

    let expr = parser.parse_expr().unwrap();

    let mut idents = HashMap::new();

    struct Visitor<'a>(&'a mut HashMap<String, Option<String>>);
    impl<'a> visit::Visitor for Visitor<'a> {
        fn visit_expr(&mut self, ex: &ast::Expr) {
            match ex.node {
                ast::ExprKind::Type(ref sub_expr, ref ty) => {
                    match sub_expr.node {
                        ast::ExprKind::Path(_, ref path) => {
                            let name = path_to_string(path);
                            let ty = ty_to_string(ty);
                            match self.0.entry(name) {
                                Entry::Vacant(e) => { e.insert(Some(ty)); },
                                Entry::Occupied(mut e) => {
                                    if e.get().is_none() {
                                        e.insert(Some(ty));
                                    } else {
                                        assert_eq!(e.get(), &Some(ty));
                                    }
                                },
                            };
                        },
                        _ => ()
                    }
                },

                ast::ExprKind::Path(_, ref path) => {
                    let name = path_to_string(path);
                    match self.0.entry(name) {
                        Entry::Vacant(e) => { e.insert(None); },
                        _ => ()
                    };
                },

                _ => ()
            };

            visit::walk_expr(self, ex);
        }
    }

    visit::Visitor::visit_expr(&mut Visitor(&mut idents), &expr);
    idents
}
