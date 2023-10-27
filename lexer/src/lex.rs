pub mod lexer_factory {
    use crate::lex::common::TokenMatcher;
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
}

pub mod tokens {
    use std::fmt;
    use crate::lex::common::Token;
    use crate::lex::common::Type;

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.token_value)
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Type::HeaderH1 => write!(f, "H1"),
                Type::HeaderH2 => write!(f, "H2"),
                Type::Unknown => write!(f, "UNKNOWN"),
                Type::Paragraph => write!(f, "PARAGRAPH"),
                Type::NewLine => write!(f, "NewLine")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::lex::common::Token;
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
                assert_eq!(token.token_type.to_string(), lex::common::Type::HeaderH1.to_string());
                break;
            }
        }
        assert_eq!(passed, true);
    }

    #[test]
    fn test_detect_multiline_paragraph() {
        let matchers = lexer_factory::LexerFactory::get_lexers();
        let lines: [String; 2] = [String::from("Hello \n"), String::from("World \n")];
        let mut tokens: Vec<Token> = vec![];

        for line in lines.iter() {
            for matcher in matchers.iter() {
                let passed = matcher.validate(&String::from(line));
                if passed {
                    let token = matcher.get_token(&String::from(line));
                    tokens.push(token);
                }
            }
        }
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap().token_type.to_string(), lex::common::Type::Paragraph.to_string());
        assert_eq!(tokens.get(1).unwrap().token_type.to_string(), lex::common::Type::Paragraph.to_string());
    }
}
