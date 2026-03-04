use owo_colors::OwoColorize;
use std::env;
use std::path::Path;
use std::{fs, io};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut path_str = String::from(".");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let first_arg = &args[1];
        println!("The first argument is: {}", first_arg);
        path_str.clear(); 
        path_str.push_str(first_arg);
    }

    let path = Path::new(&path_str);

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;

        let entry_path = entry.path();

        if entry_path.is_dir() {
            println!(
                "{} is a directory",
                entry.file_name().display().bright_blue()
            );
        } else {
            println!("{} is a file", entry.file_name().display());
        }
    }

    Ok(())
}
