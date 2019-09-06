//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{io, process, sync::mpsc, thread};

use chrono::prelude::*;
use clap;
use ctrlc;
use log::{info, warn};

mod auth;
mod clients;
mod engine;
mod json_local;
mod logging;

const VERS: &str = clap::crate_version!();

fn main() -> Result<(), io::Error> {
    logging::init();
    thread::spawn(auth::watch_userdata);

    ctrlc::set_handler(move || {
        warn!("^C / SIGINT caught. Exiting ...");
        process::exit(0);
    })
    .expect("Could not set up SIGINT handler.");

    let thetime = Utc::now();
    println!();
    println!("dirtmud v{}", VERS);

    info!("dirtmud v{}", VERS);
    info!("Startup at: {}", thetime.to_rfc2822());

    info!("Spawning engine worker");
    let (tx, rx) = mpsc::channel::<String>();
    engine::spawn_worker(rx);

    // This will block
    info!("Spawning client worker");
    clients::worker("0.0.0.0:56543", tx)?;

    Ok(())
}
