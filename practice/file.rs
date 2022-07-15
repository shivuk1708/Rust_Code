//use std::fs::File;
//use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::env;
use std::path::PathBuf;

fn read_file() -> u32 {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("./foo.txt")
        .unwrap();
    let mut str1 = String::new();
    file.read_to_string(&mut str1).expect("failed to read");
    return if str1.is_empty() {
        1
    } else {
        1 + str1.parse::<u32>().unwrap()
    };
}
fn write_file(count : u32) {
    let mut cur_config_dir = std::env::current_dir().unwrap_or(PathBuf::from("."));
    cur_config_dir.push("foo.txt");

    println!("{:?}", cur_config_dir);
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(cur_config_dir)
        .unwrap();

    write!(&file, "{}", count).expect("failed");
}
fn main ()
{
    write_file(read_file());
}