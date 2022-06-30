// import App, Arg struct
use clap::{App, Arg};

fn main() {
    //create new clap app
    let matches = App::new("echor") //
        // add app info
        .version("0.1.0")
        .author("cito")
        .about("echo in rust")
        // create args
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    println!("{:#?}", matches);
}
