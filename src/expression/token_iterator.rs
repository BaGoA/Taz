use crate::token::Token;

/// Trait to specify an iterator over set of token
pub trait TokenIterator {
    /// Get the next token.
    /// If a error occurs during the iteration, we return an error message in Err of the result.
    fn next_token(&mut self) -> Result<Token, String>;

    /// Collect all token into vector
    /// If a error occurs during the collect, we return an error message in Err of the result.
    fn collect_all_tokens(mut self) -> Result<Vec<Token>, String>
    where
        Self: Sized,
    {
        let mut tokens: Vec<Token> = Vec::with_capacity(25);

        let mut token: Token = self.next_token()?;

        while token != Token::Stop {
            if token != Token::Empty {
                tokens.push(token);
            }

            token = self.next_token()?;
        }

        return Ok(tokens);
    }
}
