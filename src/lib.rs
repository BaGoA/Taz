/*
Taz is mathematical expression evaluation library
Copyright (C) 2022  Bastian Gonzalez Acevedo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod constants;
mod functions;
mod operators;
mod token;

mod evaluator;
mod tokenizer;

pub fn evaluate(expression: &String) -> Result<f64, String> {
    let _postfix_token: Vec<token::Token> =
        evaluator::infix_to_postfix(&tokenizer::tokenize(expression.as_str())?)?;

    return Ok(43.0);
}
