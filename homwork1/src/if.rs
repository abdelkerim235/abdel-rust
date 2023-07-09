// Take an arithmetic expression as input and return an AST

# pub fn parse(&mut self) -> Result<Node, ParseError> {
    let ast = self.generate_ast(OperPrec::DefaultZero);
    match ast {
        Ok(ast) => Ok(ast),
        Err(e) => Err(e),
    }
}
