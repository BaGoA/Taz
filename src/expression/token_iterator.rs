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

    /// Filter token according to predicat
    /// The function return an TokenIterator that return filtered token
    #[cfg(test)]
    fn filter<P>(self, predicat: P) -> FilterToken<Self, P>
    where
        Self: Sized,
        P: Fn(Token) -> bool,
    {
        return FilterToken::new(self, predicat);
    }
}

/// Filter token iterator
#[cfg(test)]
pub struct FilterToken<T, P>
where
    T: TokenIterator,
    P: Fn(Token) -> bool,
{
    token_iterator: T,
    predicat: P,
}

#[cfg(test)]
impl<T, P> FilterToken<T, P>
where
    T: TokenIterator,
    P: Fn(Token) -> bool,
{
    /// Create FilterToken iterator from another TokenIterator and a predicat
    pub fn new(token_iterator: T, predicat: P) -> Self {
        return Self {
            token_iterator,
            predicat,
        };
    }
}

#[cfg(test)]
impl<T, P> TokenIterator for FilterToken<T, P>
where
    T: TokenIterator,
    P: Fn(Token) -> bool,
{
    fn next_token(&mut self) -> Result<Token, String> {
        let mut token: Token = self.token_iterator.next_token()?;

        while !(self.predicat)(token) {
            token = self.token_iterator.next_token()?;
        }

        return Ok(token);
    }
}
