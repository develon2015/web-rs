use serde::{Deserialize, Serialize};

pub const CONFIG: &str = "assets/config.json";

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Item {
    ws: String,
    listen: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Config {
    pub port: u16,
    pub item: Vec<Item>,
}