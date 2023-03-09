use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error_operation();
        return;
    }
    let operation = &args[1];
    if operation.eq("set") {
        change_gender(args);
    }
    else if operation.eq("list") {
        read_gender(args);
    }
    else {
        error_operation();
        return;
    }

}
fn read_gender(args: Vec<String>) {
    if args.len() < 3 {
        error_read();
        return;
    }
    let user = &args[2];
    let filepath = format!("/home/{}/.gender", user);
    let path = Path::new(&filepath);
    let mut file = File::open(&path).expect("user no gender file");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents.clone()),
        Err(e) => Err(e),
    };
    println!("{}", contents);
}
fn change_gender(args: Vec<String>) {
    if args.len() < 4 {
        error_set();
        return;
    }
    let user = &args[2];
    let gender = &args[3];
    let path = &format!("/home/{}/.gender", user);
    println!("{}'s gender was set to: {}", user, gender);
    println!("{}", path);
    echo(gender, &Path::new(path)).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn error_operation() {
    println!("error: operation not found");
    println!("choose one of the following options");
    println!("set");
    println!("list");
}
fn error_read() {
    println!("error: user not provided");
}
fn error_set() {
    println!("error: does not contain user and/or gender");
    println!("example of how to run command: genderutils set bob female");
}
