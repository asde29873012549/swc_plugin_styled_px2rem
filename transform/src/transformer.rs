use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{ VisitMut, VisitMutWith },
};
use crate::{
    config::Config,
    visitor::PxToRem,
    helpers::{ is_styled_components, is_styled_function, transform_css, wrap_with_px2rem },
};

struct TemplateVisitor<'a> {
    config: Config,
    px2rem_used: &'a mut bool,
}

impl PxToRem {
    pub fn new(config: Config) -> Self {
        Self { config, px2rem_used: false }
    }

    fn create_template_visitor(&mut self) -> TemplateVisitor {
        TemplateVisitor {
            config: self.config.clone(),
            px2rem_used: &mut self.px2rem_used,
        }
    }
}

impl<'a> VisitMut for TemplateVisitor<'a> {
    // Handle template elements (static string parts)
    fn visit_mut_tpl_element(&mut self, n: &mut TplElement) {
        n.visit_mut_children_with(self);

        let raw = &*n.raw;
        let transformed = transform_css(raw, &self.config);
        n.raw = JsWord::from(transformed.clone());
        n.cooked = Some(JsWord::from(transformed));
    }

    // Handle string literals
    fn visit_mut_str(&mut self, n: &mut Str) {
        n.visit_mut_children_with(self);

        if n.value.contains("px") {
            let transformed = transform_css(&n.value, &self.config);
            *n = Str {
                span: n.span,
                value: JsWord::from(transformed),
                raw: None,
            };
        }
    }

    // Handle the entire template literal (for runtime transformations)
    fn visit_mut_tpl(&mut self, n: &mut Tpl) {
        n.visit_mut_children_with(self);

        // Then handle runtime transformations
        let quasis = &mut n.quasis;
        let exprs = &mut n.exprs;

        for (i, expr) in exprs.iter_mut().enumerate() {
            // Visit nested template literals and expressions
            expr.visit_mut_with(self);

            if let Some(next_quasi) = quasis.get(i + 1) {
                if next_quasi.raw.starts_with("px") {
                    // Transform expression with px2rem
                    *expr = wrap_with_px2rem(expr, self.px2rem_used);

                    // Remove px suffix from the next quasi
                    let quasi = &mut quasis[i + 1];
                    let new_raw = quasi.raw.trim_start_matches("px").to_string();
                    quasi.raw = JsWord::from(new_raw.clone());
                    quasi.cooked = Some(JsWord::from(new_raw));
                }
            }
        }
    }
}

impl VisitMut for PxToRem {
    // Handle tagged template literals of styled-components eg: styled.div`...`
    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        if !is_styled_components(n) {
            return;
        }

        // Create a new template visitor and traverse the template
        let mut visitor = self.create_template_visitor();
        n.tpl.visit_mut_with(&mut visitor);
    }

    // Handle CallExpression form of styled-components eg: styled.div(`...`)
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        if !is_styled_function(n) {
            return;
        }

        // Process the arguments if they contain template literals
        for arg in n.args.iter_mut() {
            let mut visitor = self.create_template_visitor();
            arg.expr.visit_mut_with(&mut visitor);
        }
    }

    // Handle JSXAttribute form of styled-components eg: <div styled={{ padding: '10px' }} />
    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        if !self.config.transform_jsx_attributes {
            return;
        }

        // Process the value if it exists
        if let Some(value) = &mut n.value {
            let mut visitor = self.create_template_visitor();
            value.visit_mut_with(&mut visitor);
        }
    }
}