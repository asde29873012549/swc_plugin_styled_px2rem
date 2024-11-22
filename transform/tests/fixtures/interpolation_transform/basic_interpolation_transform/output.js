const Interpolation = styled(Avatar)`
    margin: ${px2rem(size)};
    padding: 2.133rem ${(props)=>px2rem(props.paddingHorizontal)};
    border: 0.267rem solid #000;
    width: ${({ size: size1 })=>px2rem(size1)};
    height: ${({ size: size1 })=>px2rem(size1)};
    margin-bottom: ${({ size: size1 })=>{
        return `${px2rem(size1 / 4)}`;
    }};
`;
