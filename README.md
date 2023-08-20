![Unit Tests](https://github.com/dyscott/monkey-rs/actions/workflows/rust.yml/badge.svg)
# monkey-rs
Rust Interpreter and Compiler for Monkey Programming Language

Based on the books [Writing An Interpreter In Go](https://interpreterbook.com/) and [Writing A Compiler In Go](https://compilerbook.com/) by Thorsten Ball

Built as a learning exercise to better understand the Rust and Go programming languages and how interpreters and compilers work

## Features
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
  - [ ] Loops (for, while)
- [x] REPL and File Loading
- [x] Interpreter
  - [x] Lexer / Tokenizer
  - [x] Pratt Parser
  - [x] Abstract Syntax Tree
  - [x] Evaluator
- [ ] Compiler
  - [ ] Bytecode
  - [ ] Stack-based VM

## Usage
Requires [Rust](https://www.rust-lang.org/) to be installed

### REPL
```bash
$ cargo run --release
>> let add = fn(x, y) { x + y };
null
>> add(1, 2)
3
```
### File Loading
```bash
$ cargo run --release -- examples/fibonacci.monkey
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]...
```

##