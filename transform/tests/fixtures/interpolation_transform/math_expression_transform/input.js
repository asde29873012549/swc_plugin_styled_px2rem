const Dynamic = styled.div`
    margin: ${props => props.margin}px;
    padding: ${({ size }) => size * 2}px;
    width: ${48 / 2}px;
    height: ${48 && 7}px;
    font-size: ${() => Math.min(16, 24)}px;
`;
