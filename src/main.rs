extern crate clap;
extern crate memorable;

use clap::{Arg, App, SubCommand};
use memorable::db;
use memorable::handlers;
use memorable::server;
use std::io;
use std::io::Write;
use std::process;

// This seems to be designed to be glob imported.

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("shorten")
                    .about("Shorten a new url")
                    .arg(Arg::with_name("url")
                         .required(true)
                         .help("The url to shorten so that it is memorable."))
                    .arg(Arg::with_name("title")
                         .long("title")
                         .short("t")
                         .value_name("TITLE")
                         .help("Title for the link."))
                    .arg(Arg::with_name("custom-key")
                         .long("custom-key")
                         .value_name("CUSTOM KEY")
                         .short("c")
                         .help("Use a custom shortend name instead of an \
                                autogenrated one.")))
        .subcommand(SubCommand::with_name("lookup")
                    .about("Look up an already shortened link.")
                    .arg(Arg::with_name("short-link")
                         .required(true)
                         .help("The shortend link to resolve.")))
        .subcommand(SubCommand::with_name("server")
                    .about("Start the URL shortner server."));
    let matches = app.clone().get_matches();
    let mut exit_status = 0;

    if let Some(matches) = matches.subcommand_matches("shorten") {
        let connection = db::establish_connection();
        let link = handlers::links::create_link(&connection,
                                                matches.value_of("url").unwrap().to_owned(),
                                                matches.value_of("custom-key").map(str::to_owned),
                                                matches.value_of("title").map(str::to_owned));
        match link {
            Ok(link) => {
                println!("{}", link);
            },
            Err(e) => {
                eprintln!("Error shortning link: {:?}", e);
                exit_status = 1;
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("lookup") {
        let connection = db::establish_connection();
        let short_link = matches.value_of("short-link")
            .unwrap()
            .to_owned();
        match handlers::links::get_link(&connection, short_link) {
            Ok(link) => {
                println!("{}", link);
            }
            Err(e) => {
                eprintln!("Error looking up link: {:?}", e);
                exit_status = 1;
            }
        };
    } else if let Some(_) = matches.subcommand_matches("server") {
        server::run();
    } else {
        let mut err = io::stderr();
        app.write_help(&mut err).expect("Failed to write help to STDERR");
        err.write(b"\n").expect("Failed to write help to STDERR");
        exit_status = 1;
    }

    process::exit(exit_status);
}
