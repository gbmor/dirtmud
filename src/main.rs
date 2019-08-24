//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    io,
    net::{
        TcpListener,
    },
    sync::mpsc,
    thread,
};

use chrono::prelude::*;

fn spawn_listener(ip: &str, tx: mpsc::Sender<String>) -> Result<(), io::Error> {
    match TcpListener::bind(ip) {
        Ok(lstnr) => {
            for conn in lstnr.incoming() {
                match conn {
                    Ok(strm) => {
                        strm.set_nonblocking(true)?;
                        let _txc = tx.clone();
                        thread::spawn(move || {
                            println!("Placeholder");
                        });
                    }
                    Err(err) => panic!("{}", err),
                }
            }
        }
        Err(err) => panic!("{}", err),
    }
    Ok(())
}

fn main() {
    let thetime = Utc::now();
    println!();
    println!("dirtmud 0.1-dev");
    println!("{}", thetime.to_rfc2822());

    let (tx, _rx) = mpsc::channel::<String>();
    spawn_listener("0.0.0.0:56543", tx).unwrap();
}
