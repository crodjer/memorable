# Memorable

Memorable is a URL shortening service, written in Rust. It is based on:

 - [iron](https://github.com/iron/iron) web framework.
 - [diesel](http://diesel.rs) ORM.
 - [PostgreSQL](http://postgresql.org/)

## Setup
- Install PostgreSQL. In Debian Unstable:
  ```
  sudo apt install postgresql-10
  ```
- Create a database and its owner in PostgreSQL. Add `DATABASE_URL` to shell env
  or create a `.env ` file on this project's top level with the database
  credentials:
  ```
  DATABASE_URL=postgres://clerk:i-store@localhost/memorable
  ```
- Install diesel cli:
  ```
  cargo install diesel_cli --no-default-features --features postgres
  ```
- Setup DB
  ```
  diesel setup
  ```

## CLI Interface
For initial development, I ended up writing a CLI interface so that I could play
with `diesel` without integrating the web server (`iron`):

    $ cargo run -- --help
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target/debug/memorable --help`
    memorable 0.1.0
    Rohan Jain <crodjer@gmail.com>


    USAGE:
        memorable [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help       Prints this message or the help of the given subcommand(s)
        lookup     Look up an already shortened link.
        server     Start the URL shortner server.
        shorten    Shorten a new url

### Shorten a URL
Store the `rust` website with key `rust`:

    $ cargo run -- shorten http://rust-lang.org/ --title "The Rust Programming Language" -c rust
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
          Running `target/debug/memorable shorten 'http://rust-lang.org/' --title 'The Rust Programming Language' -c rust`
    http://rust-lang.org/	rust	The Rust Programming Language

Look it back up:

    $ cargo run -- lookup rust                                       ~/projects/memorable
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
          Running `target/debug/memorable lookup rust`
    http://rust-lang.org/	rust	The Rust Programming Language

Auto generated shortening key:

    $ cargo run -- shorten http://google.com/                        ~/projects/memorable
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
          Running `target/debug/memorable shorten 'http://google.com/'`
    http://google.com/	b7fkgoc

`b7fkgoc` is the shortened URL here.

## Server
I can't call this project a URL shortening service until we actually have a web
service. Start it with:

    $ cargo run -- server

Shorten a URL with:

    $ curl -X POST "localhost:3000/shorten?url=https://en.wikipedia.org/"
    http://localhost:3000/4reqrki

Now, when you open `http://localhost:3000/4reqrki`, the service will redirect
you to: `https://en.wikipedia.org/`.
