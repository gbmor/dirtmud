//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    fs, io,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};

use log::{error, info};
use zeroize::Zeroize;

use crate::auth;


pub fn spawn_worker(ip: &str, tx: mpsc::Sender<String>) -> Result<(), io::Error> {
    match TcpListener::bind(ip) {
        Ok(lstnr) => {
            for conn in lstnr.incoming() {
                match conn {
                    Ok(strm) => {
                        info!("New connection: {:?}", strm.peer_addr().unwrap());
                        let txc = tx.clone();
                        thread::spawn(move || greet(strm, txc).unwrap());
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Err(err) => return Err(err),
    }

    Ok(())
}

fn greet(mut strm: TcpStream, _engine: mpsc::Sender<String>) -> Result<(), io::Error> {
    let CLEAR_SCREEN: String = format!("{}[2J", 27 as char);

    loop {
        strm.write_all(CLEAR_SCREEN.clone().into_bytes().as_ref())?;
        strm.write_all(&pull_greet_asset().into_bytes())?;

        let rdr = strm.try_clone().unwrap();
        let mut rdr = BufReader::new(rdr);

        strm.write_all("\t\tSelection: ".to_string().into_bytes().as_ref())?;

        let mut selection = String::new();
        rdr.read_line(&mut selection)?;

        match selection.trim() {
            "1" => {
                login(&mut strm)?;
            }
            "2" => continue,
            "3" => return Ok(()),
            _ => {
                continue
            }
        }
    }
    Ok(())
}

fn pull_greet_asset() -> String {
    fs::read_to_string("assets/greet.txt").unwrap_or_else(|err| {
        error!("{}", err);
        panic!("{}", err);
    })
}

fn login(strm: &mut TcpStream) -> Result<(), io::Error> {
    let rdr = strm.try_clone().unwrap();
    let mut rdr = BufReader::new(rdr);

    let prompt = "Username: ".to_string();
    strm.write_all(prompt.into_bytes().as_ref())?;
    let mut user = String::new();
    rdr.read_line(&mut user)?;
    user = user.trim().to_string();

    let prompt = "Password: ".to_string();
    strm.write_all(&prompt.into_bytes())?;
    let mut pass = String::new();
    rdr.read_line(&mut pass)?;
    pass = pass.trim().to_string();

    let mut pass_b = pass.into_bytes();
    let auth = auth::user_pass(&user, &pass_b);
    pass_b.zeroize();

    match auth {
        true => strm.write_all("true".to_string().into_bytes().as_ref()),
        false => strm.write_all("false".to_string().into_bytes().as_ref()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let greet = pull_greet_asset();
        assert!(greet.contains("dirtMUD"));
    }
}
