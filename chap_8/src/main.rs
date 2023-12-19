use std::fs::{self, File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::net::IpAddr;

fn main() {
    a();

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Unwrap
    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => 
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            }  
    };

    //Closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });





}

fn a() {
    b();
}

fn b() {
    c(32);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't use 22!");
    }
    println!("{}", 100.0 / num as f32);
}








fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Result::Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}



fn ipAdd() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}