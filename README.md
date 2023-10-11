# intro-to-rust

## why [Rust](https://www.rust-lang.org)?
- Rust offers fast performance with no runtime or garbage collector, enabling it to power performance-critical services and easily integrate with other languages.
- Rust is safer. Rust offers protection of both — its own abstractions and abstractions made by the developers. Languages like C++ lack such features.
- Rust is fast. Compared to Java, Rust offers faster startup times and a smaller memory footprint being faster.
- Rust is a well-designed programming language. Rust programming language allows developers to put statements in a lambda and everything is an expression, making it easier to compose particular parts of the language.
- Rust offers a flexible and expressive system. The programming language allows developers to define new container types that can hold different types of elements, generics, traits, and algebra data types.

## getting started
The primary way that folks install Rust is through a tool called Rustup, which is a Rust installer and version management tool.

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Test the installation was successful with

```cargo --version```

## Is Rust up to date?
Rust updates very frequently. If you have installed Rustup some time ago, chances are your Rust version is out of date. Get the latest version of Rust by running.

```rustup update```

## hello world
To start, we’ll use Cargo to make a new project.

```cargo new hello-world```

inside the newly created folder the code can be compiled and run with

```cargo run``
