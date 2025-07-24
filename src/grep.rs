use clap::Parser;
use std::io::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    text: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    let res = grep(args.text.as_str(), args.pattern.as_str()).unwrap_or_else(|_| {
        println!("Pattern not found in text.");
        Vec::new()
    });
    println!("Found matches {:?}", res);
}

fn grep<'a>(text: &'a str, search_text: &'a str) -> Result<Vec<&'a str>, Error> {
    if text.is_empty() {
        let err = Error::new(
            std::io::ErrorKind::DirectoryNotEmpty,
            "Text to search is empty.",
        );
        return Err(err);
    }

    if search_text.is_empty() {
        let err = Error::new(std::io::ErrorKind::DirectoryNotEmpty, "Pattern is empty.");
        return Err(err);
    }

    let lines: Vec<&str> = text.split("\n").collect();
    for line in &lines {
        if line.contains(search_text) {
            println!("{}", line);
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grep() {
        let text = "fn simple_grep()";
        let result = grep("fn simple_grep()", text);
        assert!(result.is_ok());
    }

    #[test]
    fn complex_grep() {
        let text = "a";
        let result = grep("a\na\na\na\nb\na\nb\na", text);
        assert!(result.is_ok());
    }
}
