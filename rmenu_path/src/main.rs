use std::{collections::HashSet, env, fs};

fn main() {
    for item in get_items() {
        println!("{}", item);
    }
}

fn get_items() -> HashSet<String> {
    env::var("PATH")
        .unwrap()
        .split(':')
        .flat_map(fs::read_dir)
        .flat_map(|read| {
            read.flatten()
                .map(|entry| entry.file_name().into_string().unwrap())
        })
        .collect::<HashSet<String>>()
}
