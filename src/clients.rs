//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    fs,
    io,
    io::{
        BufRead,
        BufReader,
        Write,
    },
    net::{
        TcpListener,
        TcpStream,
    },
    sync::mpsc,
    thread,
};

use log::{
    error,
    info,
};

use zeroize::Zeroize;

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
    let rdr = strm.try_clone().unwrap();
    let mut rdr = BufReader::new(rdr);

    let greetz = fs::read_to_string("assets/greet.txt")
        .unwrap_or_else(|err| {
            error!("{}", err);
            panic!("{}", err);
        });

    strm.write_all(&greetz.into_bytes())?;
    
    let prompt = "Username: ".to_string();
    strm.write_all(&prompt.into_bytes())?;
    let mut user = String::new();
    rdr.read_line(&mut user)?;
    user = user.trim().to_string();

    let prompt = "Password: ".to_string();
    strm.write_all(&prompt.into_bytes())?;
    let mut pass = String::new();
    rdr.read_line(&mut pass)?;
    pass = pass.trim().to_string();
    
    let output = format!("\nYour user: {}\nYour pass: {}\n", user, pass);
    let mut pass_b = pass.into_bytes();
    pass_b.zeroize();
    strm.write_all(&output.into_bytes())?;

    Ok(())
}
