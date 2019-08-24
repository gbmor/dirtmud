//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    io,
    sync::mpsc,
};

use chrono::prelude::*;

use log::{
    info,
};

mod clients;
mod engine;
mod logging;

fn main() -> Result<(), io::Error> {
    logging::init();

    let thetime = Utc::now();
    println!();
    println!("dirtmud 0.1-dev");

    info!("dirtmud 0.1-dev");
    info!("Startup at: {}", thetime.to_rfc2822());

    info!("Spawning engine worker");
    let (tx, rx) = mpsc::channel::<String>();
    engine::spawn_worker(rx);

    // This will block
    info!("Spawning client worker");
    clients::spawn_worker("0.0.0.0:56543", tx)?;

    Ok(())
}
