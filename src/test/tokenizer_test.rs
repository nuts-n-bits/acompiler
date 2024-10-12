use crate::tokenizer::Token;

#[test]
fn tokenizer_test_0() {
        let tokens = crate::tokenizer::tokenize(r###"
        + - * = ident identifier lol <<<< () {{{{
        "###);
        dbg!("Tokens: ");
        dbg!(&tokens);
        let expected_tokens = vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Equal,
                Token::Identifier("ident".into()),
                Token::Identifier("identifier".into()),
                Token::Identifier("lol".into()),
                Token::Lt,
                Token::Lt,
                Token::Lt,
                Token::Lt,
                Token::Lparen,
                Token::Rparen,
                Token::Lbrace,
                Token::Lbrace,
                Token::Lbrace,
                Token::Lbrace,
                Token::EOF,
        ];
        dbg!(&expected_tokens);
        assert_eq!(tokens, expected_tokens);
}