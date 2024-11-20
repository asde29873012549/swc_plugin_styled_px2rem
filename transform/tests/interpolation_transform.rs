use styled_components_px2rem::{config::Config, visitor::PxToRem};
use swc_core::ecma::{transforms::testing::test, visit::visit_mut_pass};

// Template literal interpolation cases
#[cfg(test)]
mod interpolation_transforms {
    use super::*;

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

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        ternary_expression_transform,
        r#"
        const Interpolation = styled.div`
            width: ${({ width }) => width ? `${width}px` : "100%"};
            padding: 24px;
            border-radius: ${({ $isRounded, $borderRadius }) => $isRounded ? `${$borderRadius}px` : "8px"};
            background-color: white;
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);
            transform: scale(1);
            transition: transform 0.3s ease-in-out;
            margin: ${props => (props.size > 10 ? `${props.size * 2}px` : `${props.size}px`)};
    
            &:hover {
                transform: scale(1.05) translateX(6px);
            }
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        logical_operator_expression_transform,
        r#"
        const Interpolation = styled.div`
            font-size: ${({ $fontSize }) => $fontSize || "12px"};
            padding: ${props => props.padding || "16px"};
            line-height: 1.6;
            color: #636E72;
            margin: 0;
            text-align: center;
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        math_expression_transform,
        r#"
        const Dynamic = styled.div`
            margin: ${props => props.margin}px;
            padding: ${({ size }) => size * 2}px;
            `;
        "#
    );

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

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        nested_interpolation_transform,
        r#"
        const Interpolation = styled.div`
            margin: ${props => `${props.margin}px`};
            padding: ${props => `${props.padding}px`} ${props => `${props.horizontalPadding}px`};
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        multiple_units_interpolation_transform,
        r#"
        const Interpolation = styled.div`
            width: ${props => props.width}px;
            height: ${props => props.height}rem;
            padding: ${props => props.padding}px ${props => props.margin}px;
            `;
        "#
    );

    test!(
        Default::default(),
        |_| visit_mut_pass(PxToRem::new(Config::default())),
        function_call_interpolation_transform,
        r#"
        const Interpolation = styled.div`
            padding: ${props => calculatePadding(props.size)}px;
            `;
        "#
    );
}