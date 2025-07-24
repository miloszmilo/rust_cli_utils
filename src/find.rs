use clap::Parser;
use std::{
    fs::{read_dir, DirEntry},
    io::Result,
    iter,
    path::Path,
};

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
    let res = find(args.text.as_str(), args.pattern.as_str()).unwrap_or_else(|_| {
        println!("File not found in path.");
        Box::new(iter::empty()) as Box<dyn Iterator<Item = Result<DirEntry>>>
    });
    for entry in res {
        match entry {
            Ok(e) => println!("Found file {:?}", e),
            Err(e) => println!("Error reading file {:?}", e),
        }
    }
}

fn find(
    search_path: &str,
    target_name: &str,
) -> Result<Box<dyn Iterator<Item = Result<DirEntry>>>> {
    // fn find(search_path: &str, target_name: &str) -> Result<ReadDir, Error> {
    let current_dir = Path::new(search_path);

    let paths = read_dir(current_dir)?;

    for dir in read_dir(current_dir).unwrap() {
        let found_dir = dir.unwrap().path();
        let name_of_found_dir = found_dir.to_string_lossy().to_owned();
        match name_of_found_dir == target_name {
            true => {
                println!("{}", name_of_found_dir);
            }
            false => {
                if found_dir.is_dir() {
                    let _ = find(&name_of_found_dir, target_name);
                } else {
                    return Ok(Box::new(paths));
                }
            }
        }
    }

    Ok(Box::new(paths))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_find() {
        let path = "lib.rs";
        let result = find("./", path);
        assert!(result.is_ok());
    }

    #[test]
    fn complex_find() {
        let path = "lib.rs";
        let result = find("/home/nxtperfect/", path);
        assert!(result.is_ok());
    }

    #[test]
    fn non_existing_find() {
        let path = "library.rs";
        let result = find("./", path);
        assert!(result.is_ok());
    }

    #[test]
    fn complex_non_existing_find() {
        let path = "library.rs.non.existant";
        let result = find("/home/nxtperfect/", path);
        assert!(result.is_ok());
    }
}
