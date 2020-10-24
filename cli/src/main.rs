use clap::{App, Arg};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

mod idkrequest;

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

    let input = matches.value_of("INPUT").unwrap();

    let command = match idkrequest::make_request(input) {
        Ok(cmd) => cmd,
        Err(error) => panic!("{:?}", error),
    };

    println!("{}", command);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match ctx.set_contents(command) {
        Err(error) => println!("{:?}", error),
        _ => (),
    };
}
