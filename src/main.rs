extern crate clap;
extern crate memorable;

use clap::{Arg, App, SubCommand};
use memorable::db;
use memorable::handlers;
use std::process;

// This seems to be designed to be glob imported.

fn main() {
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("shorten")
                    .about("Shorten a new link")
                    .arg(Arg::with_name("link")
                         .required(true)
                         .help("The link to shorten so that it is memorable."))
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
                         .help("The shortend link to resolve.")));
    let matches = app.clone().get_matches();

    // Clap gave us matches, connect to the DB.
    let connection = db::establish_connection();

    if let Some(matches) = matches.subcommand_matches("shorten") {
        let link = handlers::links::create_link(&connection,
                                                matches.value_of("link").unwrap(),
                                                matches.value_of("custom-key"),
                                                matches.value_of("title"));
        println!("Link: {:?}", link.unwrap());
    } else if let Some(matches) = matches.subcommand_matches("lookup") {
        let short_link = matches.value_of("short-link").unwrap();
        println!("Link: {:?}", handlers::links::get_link(&connection,
                                                         short_link));
    } else {
        app.print_help().unwrap();
        process::exit(1);
    }
}
