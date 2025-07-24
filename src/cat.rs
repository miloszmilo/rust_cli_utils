use clap::Parser;
use std::io::Read;
use std::{fs::File, io::Error, path::Path};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let res = cat(args.path.as_str()).unwrap_or_else(|_| {
        println!("File not found.");
        String::new()
    });
    println!("{:?}", res);
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
