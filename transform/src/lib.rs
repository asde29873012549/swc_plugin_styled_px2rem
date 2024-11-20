use swc_core::plugin::{
    metadata::TransformPluginProgramMetadata,
    plugin_transform,
};
use swc_core::ecma::{
    ast::*,
    visit::VisitMutWith,
};
use swc_core::common::{DUMMY_SP, SyntaxContext};

pub mod config;
pub mod visitor;
pub mod transformer;
pub mod helpers;
pub mod regex;

use config::Config;
use visitor::PxToRem;
use helpers::create_px2rem_function;

#[plugin_transform]
pub fn styled_components_px2rem(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    let config = Config::default();
    let mut visitor = PxToRem::new(config);
    program.visit_mut_with(&mut visitor);

    if visitor.px2rem_used {
        // Inject px2rem function into the program
        let px2rem_function = create_px2rem_function(&visitor.config);

        if let Program::Module(module) = &mut program {
            module.body.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(
                VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Var,
                    declare: false,
                    decls: vec![VarDeclarator {
                        span: DUMMY_SP,
                        name: Pat::Ident(Ident::new("px2rem".into(), DUMMY_SP, SyntaxContext::empty()).into()),
                        init: Some(Box::new(px2rem_function)),
                        definite: false,
                    }],
                    ctxt: SyntaxContext::empty(),
                },
            )))));
        }
    }

    program
}