# Taz
Taz is Rust library to evaluate a mathematical expression.
To do this we convert the expression in postfix expression, then we evaluate the postfix expresion using a operand stack.

For example we can evaluate the following expression:
- *1 + 1*
- *2.0 * (4.43 - 5.99) / 3.0*
- *sqrt(x^2 + y^2)*
- *cos(pi / 4.0)^2 + sin(pi / 4.0)^2*

## Build
Build of Taz core is made by [Rust](https://www.rust-lang.org/) tool [Cargo](https://doc.rust-lang.org/cargo/)

To build Taz, you can use the following command:

	*cargo build* to compile in debug mode
	*cargo build --release* to compile in release mode

To launch Taz units tests, you can use the following command:

	*cargo test* to launch tests in debug mode
	*cargo test --release* to launch tests in release mode

## Documentation
Taz core code documentation is made also by Cargo with the following command:

	*cargo doc* to generate the documentation
	*cargo doc --open* to open the documention in your browser

## Licensing
Taz library is free software, you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License.