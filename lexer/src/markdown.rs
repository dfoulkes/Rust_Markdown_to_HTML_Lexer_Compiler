pub mod headers {
    use crate::lex::common::{Token, Type, TokenMatcher};

    pub struct HeaderMatcher;

    impl TokenMatcher for HeaderMatcher {
        fn validate(&self, line: &String) -> bool {
            if line.trim().starts_with("##") {
                return true;
            } else if line.trim().starts_with("#") {
                return true;
            }
            return false;
        }
        fn get_token(&self, line: &String) -> Token {
            return if line.trim().starts_with("##") {
                parse_h2(line)
            } else if line.trim().starts_with("#") {
                parse_h1(line)
            } else {
                Token {
                    token_type: Type::Unknown,
                    token_value: format!("{}", ""),
                }
            };
        }
    }

    fn parse_h1(p0: &String) -> Token {
        let mut p1 = p0.replace("#", "");
        p1 = p1.trim().to_string();
        return Token {
            token_type: Type::HeaderH1,
            token_value: format!("{}", p1),
        };
        //return format!("<H1>{}</H1>", p1);
    }

    fn parse_h2(p0: &String) -> Token {
        let mut p1 = p0.replace("##", "");
        p1 = p1.trim().to_string();
        return Token {
            token_type: Type::HeaderH2,
            token_value: format!("{}", p1),
        };
    }
}

pub mod body {
    use crate::lex::common::{Token, TokenMatcher, Type};

    pub struct BodyMatcher;

    impl TokenMatcher for BodyMatcher {
        fn validate(&self, line: &String) -> bool {
            return is_the_line_empty(line) || is_there_text_on_this_line(line);
        }
        fn get_token(&self, line: &String) -> Token {
            return if is_the_line_empty(line) {
                Token {
                    token_type: Type::NewLine,
                    token_value: format!("{}", ""),
                }
            } else {
                Token {
                    token_type: Type::Paragraph,
                    token_value: format!("{}", line),
                }
            };
        }
    }

    fn is_the_line_empty(line: &String) -> bool {
        return line.trim().is_empty();
    }

    fn is_there_text_on_this_line(line: &String) -> bool {
        return !line.trim().is_empty();
    }
}

// ## USE CASES TO COVER:
// 1. Should return <H1>Test</H1> for #Test
// 2. Should return <H2>Test</H2> for ##Test
// 3. Should return <H1>Test</H1> even when there's white space between line start and first char
// 4. Should return <H2>Test</H2> even when there's white space between line start and first char
// 5. Should return UNKNOWN when line ends with newline
// 6. Should return UNKNOWN when there's only a line ending
#[cfg(test)]
mod tests {
    use crate::lex::common::{Type, TokenMatcher};
    use crate::markdown::body::BodyMatcher;
    use crate::markdown::headers::HeaderMatcher;

    #[test]
    fn should_return_h1_tag_for_test() {
        let test: String = String::from("#Test");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::HeaderH1.to_string());
    }

    #[test]
    fn should_return_h2_tag_for_test() {
        let test: String = String::from("##Test");
        let header_matcher = HeaderMatcher;
        let passed = header_matcher.validate(&test);
        let token = header_matcher.get_token(&test);
        assert!(passed);
        assert_eq!(token.token_type.to_string(), Type::HeaderH2.to_string());
    }

    #[test]
    fn should_return_h1_even_when_theres_white_space_between_line_start_and_first_char() {
        let test: String = String::from(" #Test");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::HeaderH1.to_string());
    }

    #[test]
    fn should_return_h2_even_when_theres_white_space_between_line_start_and_first_char() {
        let test: String = String::from(" ##Test");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::HeaderH2.to_string());
    }

    #[test]
    fn should_return_paragraph_when_line_ends_with_newline() {
        let test: String = String::from("Test\n");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::Unknown.to_string());
    }

    #[test]
    fn should_return_new_line_when_theres_only_a_line_ending() {
        let test: String = String::from("\n");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::Unknown.to_string());
    }

    #[test]
    fn should_return_unknown_when_line_is_empty() {
        let test: String = String::from("");
        let header_matcher = HeaderMatcher;
        let result = header_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::Unknown.to_string());
    }

    #[test]
    fn test_should_return_single_line_text_as_paragraph_token() {
        let test: String = String::from("Hello World");
        let body_matcher = BodyMatcher;
        let result = body_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::Paragraph.to_string());
        assert_eq!(result.token_value, test);
    }

    #[test]
    fn test_should_return_a_new_line_token_when_a_line_only_contains_an_line_return() {
        let test: String = String::from("\n");
        let body_matcher = BodyMatcher;
        let result = body_matcher.get_token(&test);
        assert_eq!(result.token_type.to_string(), Type::NewLine.to_string());
    }
}
