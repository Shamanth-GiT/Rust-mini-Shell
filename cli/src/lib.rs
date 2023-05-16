use std::fs; // used for file system functionalities
use std::io;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn create_file(args: &[&str]) -> (){
    let filename = &args[1];

    fs::File::create(filename)
            .expect("something went wrong with creating the file ");

    println!("File has been created with the name: {}", filename);   
}

pub fn read_file(args: &[&str]) -> (){
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    println!("{}", contents);
}

pub fn write_to_file(args: &[&str]) -> (){
    let filename: &&str = &args[1];
    let rs:bool = Path::new(filename).exists();

    println!("Please include the message you would like to write to {}", filename);
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();

    if rs == true{
        let mut file = fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("cannot open file");
        
        file.write(msg.as_bytes()).expect("failed to write to file");
    }
    else {
        let mut file = fs::File::create(filename)
            .expect("something went wrong with opening the file for writing");

        file.write(msg.as_bytes()).expect("writing failed");
    }
    println!("message appended to file");
}

pub fn over_write_content(args: &[&str]) -> (){
    let filename = &args[1];
    let rs:bool = Path::new(filename).exists();

    println!("Please include the message you would like to write to {}", filename);
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();

    println!("Please also include the index at which you would like to start writing");
    let num:u64;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    num = input.trim().parse().unwrap();
    
    if rs == true{
        let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
        
        file.seek(SeekFrom::Start(num)).unwrap();
        file.write(msg.as_bytes()).expect("failed to write to file");
    }
}

pub fn find_all_instances(args: &[&str]) -> (){
    let filename = &args[1];
    let word = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    let v: Vec<_> = contents.match_indices(word).collect();
    println!("{:?}, {} instances", v, v.len());
}

pub fn size(args: &[&str]) -> (){
    let filename = &args[1];
    let file = fs::File::open(filename)
        .expect("something went wrong when trying to open the file");
    let size = file.metadata().unwrap().len();

    println!("{} is {} bytes", filename, size);
}

pub fn remove_file(args: &[&str]) -> (){
    let filename = &args[1];
    fs::remove_file(filename).expect("something went wrong with deleting the file");
    println!("{} was removed.", filename);
}

pub fn man_page() -> (){
    println!("cargo run R <filename>: reads the contents of a file");
    println!("cargo run W <filename>: writes to a file (will create the file if it doesn't exist or append to a file if it does exist)");
    println!("cargo run C <filename>: creates a file");
    println!("cargo run find <filename> <query>: finds all instances of query word in file");
    println!("cargo run size <filename>: finds the size of a file");
    println!("cargo run rm <filename>: removes/deletes a file");
    println!("cargo run oW <filename>: overwrites contents of a file with user defined location");
    println!("cargo run man: overwrites contents of a file with user defined location");
}

/*
    (1) Read the input file.
    (2) Read the files with file names included in the input file.
    (3) Find word in files.
    (4) Print the word with indexes of files it is present in.
 */
pub fn finder_across_all(args: &[&str]) -> (){
    let filename = &args[1];
    let word = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    let mut i = 0;
    print!("{}: ", word);
    for file in contents.split_whitespace(){
        let in_contents = fs::read_to_string(file)
            .expect("something went wrong with opening the file");
        let v: Vec<_> = in_contents.match_indices(word).collect();
        if v.len() > 0 {
            print!("{} ", i);
        }
        i+=1;
    }
}