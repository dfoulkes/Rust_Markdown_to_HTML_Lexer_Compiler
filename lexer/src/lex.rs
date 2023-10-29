pub mod markdown_lexer {
    use crate::lex::tokens::Token;
    use crate::lex::lexer_factory::LexerFactory;

    pub fn parse(contents: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let matchers = LexerFactory::get_lexers();
        for line in contents.lines() {
            for matcher in matchers.iter() {
                //print!("Using matcher against line {}:", line);
                if matcher.validate(&line.to_string()) {
                    let token = matcher.get_token(&line.to_string());
                    tokens.push(token);
                    break;
                }
            }
        }
        return tokens;
    }
}

pub mod lexer_factory {
    use crate::lex::tokens::TokenMatcher;
    use crate::markdown::body::BodyMatcher;
    use crate::markdown::headers::HeaderMatcher;

    pub struct LexerFactory;

    impl LexerFactory {
        pub fn get_lexers() -> Vec<Box<dyn TokenMatcher>> {
            // const TOKEN_MATCHER: dyn TokenMatcher = HeaderMatcher;
            let lexers: Vec<Box<dyn TokenMatcher>> = vec![Box::new(HeaderMatcher),
                                                          Box::new(BodyMatcher),
            ];
            return lexers;
        }
    }
}

pub mod common {
    // pub enum Type {
    //     HeaderH1,
    //     HeaderH2,
    //     Unknown,
    //     Paragraph,
    //     NewLine,
    // }
    //
    // pub struct Token {
    //     pub token_type: Type,
    //     pub token_value: String,
    // }
    //
    // pub trait TokenMatcher {
    //     fn validate(&self, line: &String) -> bool;
    //     fn get_token(&self, line: &String) -> Token;
    // }
}

pub mod tokens {
    use std::fmt;
    // use crate::lex::common::Token;
    // use crate::lex::common::Type;

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.token_value)
        }
    }

    pub enum Type {
        HeaderH1,
        HeaderH2,
        Unknown,
        Paragraph,
        NewLine,
    }

    pub struct Token {
        pub token_type: Type,
        pub token_value: String,
    }

    pub trait TokenMatcher {
        fn validate(&self, line: &String) -> bool;
        fn get_token(&self, line: &String) -> Token;
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Type::HeaderH1 => write!(f, "H1"),
                Type::HeaderH2 => write!(f, "H2"),
                Type::Unknown => write!(f, "Unknown"),
                Type::Paragraph => write!(f, "Paragraph"),
                Type::NewLine => write!(f, "NewLine")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::lex::tokens::{Token, TokenMatcher};
    use crate::lex::lexer_factory;

    #[test]
    fn test_that_lexer_factory_returns_a_list_of_token_matchers() {
        let matchers = lexer_factory::LexerFactory::get_lexers();
        assert_eq!(matchers.len(), 2);
    }

    #[test]
    fn test_that_checks_if_a_line_is_a_header() {
        let matchers = lexer_factory::LexerFactory::get_lexers();
        let mut passed = false;
        for matcher in matchers {
            let markdown_text = "# Hello World";
            passed = matcher.validate(&String::from(markdown_text));
            if passed {
                let token = matcher.get_token(&String::from(markdown_text));
                assert_eq!(token.token_type.to_string(), lex::tokens::Type::HeaderH1.to_string());
                break;
            }
        }
        assert_eq!(passed, true);
    }

    #[test]
    fn test_should_return_multi_paragraph_with_line_break_token() {
        let matchers = lexer_factory::LexerFactory::get_lexers();
        let lines: Vec<String> = vec![String::from("There is a theory which states that if ever anyone discovers exactly what the Universe is for and why it is here, it will instantly disappear and be replaced by something even more bizarre and inexplicable.  \n"),
                                      String::from(" \n"),
                                      String::from("There is another theory which states that this has already happened.  \n")];
        let mut tokens: Vec<Token> = vec![];
        parse_lines(matchers, lines, &mut tokens);
    }

    fn parse_lines(matchers: Vec<Box<dyn TokenMatcher>>, lines: Vec<String>, tokens: &mut Vec<Token>) {
        for line in lines.iter() {
            for matcher in matchers.iter() {
                let passed = matcher.validate(&String::from(line));
                if passed {
                    let token = matcher.get_token(&String::from(line));
                    tokens.push(token);
                }
            }
        }
    }

    #[test]
    fn test_detect_multiline_paragraph() {
        let matchers = lexer_factory::LexerFactory::get_lexers();
        let lines: Vec<String> = vec![String::from("Hello \n"), String::from("World \n")];
        let mut tokens: Vec<Token> = vec![];
        parse_lines(matchers, lines, &mut tokens);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap().token_type.to_string(), lex::tokens::Type::Paragraph.to_string());
        assert_eq!(tokens.get(1).unwrap().token_type.to_string(), lex::tokens::Type::Paragraph.to_string());
    }

    #[test]
    fn test_parse_should_produce_a_valid_ast() {
        let contents = String::from("# Rust - Markdown to HTML Converter
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
- Learn how to use the Rust standard library.");

        let tokens = lex::markdown_lexer::parse(contents);
        assert_eq!(tokens.len(), 26);
        assert_eq!(tokens.get(0).unwrap().token_type.to_string(), lex::tokens::Type::HeaderH1.to_string());
        assert_eq!(tokens.get(1).unwrap().token_type.to_string(), lex::tokens::Type::Paragraph.to_string());
        assert_eq!(tokens.get(2).unwrap().token_type.to_string(), lex::tokens::Type::NewLine.to_string());
        assert_eq!(tokens.get(3).unwrap().token_type.to_string(), lex::tokens::Type::HeaderH2.to_string());
        assert_eq!(tokens.get(4).unwrap().token_type.to_string(), lex::tokens::Type::NewLine.to_string());
        assert_eq!(tokens.get(5).unwrap().token_type.to_string(), lex::tokens::Type::Paragraph.to_string());
    }
}
