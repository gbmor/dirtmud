//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::sync::RwLock;

use bcrypt;
use inotify;
use lazy_static::lazy_static;
use log::{info, warn};

use crate::json_local;

struct Userdata {
    cache: RwLock<json::JsonValue>,
}

lazy_static! {
    static ref USERDATA: Userdata = Userdata {
        cache: RwLock::new(json_local::pull_user_data()),
    };
}

pub fn watch_userdata() {
    let mut inotify = inotify::Inotify::init().expect("Failed to spin up inotify");

    inotify
        .add_watch("assets/data/users.json", inotify::WatchMask::MODIFY)
        .expect("Failed to add userdata watcher");

    info!("Watching users.json for changes");

    let mut buffer = [0u8; 40960];

    loop {
        let evnts = inotify
            .read_events_blocking(&mut buffer)
            .expect("Can't read inotify events");

        for evnt in evnts {
            if evnt.mask.contains(inotify::EventMask::MODIFY) {
                info!("Userdata modified: reloading cache");
                let mut tmp = USERDATA.cache.write().unwrap();
                *tmp = json_local::pull_user_data();
            }
        }
    }
}

pub fn user_pass(user: &str, pass: &[u8]) -> bool {
    let stored_hash = USERDATA.cache.read().unwrap();
    let stored_hash = stored_hash[user]["hash"].to_string();

    match bcrypt::verify(&pass, &stored_hash) {
        Ok(resp) => {
            info!("User logged in: {}", user);
            return resp;
        }
        Err(_) => {
            warn!("Invalid auth for user: {}", user);
            return false;
        }
    }
}
