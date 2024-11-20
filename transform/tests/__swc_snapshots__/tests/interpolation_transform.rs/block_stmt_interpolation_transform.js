const Interpolation = styled.div`
            width: ${({ $size })=>px2rem($size)};
            margin-bottom: ${px2rem(({ $size })=>{
    return $size / 5;
})};
            `;
