use csv::{Error as CsvError, ReaderBuilder, Terminator, WriterBuilder};
use colored::*;

use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

type Store = BTreeMap<String, String>;

const BUFFER_SIZE: usize = 1024;
const DEFAULT_TCP_ADDRESS: &str = "127.0.0.1:7474";

pub fn launch() -> () {
    let mut store: Store = Store::new();
    let args: Vec<String> = std::env::args().collect();
    let default_address = DEFAULT_TCP_ADDRESS.to_string();
    let address = args.get(1).unwrap_or(&default_address);

    match TcpListener::bind(address) {
        Ok(listener) => {
            println!("{} {} {}", "Running".blue(), "kavakava".green(), format!("@ {}", address).blue());
            for stream in listener.incoming() {
                handle_incoming(stream, &mut store)
            }
        }
        Err(err) => print_error("Could not launch kavakava", err)
    }
}

fn handle_incoming(stream: Result<TcpStream, std::io::Error>, store: &mut Store) {
    if let Ok(mut stream) = stream {
        let mut buffer = [0; BUFFER_SIZE];
        if let Ok(_) = stream.read(&mut buffer) {
            let arg_readable = String::from_utf8_lossy(&buffer[7..]);
            let mut args: Vec<&str> = arg_readable.split(";").collect();
            args.pop();

            let response = process_task(store, &buffer[..6], &args);

            if let Err(err) = write_response(&mut stream, response) {
                print_error("Error sending the response", err);
            }
        }
    }
}

fn print_error(msg: &str, err: std::io::Error) -> () {
    println!("ERROR {} {}", msg.bright_red(), err)
}

fn process_task(store: &mut Store, task: &[u8], args: &Vec<&str>) -> String {
    match (task, args.get(0), args.get(1)) {
        (b"import", Some(path), Some(delimiter)) =>
            handle_csv_result(read_csv(store, path, delimiter), "Error reading file"),
        (b"export", Some(path), Some(delimiter)) =>
            handle_csv_result(write_csv(store, path, delimiter), "Error writing file"),
        (b"update", _, _) => update(store, args),
        (b"delete", _, _) => delete(store, args),
        (b"length", _, _) => format!("{}", store.len()),
        (b"bykeys", _, _) => format!("{:?}", by_keys(store, &args)),
        (b"byvals", _, _) => format!("{:?}", by_values(store, &args)),
        _ => String::from("Unknown task.")
    }
}

fn write_response(stream: &mut TcpStream, response: String) -> Result<(), std::io::Error> {
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn by_keys(store: &Store, keys: &Vec<&str>) -> Store {
    keys.iter().map(|key| {
        let key_string = key.to_string();
        let value = store.get(&key_string).unwrap_or(&String::default()).to_string();
        (key_string, value)
    }).collect()
}

fn by_values(store: &Store, values: &Vec<&str>) -> Store {
    store.iter()
        .filter(|(_, value)| values.contains(&value.as_str()))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

fn delete(store: &mut Store, args: &Vec<&str>) -> String {
    for key in args.iter() {
        store.remove(&key.to_string());
    }
    ok()
}

fn update(store: &mut Store, args: &Vec<&str>) -> String {
    for chunk in args.as_slice().chunks(2) {
        store.insert(chunk[0].trim().to_string(), chunk[1].trim().to_string());
    }
    ok()
}

fn ok() -> String {
    String::from("OK")
}

fn read_csv(store: &mut Store, path: &str, delimiter: &str) -> Result<(), CsvError> {
    let mut rdr =
        ReaderBuilder::new()
            .has_headers(false)
            .terminator(Terminator::CRLF)
            .delimiter(delimiter.as_bytes()[0])
            .from_path(path)?;

    for record in rdr.records() {
        let rec = record?;
        if let (Some(key), Some(value)) = (rec.get(0), rec.get(1)) {
            store.insert(key.to_string(), value.to_string());
        }
    }
    Ok(())
}

fn write_csv(store: &Store, path: &str, delimiter: &str) -> Result<(), CsvError> {
    let mut wtr =
        WriterBuilder::new()
            .has_headers(false)
            .terminator(Terminator::CRLF)
            .delimiter(delimiter.as_bytes()[0])
            .from_path(path)?;

    for (key, value) in store.iter() {
        wtr.write_record(&[key, value])?;
    }
    wtr.flush()?;

    Ok(())
}

fn handle_csv_result(res: Result<(), CsvError>, error_msg: &str) -> String {
    if res.is_ok() { ok() } else { error_msg.to_string() }
}
