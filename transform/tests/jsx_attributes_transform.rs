use styled_components_px2rem::{config::Config, visitor::PxToRem};
use swc_core::ecma::{transforms::testing::test, visit::visit_mut_pass};
use swc_ecma_parser::{EsSyntax, Syntax};

#[cfg(test)]
mod jsx_attribute_transforms {
    use super::*;

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        simple_jsx_prop,
        r#"
        <div width="100px" />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        simple_jsx_prop_with_interpolation,
        r#"
        <div width={`${size}px`} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        simple_jsx_prop_with_multiple_interpolation,
        r#"
        <div width={`${width}px`} height={`${height}px`} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        simple_jsx_prop_with_multiple_values_in_interpolation,
        r#"
        <div size={`${width}px ${height}px`} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        simple_jsx_style_prop,
        r#"
        <div style={{ padding: "16px", margin: "8px" }} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        jsx_style_with_expressions,
        r#"
        <div style={{ 
            padding: `${size}px`,
            margin: `${margin}px ${padding}px`
        }} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        jsx_style_with_object_spread,
        r#"
        <div style={{ 
            ...baseStyles,
            padding: "20px",
            ...overrideStyles
        }} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        jsx_style_with_conditional,
        r#"
        <div style={{ 
            padding: isLarge ? "32px" : "16px",
            margin: isMobile && "8px"
        }} />
        "#
    );

    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            let mut config = Config::default();
            config.transform_jsx_attributes = true;
            visit_mut_pass(PxToRem::new(config))
        },
        jsx_style_with_function_calls,
        r#"
        <div style={{ 
            padding: getPadding("16px"),
            margin: calculateSpacing(8) + "px"
        }} />
        "#
    );

    // Test that transformation is skipped when config.transform_jsx_attribute is false
    test!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        jsx_style_transform_disabled,
        r#"
        <div style={{ padding: "16px", margin: "8px" }} />
        "#
    );
}