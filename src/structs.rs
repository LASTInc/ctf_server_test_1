use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct StatusAns {
    pub status: i32,
}



#[derive(Serialize, Deserialize)]
pub struct TwitsModel {
    pub name: String,
    pub image: String,
    pub someText: String,
}


impl TwitsModel {
    pub fn new(
        name: &str,
        image: &str,
        someText: &str
    ) -> Self {
        TwitsModel {
            name: name.to_string(),
            image: image.to_string(),
            someText: someText.to_string()
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct UserModel {
    pub login: String,
    pub password: String,
}

impl UserModel {
    pub fn new(
        login: &str,
        password: &str
    ) -> Self {
        UserModel {
            login: login.to_string(),
            password: password.to_string()
        }
    }
}