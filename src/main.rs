extern crate postgres;
extern crate getopts;
use postgres::{Connection, SslMode};

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::path::Path;
use std::env;

fn main() {

    let mut nginx_path = String::new();

    for (key, value) in env::vars() {
        if key == "nginx_path" {
            nginx_path = value;
            println!("The file path is {}", &nginx_path);
        }
    }
    println!("{:?}", nginx_path);
    let b: &Path = Path::new(&nginx_path);
    let res = parse_log_file_by_line(&b);
    // Check the result
    match res {
        Ok(_) => println!("Succesfully processed!"),
        _ => println!("An Error ocured! : {:?}", res),
    }

    let db_res = db("wepa").execute("\\dt;", &[]);
    println!("{:?}", db_res);


    println!("Started !!");



}

// Reads log file line by line
fn parse_log_file_by_line(path: &Path) -> Result<(), io::Error> {
    // Opens the log file and read it, what then ?
    println!("{:?}", path);
    let file = try!(File::open(path));


    let file = BufReader::new(&file);

    for line in file.lines() {
        println!("{:?}", line);
    }

    Ok(())
}


// Returns a db connection which we will use to store the log data
fn db(connection: &'static str) -> postgres::Connection {

    let conn = Connection::connect(connection, SslMode::None);
    conn.unwrap()
}
