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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_data() {
        let lhs = pull_user_data();
        let lhs = lhs["testuser"]["hash"].clone();

        let rhs = "$2b$09$PyndVwMZKX9qYUGn/tXHv.Wfi62r.7Dra7j3WA21wlIyfTVROYPFG";

        assert_eq!(lhs, rhs);
    }
}
