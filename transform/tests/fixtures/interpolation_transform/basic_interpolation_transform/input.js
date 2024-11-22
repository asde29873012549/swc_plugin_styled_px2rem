const Interpolation = styled(Avatar)`
    margin: ${size}px;
    padding: 8px ${props => props.paddingHorizontal}px;
    border: 1px solid #000;
    width: ${({ size }) => size}px;
    height: ${({ size }) => size}px;
    margin-bottom: ${({ size }) => {
        return `${size / 4}px`
    }};
`;
