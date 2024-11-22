const NestedComponent = styled.div`
    ${props => `
        padding: ${px2rem(props.size)};
        margin: ${px2rem(props.margin)};
    `}
    ${({ theme }) => theme.isMobile && `
        font-size: 3.733rem;
        line-height: 4.267rem;
    `}
    ${() => {
        const size = "4.267rem";
        const base_margin = 12;
        const avartar_margin = base_margin / 2;

        return `
            margin-bottom: ${px2rem(avartar_margin)};
            width: ${size};
            height: ${size};

            &:hover {
                background-color: red;
                transform: scale(1.1);
                padding: 2.667rem;
            }
        `;
    }}
    ${absolute({ top: "0.8rem", left: "0.533rem", zIndex: 1 })};
`;
