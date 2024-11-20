use swc_core::ecma::ast::*;
use swc_core::common::{DUMMY_SP, SyntaxContext};

use crate::config::Config;


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

pub fn create_px2rem_function(_config: &Config) -> Expr {
    let params = vec![Pat::Ident(BindingIdent {
        id: Ident::new("value".into(), DUMMY_SP, SyntaxContext::empty()),
        type_ann: None,
    })];

    let body = Box::new(Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params: vec![Pat::Ident(BindingIdent {
            id: Ident::new("value".into(), DUMMY_SP, SyntaxContext::empty()),
            type_ann: None,
        })],
        body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                "parseFloat".into(),
                DUMMY_SP,
                SyntaxContext::empty(),
            )))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Ident(Ident::new(
                    "value".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                ))),
            }],
            type_args: None,
            ctxt: SyntaxContext::empty(),
        })))),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
        ctxt: SyntaxContext::empty(),
    }));

    Expr::Paren(ParenExpr {
        span: DUMMY_SP,
        expr: Box::new(Expr::Arrow(ArrowExpr {
            span: DUMMY_SP,
            params,
            body: Box::new(BlockStmtOrExpr::Expr(body)),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ctxt: SyntaxContext::empty(),
        })),
    })
}