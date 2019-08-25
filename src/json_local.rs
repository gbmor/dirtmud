//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    fs,
};

use json;

const USER_DATA_PATH: &str = "assets/data/users.json";

pub fn pull_user_data() -> json::JsonValue {
    match fs::read_to_string(USER_DATA_PATH) {
        Ok(data) => {
            json::parse(&data).unwrap()
        }
        Err(err) => panic!("{}", err),
    }
}
