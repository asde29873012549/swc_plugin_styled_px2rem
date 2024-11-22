use swc_core::plugin::{
    metadata::TransformPluginProgramMetadata,
    plugin_transform,
};
use swc_core::ecma::{
    ast::*,
    visit::VisitMutWith,
};
use swc_core::common::{DUMMY_SP, SyntaxContext};
use serde_json;

pub mod config;
pub mod visitor;
pub mod transformer;
pub mod helpers;
pub mod regex;
pub mod px2rem_fn;

use config::Config;
use visitor::PxToRem;
use px2rem_fn::create_px2rem_function;

#[plugin_transform]
pub fn styled_components_px2rem(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    // get config specified in the next.config.js
    let config = metadata
        .get_transform_plugin_config()
        .and_then(|json_str| serde_json::from_str::<Config>(&json_str).ok())
        .unwrap_or_default();

    let mut visitor = PxToRem::new(config);
    program.visit_mut_with(&mut visitor);

    if visitor.px2rem_used {
        // create px2rem function
        let px2rem_function = create_px2rem_function(&visitor.config);

        if let Program::Module(module) = &mut program {
            // Inject px2rem function at the beginning of the module
            module.body.insert(0, ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(
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