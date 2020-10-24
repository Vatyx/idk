use clap::{App, Arg};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

mod idkrequest;

fn main() {
    let matches = App::new("idk cli")
                        .version("1.0")
                        .about("Generate commands you don't know.")
                        .arg(Arg::with_name("INPUT")
                            .multiple(true))
                        .subcommand(App::new("login")
                            .about("Log into idk"))
                        .get_matches();

    if matches.is_present("login") {
        println!("You're trying to log in");
        return;
    }

    let inputs: Vec<_> = matches.values_of("INPUT").unwrap().collect();
    let input = inputs.join(" ");

    let command = match idkrequest::make_request(&input) {
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
