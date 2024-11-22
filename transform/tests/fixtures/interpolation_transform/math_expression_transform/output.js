const Dynamic = styled.div`
    margin: ${(props) => px2rem(props.margin)};
    padding: ${({ size }) => px2rem(size * 2)};
    width: ${px2rem(48 / 2)};
    height: ${px2rem(48 && 7)};
    font-size: ${() => px2rem(Math.min(16, 24))};
`;
