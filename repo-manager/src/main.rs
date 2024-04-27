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
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets json format bitbucket pull request")
                .takes_value(true),
        )
        .get_matches();

    // config 값이 전달되었는지 확인
    if let Some(c) = matches.value_of("input_file") {
        println!("Value for input_file: {}", c);
    } else {
        // print all usage
        println!("{}", matches.usage());
    }
}
