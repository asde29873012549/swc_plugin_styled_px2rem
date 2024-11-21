const Interpolation = styled(Avatar)`
            margin: ${px2rem(size)};
            padding: 2.133rem ${(props)=>px2rem(props.paddingHorizontal)};
            border: 0.267rem solid #000;
            width: ${({ size })=>px2rem(size)};
            height: ${({ size })=>px2rem(size)};
            margin-bottom: ${({ size })=>{
    return `${px2rem(size / 4)}`;
}};
            `;
