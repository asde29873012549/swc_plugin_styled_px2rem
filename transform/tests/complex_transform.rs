use styled_components_px2rem::{config::Config, visitor::PxToRem};
use swc_core::ecma::{transforms::testing::test, visit::visit_mut_pass};

// Complex nested structures and combinations
#[cfg(test)]
mod complex_transforms {
    use super::*;

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
    

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        media_queries,
        r#"
        const Responsive = styled.div`
            padding: 16px;
            @media (max-width: 768px) {
                padding: 8px;
                font-size: 14px;
            }
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        mixed_units,
        r#"
        const Mixed = styled.div`
            margin: 16px 1em 24px 2rem;
            padding: 8px 10% 12px calc(100% - 20px);
        `;"#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        css_variables,
        r#"
        const Variables = styled.div`
            --spacing: 16px;
            margin: var(--spacing);
            padding: calc(var(--spacing) * 2);
        `;"#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        complex_calcs,
        r#"
        const Calc = styled.div`
            margin: calc(100vh - 20px);
            padding: calc(16px + 2vw);
            width: calc(100% - 32px);
        `;"#
    );
}