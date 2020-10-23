// use quicli::prelude::*;
// use structopt::Structop;

mod idkrequest;

// #[derive(StructOpt)]
// struct Cli {
//     request: String,
// }

fn main() {
    println!("{:?}", idkrequest::make_request("test"))
}