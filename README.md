# Taz
Taz is Rust library to evaluate a mathematical expression.

For example we can evaluate the following expression:
- *1 + 1*
- *2.0 * (4.43 - 5.99) / 3.0*
- *sqrt(x^2 + y^2)*
- *cos(pi / 4.0)^2 + sin(pi / 4.0)^2*

## Build
Build of Taz is made by [Rust](https://www.rust-lang.org/) tool [Cargo](https://doc.rust-lang.org/cargo/)

To build Taz, you can use the following command:

	*cargo build* to compile in debug mode
	*cargo build --release* to compile in release mode

To launch Taz units tests, you can use the following command:

	*cargo test* to launch tests in debug mode
	*cargo test --release* to launch tests in release mode

## Code Documentation
Taz code documentation is made also by Cargo with the following command:

	*cargo doc* to generate the documentation
	*cargo doc --open* to open the documention in your browser

## Documentation
The evaluation begin by a tokenization. This step transform a string representing the expression to list of tokens which
correspond to infix expression

Then, we convert this infix expression into postfix expression. Indeed an evaluation of postfix expression is easier.

Finally we evaluate the posfix expression by stack method.
