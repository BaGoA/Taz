use crate::token::Token;

/// Trait to specify an iterator over set of token
pub trait TokenIterator {
    /// Get the next token.
    /// If a error occurs during the iteration, we return an error message in Err of the result.
    fn next_token(&mut self) -> Result<Token, String>;

    /// Determines if the elements of this TokenIterator are equal to to those of vector of tokens
    #[cfg(test)]
    fn equal(mut self, tokens: &[Token]) -> Result<bool, String>
    where
        Self: Sized,
    {
        let mut next_token: Token = self.next_token()?;

        for token in tokens {
            // If next_token is a Token::Stop that means that the TokenIterator reach the end of its iterations
            // But there are still tokens left to test, so we return false also in this case
            if next_token == Token::Stop || *token != next_token {
                return Ok(false);
            }

            next_token = self.next_token()?;
        }

        // Last token give by the TokenIterator must be equal to Token::Stop
        return Ok(next_token == Token::Stop);
    }

    /// Collect all token into vector
    /// If a error occurs during the collect, we return an error message in Err of the result.
    #[cfg(test)]
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
