use std::fs;
use std::{env, process::exit};

struct Arguments {
    directory_path: String,
    search_string: String,
    destination_path: String,
}

fn parse_args(args: &Vec<String>) -> Result<Arguments, String> {
    if args.len() == 3 {
        let arguments = Arguments {
            directory_path: args[0].clone(),
            search_string: args[1].clone(),
            destination_path: args[2].clone(),
        };

        return Ok(arguments);
    }
    Err("Usage: stringmatch <directory_path> <search_string> <destination_path>".to_string())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args = match parse_args(&args) {
        Ok(a) => a,
        Err(s) => {
            println!("{}", s);
            exit(0);
        }
    };

    let files = match fs::read_dir(args.directory_path) {
        Ok(f) => f,
        Err(_) => {
            println!(
                "Failed to read source directory. Are you sure you entered the correct path? kekw"
            );
            exit(0);
        }
    };

    for file in files {
        let f = match file {
            Ok(de) => de,
            Err(_) => {
                println!("Error with DirEntry");
                exit(0);
            }
        };
        let path = f.path();

        let name = match path.file_name() {
            Some(n) => n,
            None => {
                println!("something");
                exit(0);
            }
        };

        if name.to_string_lossy().contains(args.search_string.as_str()) {
            let cpy_path = args.destination_path.to_string() + "/" + &name.to_string_lossy();
            match fs::copy(&path, &cpy_path) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "Copy failed. Are you sure you entered the correct destination path? kekw"
                    );
                    exit(0);
                }
            };
        }
    }
}
