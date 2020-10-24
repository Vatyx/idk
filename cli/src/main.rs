
mod idkrequest;

use clap::{App, Arg};

fn main() {
    let matches = App::new("idk cli")
                        .version("1.0")
                        .about("Generate commands you don't know.")
                        .arg(Arg::with_name("INPUT")
                            .required(true)
                            .index(1))
                        .subcommand(App::new("login")
                            .about("Log into idk"))
                        .get_matches();

    println!("{}", idkrequest::make_request(matches.value_of("INPUT").unwrap()));
}
