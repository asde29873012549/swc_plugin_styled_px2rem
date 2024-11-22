use swc_core::ecma::ast::*;
use swc_core::common::{
    DUMMY_SP,
    SyntaxContext,
    util::take::Take,
};
use crate::{
    config::Config,
    regex::PX_REGEX,
};

pub fn is_styled_identifier(ident: &Ident) -> bool {
    let tags = ["styled", "css", "createGlobalStyle", "keyframes"];
    tags.contains(&ident.sym.as_ref())
}

pub fn is_styled_member(member_expr: &MemberExpr) -> bool {
    match &*member_expr.obj {
        Expr::Ident(ident) => is_styled_identifier(ident),
        Expr::Member(member) => is_styled_member(member),
        _ => false,
    }
}

pub fn is_styled_function(call_expr: &CallExpr) -> bool {
    match &call_expr.callee {
        Callee::Expr(expr) => match &**expr {
            Expr::Ident(ident) => is_styled_identifier(ident),
            Expr::Member(member) => is_styled_member(member),
            Expr::Call(call) => is_styled_function(call),
            _ => false,
        },
        _ => false,
    }
}

pub fn is_styled_components(tagged_tpl: &TaggedTpl) -> bool {
    let tag_expr = &tagged_tpl.tag;
    match &**tag_expr {
        Expr::Ident(ident) => is_styled_identifier(ident),
        Expr::Member(member) => is_styled_member(member),
        Expr::Call(call) => is_styled_function(call),
        _ => false,
    }
}

// check if either the following:
// - identifier
// - call expression
// - binary expression
// - string literal
// - number literal
// - member expression
// - optional chaining
pub fn is_pure_expression(expr: &Expr) -> bool {
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

pub fn transform_css(css: &str, config: &Config) -> String {
    let root_font_size = config.root_value;
    let unit_precision = config.unit_precision;
    
    let result = PX_REGEX.replace_all(css, |caps: &regex::Captures| {
        let px_value: f64 = caps[1].parse().unwrap_or(0.0);
        if px_value < config.min_pixel_value {
            return format!("{}px", px_value);
        }
        let rem_value = (px_value * config.multiplier) / root_font_size;

        if rem_value.fract() == 0.0 {
            // If it's a integer, don't show decimal places
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

// create px2rem call expression
fn create_px2rem_call(expr: Box<Expr>, args_expr: Option<Box<Expr>>) -> Expr {
    let mut args = vec![ExprOrSpread {
        spread: None,
        expr,
    }];

    if let Some(extra_args) = args_expr {
        args.push(ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: extra_args,
        });
    }

    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(
            Ident::new("px2rem".into(), DUMMY_SP, SyntaxContext::empty())
        ))),
        args,
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

// wrap expression with px2rem call expression
pub fn wrap_with_px2rem(expr: &mut Expr, px2rem_used: &mut bool) -> Box<Expr> {
    *px2rem_used = true;
    match expr {
        Expr::Arrow(arrow_expr) => {
            if let BlockStmtOrExpr::BlockStmt(_) = &*arrow_expr.body {
                // Wrap the entire arrow function with px2rem
                Box::new(create_px2rem_call(Box::new(expr.take()), None))
            } else if let BlockStmtOrExpr::Expr(body_expr) = &mut *arrow_expr.body {
                if is_pure_expression(&**body_expr) {
                    // Wrap only the body with px2rem
                    arrow_expr.body = Box::new(BlockStmtOrExpr::Expr(
                        Box::new(create_px2rem_call(body_expr.take(), None))
                    ));
                } else {
                    // Recursively process the body
                    let new_body = wrap_with_px2rem(&mut **body_expr, px2rem_used);
                    arrow_expr.body = Box::new(BlockStmtOrExpr::Expr(new_body));
                }
                Box::new(expr.take())
            } else {
                Box::new(expr.take())
            }
        },
        Expr::Cond(cond_expr) => {
            // Recursively process the consequent and alternate
            cond_expr.cons = wrap_with_px2rem(&mut *cond_expr.cons, px2rem_used);
            cond_expr.alt = wrap_with_px2rem(&mut *cond_expr.alt, px2rem_used);
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
                    create_px2rem_call(
                        Box::new(expr.take()),
                        Some(Box::new(Expr::Ident(args_ident))),
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
            Box::new(create_px2rem_call(Box::new(expr.take()), None))
        },
    }
}