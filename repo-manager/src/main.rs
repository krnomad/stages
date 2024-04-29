use std::{io::Error, fs::File, io::Read};

use clap::{Arg, App};

fn main() {
    // let args: Vec<String> = std::env::args().skip(1).collect();
    // println!("{:?}", args)

    let matches = App::new("repo-manager")
        .version("0.1")
        .author("Jongwon kang")
        .about("repo helper command")
        .arg(
            Arg::with_name("input_file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets json format bitbucket pull request")
                .takes_value(true),
        )
        .get_matches();

    let file_name = match matches.value_of("input_file") {
        Some(f) => f,
        None => {
            eprintln!("Error: No input file specified");
            std::process::exit(1);
        }
    };    

    // Attempt to open the file
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    // Read the file contents into a string
    let mut contents = String::new();
    // read file contents into `contents`
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
    // print it
    println!("{}", contents);
}

fn read_file(file_name: &str) -> Result<String, Error> {
    let mut contents = String::new();

    // Attempt to open the file
    let mut file = File::open(file_name)?.read_to_string(&mut contents)?;
    
    // Read the file contents into a string
    
    Ok(contents)
}
