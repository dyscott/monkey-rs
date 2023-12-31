![Unit Tests](https://github.com/dyscott/monkey-rs/actions/workflows/rust.yml/badge.svg)
# monkey-rs
Rust Interpreter and Compiler for Monkey Programming Language

Based on the books [Writing An Interpreter In Go](https://interpreterbook.com/) and [Writing A Compiler In Go](https://compilerbook.com/) by Thorsten Ball

Built as a learning exercise to better understand the Rust and Go programming languages and how interpreters and compilers work

## Usage
Requires [Rust](https://www.rust-lang.org/) to be installed

### REPL
```bash
$ cargo run --release
>> let add = fn(x, y) { x + y };
>> add(1, 2);
3
```

### File Loading
```bash
$ cargo run --release -- --file=examples/fibonacci.monkey
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34 ...]
```
*Use the `-i` or `--interpreter` flag to run in interpreter mode instead of compiler mode for REPL and File Loading*

### Benchmarking
Compare the performance of the interpreter and compiler modes
```bash
$ cargo run --release -- --benchmark --file=examples/fibonacci-benchmark.monkey
Parsing took: 18.791µs
Evaluation (interpreter) took: 42.488771428s
Compilation took: 7.754µs
Execution (VM) took: 5.539178683s
```
*The compiler is almost 8x faster than the interpreter for this example!*

## Features
monkey-rs aims to be a fully featured interpreter and compiler for the Monkey Programming Language with additional features inspired by other languages such as Python.

Features progress is tracked below:
- [x] Language features from [Monkey Programming Language](https://monkeylang.org/)
  - [x] C-like syntax
  - [x] Primitive Types (integers, booleans, strings, arrays, hash maps)
  - [x] Arithmetic Expressions
  - [x] Let and return statements
  - [x] Conditionals
  - [x] Functions (first-class, higher-order, closures)
  - [x] Built-in functions (len, puts, push, etc.)
- [x] Additional language features
  - [x] Better string parsing - character escaping and error handling
  - [x] String indexing (ex: `"hello"[4]` -> `4`)
  - [x] Python-like string and array slicing (ex: `[1, 2, 3, 4][1:-1]` -> `[2, 3]`)
- [x] REPL and File Loading
- [x] Interpreter
  - [x] Lexer / Tokenizer
  - [x] Pratt Parser
  - [x] Abstract Syntax Tree
  - [x] Evaluator
- [x] Bytecode Compiler 
  - [x] Conversion from AST to Bytecode
  - [ ] File Output
- [x] Virtual Machine
  - [x] Stack-based VM
  - [x] Bytecode Interpreter
- [x] Unit Tests

The only major missing feature is file output for the bytecode compiler - this is currently a work in progress. Perhaps new language features and performance improvements could be added in the future as well.