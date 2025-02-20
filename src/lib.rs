use std::fs::{self, File, ReadDir};
use std::io::{prelude::*, Error};
use std::path::Path;

fn echo(text: &str) {
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

fn find() {
    todo!();
}

fn grep() {
    todo!();
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
    fn gigabyte_cat() {
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
}
