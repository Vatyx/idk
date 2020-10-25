use clap::{App, Arg};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;

mod idkrequest;
mod authserver;

const CONFIG_FILE: &str = "idk.config";

const LOGIN_PAGE: &str = "https://app.idkcli.com/?ref=cli";

fn get_config_path() -> std::io::Result<PathBuf> {
    Ok(std::env::current_exe()?.parent().unwrap().join(CONFIG_FILE))
}

fn load_config() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let content: String = fs::read_to_string(get_config_path()?)?;
    return Ok(content);
}

fn save_config(config: &String) {
    let mut file = fs::File::create(get_config_path().unwrap()).unwrap();
    file.write_all(config.as_bytes()).unwrap();
}

fn make_backend_request(input: &String, access_token: &String) {
    let command = match idkrequest::make_request(&input, access_token) {
        Ok(cmd) => cmd,
        Err(error) => panic!("{:?}", error),
    };

    println!("{}", command);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(command).unwrap();
}

fn do_login() -> String {
    Command::new("open").arg(LOGIN_PAGE).output().ok();

    println!("Waiting for login...");

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let token = rt.block_on(authserver::start_server());

    save_config(&token);

    println!("Successfully logged in! Try asking for a command.");

    return token;
}

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
        do_login();
        return;
    }

    let text = matches.values_of("INPUT");
    if let None = text {
        println!("No input provided!");
        return;
    }

    let inputs: Vec<_> = text.unwrap().collect();

    let input = inputs.join(" ");
    // println!("{}", input);

    if let Ok(access_token) = load_config() {
        make_backend_request(&input, &access_token);
    }
    else {
        println!("You need to be logged in to use idk.");
    }
}
