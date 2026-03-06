use owo_colors::OwoColorize;
use std::env;
use std::fs::{DirEntry, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

fn get_permissions_substring(char_to_match: char) -> String {
    match char_to_match {
        '1' => return String::from("--x"),
        '2' => return String::from("-w-"),
        '3' => return String::from("-wx"),
        '4' => return String::from("r--"),
        '5' => return String::from("r-x"),
        '6' => return String::from("rw-"),
        '7' => return String::from("rwx"),
        _ => return String::from("---"),
    }
}

fn human_readable_permissions(entry: &DirEntry, perms: Permissions) -> io::Result<String> {
    let mut permissions = String::new();
    let file_type = entry.file_type()?;

    let mut f_type = "-";
    if file_type.is_dir() {
        f_type = "d";
    }
    if file_type.is_symlink() {
        f_type = "l";
    }
    permissions.replace_range(0..0, f_type);

    // Set owner, group, and other permissions
    let mode = format!("{:o}", perms.mode() & 0o777);
    for (_index, character) in mode.chars().enumerate() {
        let perms = get_permissions_substring(character);
        permissions.push_str(&perms);
    }

    return Ok(permissions);
}

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
        let metadata = entry.metadata()?;
        let permissions = human_readable_permissions(&entry, metadata.permissions())?;

        let entry_path = entry.path();
        if entry_path.is_dir() {
            println!(
                "{} {} is a directory",
                permissions,
                entry.file_name().display().bright_blue()
            );
        } else {
            println!("{} {} is a file", permissions, entry.file_name().display());
        }
    }

    Ok(())
}
