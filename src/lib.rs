use swc_core::plugin::{
    metadata::TransformPluginProgramMetadata,
    plugin_transform,
};
use swc_core::ecma::{
    ast::*,
    visit::{VisitMutWith, visit_mut_pass},
    transforms::testing::test
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
pub fn process_program(
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


#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    basic_transform,
    r#"const Button = styled(Button)`
        padding: 16px;
        margin: 8px;
        font-size: 14px;
    `;"#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    nested_transform,
    r#"
    const Container = styled.div`
        padding: 32px;
        > div {
            margin: 16px;
            font-size: 14px;
        }
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    css_tag_transform,
    r#"
    const styles = css`
        width: 200px;
        height: 100px;
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    keyframes_tag_transform,
    r#"
    const fadeIn = keyframes`
        from {
            margin-top: 20px;
            opacity: 0;
        }
        to {
            margin-top: 0px;
            opacity: 1;
        }
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    create_global_style_transform,
    r#"
    const GlobalStyle = createGlobalStyle`
        body {
            margin: 0px;
            padding: 16px;
            font-size: 14px;
        }
    `;
    "#
);


#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    mixed_units_transform,
    r#"
    const Mixed = styled.div`
        margin: 16px 1em 24px 2rem;
        padding: 8px 10%;
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    complex_transform,
    r#"
    const Card = styled.div`
        margin: 16px;
        padding: 24px;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

        > header {
            font-size: 20px;
            margin-bottom: 16px;
        }

        > section {
            font-size: 14px;
            line-height: 20px;

            &:hover {
                color: red;
                font-size: 16px;
            }
        }

        &:hover {
            transform: translateY(-2px);
        }
    `;
    "#
);


#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    basic_interpolation_transform,
    r#"
    const Interpolation = styled.div`
        margin: ${size}px;
        padding: 8px ${props => props.paddingHorizontal}px;
        border: 1px solid #000;
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    ternary_interpolation_transform,
    r#"
    const Interpolation = styled.div`
        width: ${({ width }) => width ? `${width}px` : "100%"};
        padding: 24px;
        border-radius: ${({ $isRounded }) => $isRounded ? "16px" : "8px"};
        background-color: white;
        box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);
        transform: scale(1);
        transition: transform 0.3s ease-in-out;

        &:hover {
            transform: scale(1.05) translateX(6px);
        }
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    operator_interpolation_transform,
    r#"
    const Interpolation = styled.div`
        font-size: ${({ $fontSize }) => $fontSize || "12px"};
        line-height: 1.6;
        color: #636E72;
        margin: 0;
        text-align: center;
    `;
    "#
);

#[cfg(test)]
test!(
    Default::default(),
    |_| visit_mut_pass(PxToRem::new(Config::default())),
    block_stmt_interpolation_transform,
    r#"
    const Interpolation = styled.div`
        width: ${({ $size }) => $size}px;
        margin-bottom: ${({ $size }) => {
            return $size / 5
        }}px;
    `;
    "#
);