use styled_components_px2rem::{config::Config, visitor::PxToRem};
use swc_core::ecma::{transforms::testing::test, visit::visit_mut_pass};

// Basic styled-components transformations
#[cfg(test)]
mod basic_transforms {
    use super::*;

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        simple_px_values,
        r#"
        const Button = styled(Button)`
            padding: 16px;
            margin: 8px;
            font-size: 14px;
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        zero_px_values,
        r#"
        const Box = styled.div`
            margin: 0px;
            padding: 0px;
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        decimal_px_values,
        r#"
        const Text = styled.p`
            font-size: 14.5px;
            line-height: 18.2px;
            `;
        "#
    );

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
}
