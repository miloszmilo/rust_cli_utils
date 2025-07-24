use clap::Parser;
use std::{
    fs::{read_dir, DirEntry},
    io::{Error, Result},
    iter,
    path::Path,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let res = ls(args.path.as_str()).unwrap_or_else(|_| {
        println!("No files found in path.");
        Box::new(iter::empty()) as Box<dyn Iterator<Item = Result<DirEntry>>>
    });
    for entry in res {
        match entry {
            Ok(e) => println!("Found file {:?}", e),
            Err(e) => println!("Error reading file {:?}", e),
        }
    }
}

fn ls(path: &str) -> Result<Box<dyn Iterator<Item = Result<DirEntry>>>> {
    // fn ls(path: &str) -> Result<ReadDir, Error> {
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
    let paths = read_dir(file_path)?;

    for dir in read_dir(file_path)? {
        println!("{}", dir.unwrap().path().display());
    }
    Ok(Box::new(paths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ls() {
        let path = "./";
        let mut expected: Vec<String> = read_dir(path)
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
}
