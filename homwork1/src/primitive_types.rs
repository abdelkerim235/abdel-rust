// Construct AST node for numbers, taking into account
// negative prefixes while handling parenthesis


# fn parse_number(&mut self) -> Result<Node, ParseError> {
    let token = self.current_token.clone();
    match token {
        Token::Subtract => {
            self.get_next_token()?;
            let expr = self.generate_ast(OperPrec::Negative)?;
            Ok(Node::Negative(Box::new(expr)))
        }
        Token::Num(i) => {
            self.get_next_token()?;
            Ok(Node::Number(i))
        }
        Token::LeftParen => {
            self.get_next_token()?;
            let expr = self.generate_ast(OperPrec::DefaultZero)?;
            self.check_paren(Token::RightParen)?;

            if self.current_token == Token::LeftParen {
                let right = self.generate_ast(OperPrec::MulDiv)?;
                return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
            }

            Ok(expr)
        }
        _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
    }
}