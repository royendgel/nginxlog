extern crate postgres;
extern crate getopts;
extern crate regex;
use postgres::{Connection, SslMode};

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::path::Path;
use std::env;

use regex::Regex;

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

    // let db_res = db("wepa").execute("\\dt;", &[]);
    // println!("{:?}", db_res);
    //
    //
    // println!("Started !!");
    // // Test



}

// Reads log file line by line
fn parse_log_file_by_line(path: &Path) -> Result<(), io::Error> {
    // Opens the log file and read it, what then ?
    println!("{:?}", path);
    let file = try!(File::open(path));

    let file = BufReader::new(&file);

    for line in file.lines() {
        extract_line(line.unwrap().as_str());
    }

    Ok(())
}


// Returns a db connection which we will use to store the log data
fn db(connection: &'static str) -> postgres::Connection {

    let conn = Connection::connect(connection, SslMode::None);
    conn.unwrap()
}

// processes line , maybe this function should call db ??
fn extract_line(line: &str) -> &str {
    // I'm depending on a space FIXME !!
    let re =
        Regex::new(r#"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) - - \[(.*)\] "(POST|GET) (.+)(HTTP.*?").(\d+).(\d+) (.*)"#)
            .unwrap();
    // println!("{:?}", re);
    for cap in re.captures_iter(line) {
        // println!("{:?}", line);
        // println!("IP: {} Date: {} method: {} url: {}",
        println!("IP: {} Date: {} method: {} url: {} request: {} status: {} body_bytes_sent: {} \
                  http_referer: {} http_user_agent: {}",
                 cap.at(1).unwrap_or(""),
                 cap.at(2).unwrap_or(""),
                 cap.at(3).unwrap_or(""),
                 cap.at(4).unwrap_or(""),
                 cap.at(5).unwrap_or(""),
                 cap.at(6).unwrap_or(""),
                 cap.at(7).unwrap_or(""),
                 cap.at(8).unwrap_or(""),
                 cap.at(9).unwrap_or(""));
    }

    "re.as_str()"

}
