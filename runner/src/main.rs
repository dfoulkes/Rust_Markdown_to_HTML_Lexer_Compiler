use clap::Parser;
use lexer::lex::markdown_lexer::parse;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Params {
    // Path to the markdown file to parse.
    #[arg(short = 'f', long = "file", long_help = "The path to the markdown file to parse.")]
    path: String,
}

fn is_markdown_file(path: &str) -> Result<(), String> {
    if !path.ends_with(".md") {
        return Err(format!("Expected a markdown file, instead I have {}", path));
    }
    return Ok(());
}

fn main() {
    let args = Params::parse();
    // if not a markdown file then throw error
    is_markdown_file(&args.path).expect("Application ended due to:");
    let contents = file_as_string(&args.path);
    let tokens = parse(contents);
    for token in tokens.iter() {
        println!("Token: {}  Value: {}", token.token_type.to_string(), token.token_value);
    }
}

fn file_as_string(path: &str) -> String {
    return std::fs::read_to_string(path).expect("Something went wrong, expected to be able to read file contents");
}

#[cfg(test)]
mod test {

    use super::is_markdown_file;
    #[test]
    fn should_return_true_when_markdown_file() {
        let result = is_markdown_file("test.md");
        assert!(result.is_ok());
    }

    #[test]
    fn should_return_false_when_not_markdown_file() {
        let result = is_markdown_file("test.txt");
        assert!(result.is_err());
    }

    #[test]
    fn should_return_false_when_not_markdown_file_with_path() {
        let result = is_markdown_file("/home/dummy/test.txt");
        assert!(result.is_err());
    }

}
