const Interpolation = styled.div`
    width: ${({ width })=>width ? `${px2rem(width)}` : "100%"};
    padding: 6.4rem;
    border-radius: ${({ $isRounded, $borderRadius })=>$isRounded ? `${px2rem($borderRadius)}` : "2.133rem"};
    background-color: white;
    box-shadow: 0 3.2rem 6.4rem rgba(0, 0, 0, 0.12);
    transform: scale(1);
    transition: transform 0.3s ease-in-out;
    margin: ${(props)=>props.size > 10 ? `${px2rem(props.size * 2)}` : `${px2rem(props.size)}`};
    
    &:hover {
        transform: scale(1.05) translateX(1.6rem);
    }
`;
