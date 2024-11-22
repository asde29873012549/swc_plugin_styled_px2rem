use swc_core::ecma::ast::*;
use swc_core::common::{DUMMY_SP, SyntaxContext};

use crate::config::Config;


// creation of px2rem function using AST
// created function:
// var px2rem = (value) => "".concat((parseFloat(value) / {root_value} * {multiplier}).toFixed({unit_precision}), "rem");

pub fn create_px2rem_function(config: &Config) -> Expr {
    // Create parameter: (value)
    let param = Pat::Ident(BindingIdent {
        id: Ident::new("value".into(), DUMMY_SP, SyntaxContext::empty()),
        type_ann: None,
    });

    // Create the calculation: (value / root_value) * multiplier
    let calculation = Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::Mul,
        left: Box::new(Expr::Bin(BinExpr {
            span: DUMMY_SP,
            op: BinaryOp::Div,
            left: Box::new(Expr::Call(CallExpr {
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
            })),
            right: Box::new(Expr::Lit(Lit::Num(Number::from(config.root_value)))),
        })),
        right: Box::new(Expr::Lit(Lit::Num(Number::from(config.multiplier)))),
    }));

    // Create toFixed call with unit_precision
    let to_fixed_call = Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: calculation,
            prop: MemberProp::Ident(IdentName {
                sym: "toFixed".into(),
                span: DUMMY_SP,
            }),
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Num(Number::from(config.unit_precision)))),
        }],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    }));

    // Create template literal: `${value}rem`
    let template = Box::new(Expr::Tpl(Tpl {
        span: DUMMY_SP,
        exprs: vec![to_fixed_call],
        quasis: vec![
            TplElement {
                span: DUMMY_SP,
                cooked: Some("".into()),
                raw: "".into(),
                tail: false,
            },
            TplElement {
                span: DUMMY_SP,
                cooked: Some("rem".into()),
                raw: "rem".into(),
                tail: true,
            },
        ],
    }));

    // Create the function body
    let body = BlockStmtOrExpr::Expr(template);

    // Create the arrow function
    Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params: vec![param],
        body: Box::new(body),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
        ctxt: SyntaxContext::empty(),
    })
}
