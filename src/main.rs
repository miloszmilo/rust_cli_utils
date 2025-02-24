use clap::{Parser, Subcommand};
use std::fs::{self, File, ReadDir};
use std::io::{prelude::*, Error};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    function: String,

    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    match args.function {
        // for each match run function
        // if missing param, send err
        _ => todo!(),
    }
    // let matches = App::new("cli_utils").get_matches();
}

fn echo(text: &str) {
    assert!(text.len() > 0);
    println!("{text}");
}

fn echo_from_vec(text: Vec<&str>) {
    for t in text {
        println!("{t}");
    }
}

fn cat(path: &str) -> Result<String, Error> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        let err = Error::new(std::io::ErrorKind::NotFound, "File doesn't exist.");
        return Err(err);
    }

    if file_path.is_dir() {
        let err = Error::new(
            std::io::ErrorKind::IsADirectory,
            "Expected file path, got directory path.",
        );
        return Err(err);
    }

    if file_path.has_root() {
        let err = Error::new(
            std::io::ErrorKind::PermissionDenied,
            "No permission to open root file.",
        );
        return Err(err);
    }

    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{content}");
    Ok(content)
}

fn ls(path: &str) -> Result<ReadDir, Error> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        let err = Error::new(std::io::ErrorKind::NotFound, "File doesn't exist.");
        return Err(err);
    }

    if !file_path.is_dir() {
        let err = Error::new(
            std::io::ErrorKind::NotADirectory,
            "Expected directory, got file path.",
        );
        panic!("{err}");
    }
    let paths = fs::read_dir(file_path).unwrap();

    for dir in fs::read_dir(file_path).unwrap() {
        println!("{}", dir.unwrap().path().display());
    }
    Ok(paths)
}

fn find(search_path: &str, target_name: &str) -> Result<ReadDir, Error> {
    let current_dir = Path::new(search_path);

    let paths = fs::read_dir(current_dir).unwrap();

    for dir in fs::read_dir(current_dir).unwrap() {
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
                    return Ok(paths);
                }
            }
        }
    }

    Ok(paths)
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
    fn simple_echo_test() {
        echo("Hello, World!");
    }
    #[test]
    fn long_echo_test() {
        echo("Hello, World! 1edfc4f8-717f-4207-bc09-4c8c71dac8e2 24c16597-cb3e-474b-8983-0dbd1a453ff6 c03f0c49-b5fb-4f6b-8a98-1ad64cc52180 0b05130f-c164-46d1-a9ad-0075b69a3571 c10a2589-f5b6-4928-bbe0-0d455459f106 0de8191c-e775-46d5-9a71-4bf4b25caa01");
    }
    #[test]
    fn string_vec_echo_test() {
        echo_from_vec(vec!["1", "2", "3"]);
    }
    #[test]
    fn five_consecutive_echo_test() {
        for num in 1..5 {
            echo(format!("Hello, World! {}", num).as_str());
        }
    }

    #[test]
    fn simple_cat() {
        let result = cat("./test_files/hello.txt").unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn empty_path_cat() {
        let result = cat("");
        assert!(result.is_err());
    }

    #[test]
    fn directory_cat() {
        let result = cat("./test_files");
        assert!(result.is_err());
    }

    #[test]
    fn no_permission_cat() {
        let result = cat("/bin/7z");
        assert!(result.is_err());
    }

    #[test]
    fn ten_mb_cat() {
        let path = "./test_files/10mb.txt";
        let result = cat(path).unwrap();
        let mut expected = String::new();
        let _ = File::open(path).unwrap().read_to_string(&mut expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn bench_gigabyte_cat() {
        let path = "./test_files/1gb.txt";
        let result = cat(path).unwrap();
        let mut expected = String::new();
        let _ = File::open(path).unwrap().read_to_string(&mut expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_ls() {
        let path = "./";
        let mut expected: Vec<String> = fs::read_dir(path)
            .unwrap()
            .map(|dir| dir.unwrap().path().display().to_string())
            .collect();
        let mut res_dirs: Vec<String> = ls(path)
            .unwrap()
            .map(|dir| dir.unwrap().path().display().to_string())
            .collect();
        res_dirs.sort();
        expected.sort();
        assert_eq!(res_dirs, expected);
    }

    #[test]
    fn non_existing_ls() {
        let path = "./does/not/exist/path";
        let result = ls(path);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        }
    }

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
