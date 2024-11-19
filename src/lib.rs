use swc_core::plugin::{
    metadata::TransformPluginProgramMetadata,
    plugin_transform,
};
use swc_core::ecma::{
    ast::Program,
    visit::{VisitMutWith, visit_mut_pass},
    transforms::testing::test,
};

pub mod config;
pub mod visitor;
pub mod transformer;
pub mod helpers;
pub mod regex;

use config::Config;
use visitor::PxToRem;

#[plugin_transform]
pub fn process_program(
    program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    let config = Config::default();
    let mut program = program;
    let mut visitor = PxToRem::new(config);
    program.visit_mut_with(&mut visitor);
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
        }

        &:hover {
            transform: translateY(-2px);
        }
    `;
    "#
);
