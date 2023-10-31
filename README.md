[![Coverage Status](https://coveralls.io/repos/github/dfoulkes/Rust_Markdown_to_HTML_Lexer_Compiler/badge.svg)](https://coveralls.io/github/dfoulkes/Rust_Markdown_to_HTML_Lexer_Compiler)
# Rust - Markdown to HTML Converter

---

## Overview

This is a simple markdown to html converter written in Rust. It is a learning project for me to get familiar with Rust.
The main reason I chose this project for rust is because it is a simple project that I have done in other languages and
Rust as a language is very well suited for this type of project due to the benefits of being a static language when it 
comes to tokenizing and parsing text.


## Structure

The project is broken up into 3 main crates:
- runner - This is the main entry point for the application. It is responsible for parsing the command line arguments.
- markdown - This is the core of the application. It is responsible for parsing the markdown into an [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
- compiler - This is responsible for taking the AST and converting it into HTML.


## Personal Objectives

- Learn Rust syntax.
- Learn how to structure a Rust project.
- Learn how to use the Rust compiler.
- Learn TDD in Rust.
- Learn how to use the Rust standard library.


# Pipeline Status
[![Build & Test](https://github.com/dfoulkes/Rust_Markdown_to_HTML_Lexer_Compiler/actions/workflows/build_test.yml/badge.svg)](https://github.com/dfoulkes/Rust_Markdown_to_HTML_Lexer_Compiler/actions/workflows/build_test.yml)