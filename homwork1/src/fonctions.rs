// Create a new instance of Parserpub fn new(expr: &'a str) ->
pub fn new(expr: &'a str) -> Result<Self, ParseError> {
    let mut lexer = Tokenizer::new(expr);
    let cur_token = match lexer.next() {
        Some(token) => token,
        None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    };
    Ok(Parser {
        tokenizer: lexer,
        current_token: cur_token,
    })
}
