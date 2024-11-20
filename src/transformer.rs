use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{ VisitMut, VisitMutWith },
};
use swc_core::common::DUMMY_SP;
use swc_core::common::SyntaxContext;
use swc_core::common::util::take::Take;
use crate::{
    config::Config,
    visitor::PxToRem,
    regex::PX_REGEX,
    helpers::is_styled_components,
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

        println!("visit_mut_str: n.value: {}", n.value);

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
                    *expr = insert_px2rem_call(expr, self.px2rem_used);

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
    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        if !is_styled_components(n) {
            return;
        }

        // Create a new template visitor and traverse the template
        let mut visitor = self.create_template_visitor();
        n.tpl.visit_mut_with(&mut visitor);
    }
}

// Move helper functions outside impl blocks
fn transform_css(css: &str, config: &Config) -> String {
    let root_font_size = config.root_value;
    let unit_precision = config.unit_precision;
    
    let result = PX_REGEX.replace_all(css, |caps: &regex::Captures| {
        let px_value: f64 = caps[1].parse().unwrap_or(0.0);
        if px_value < config.min_pixel_value {
            return format!("{}px", px_value);
        }
        let rem_value = (px_value * config.multiplier) / root_font_size;

        if rem_value.fract() == 0.0 {
            // If it's a whole number, don't show decimal places
            format!("{}rem", rem_value as i64)
        } else {
            let formatted =
                format!("{:.precision$}", rem_value, precision = unit_precision)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string();
            
            format!("{}rem", formatted)
        }
    });

    result.to_string()
}

fn create_px2rem_call(expr: Box<Expr>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(
            Ident::new("px2rem".into(), DUMMY_SP, SyntaxContext::empty())
        ))),
        args: vec![ExprOrSpread {
            spread: None,
            expr,
        }],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

fn create_px2rem_call_with_args(expr: Box<Expr>, args_expr: Box<Expr>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(
            Ident::new("px2rem".into(), DUMMY_SP, SyntaxContext::empty())
        ))),
        args: vec![
            ExprOrSpread {
                spread: None,
                expr,
            },
            ExprOrSpread {
                spread: Some(DUMMY_SP),
                expr: args_expr,
            }
        ],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

fn is_pure_expression(expr: &Expr) -> bool {
    match expr {
        Expr::Ident(_) |
        Expr::Call(_) |
        Expr::Bin(_) |
        Expr::Lit(Lit::Str(_)) |
        Expr::Lit(Lit::Num(_)) |
        Expr::Member(_) => true,
        Expr::OptChain(opt_chain) => {
            matches!(&*opt_chain.base, OptChainBase::Call(_) | OptChainBase::Member(_))
        },
        _ => false,
    }
}

fn insert_px2rem_call(expr: &mut Expr, px2rem_used: &mut bool) -> Box<Expr> {
    *px2rem_used = true;
    match expr {
        Expr::Arrow(arrow_expr) => {
            if let BlockStmtOrExpr::BlockStmt(_) = &*arrow_expr.body {
                // Wrap the entire arrow function with px2rem
                Box::new(create_px2rem_call(Box::new(expr.take())))
            } else if let BlockStmtOrExpr::Expr(body_expr) = &mut *arrow_expr.body {
                if is_pure_expression(&**body_expr) {
                    // Wrap only the body with px2rem
                    arrow_expr.body = Box::new(BlockStmtOrExpr::Expr(
                        Box::new(create_px2rem_call(body_expr.take()))
                    ));
                } else {
                    // Recursively process the body
                    let new_body = insert_px2rem_call(&mut **body_expr, px2rem_used);
                    arrow_expr.body = Box::new(BlockStmtOrExpr::Expr(new_body));
                }
                Box::new(expr.take())
            } else {
                Box::new(expr.take())
            }
        },
        Expr::Cond(cond_expr) => {
            // Recursively process the consequent and alternate
            cond_expr.cons = insert_px2rem_call(&mut *cond_expr.cons, px2rem_used);
            cond_expr.alt = insert_px2rem_call(&mut *cond_expr.alt, px2rem_used);
            Box::new(expr.take())
        },
        Expr::Fn(_fn_expr) => {
            // Transform function expression into an arrow function with rest parameters
            let args_ident = Ident::new("args".into(), DUMMY_SP, SyntaxContext::empty());
            let rest_pat = Pat::Rest(RestPat {
                span: DUMMY_SP,
                dot3_token: DUMMY_SP,
                arg: Box::new(Pat::Ident(BindingIdent::from(args_ident.clone()))),
                type_ann: None,
            });

            let arrow_expr = Expr::Arrow(ArrowExpr {
                span: DUMMY_SP,
                params: vec![rest_pat],
                body: Box::new(BlockStmtOrExpr::Expr(Box::new(
                    create_px2rem_call_with_args(
                        Box::new(expr.take()),
                        Box::new(Expr::Ident(args_ident)),
                    )
                ))),
                is_async: false,
                is_generator: false,
                type_params: None,
                return_type: None,
                ctxt: SyntaxContext::empty(),
            });

            Box::new(arrow_expr)
        },
        _ => {
            // Wrap the expression directly with px2rem
            Box::new(create_px2rem_call(Box::new(expr.take())))
        },
    }
}
