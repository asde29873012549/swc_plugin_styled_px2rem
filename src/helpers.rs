use swc_core::ecma::ast::*;

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