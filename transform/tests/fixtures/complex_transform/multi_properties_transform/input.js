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
