use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Calender {
    pub name:  String,
    pub desc:  Option<String>,

    pub event: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub desc: Option<String>,

    pub time: Option<Datetime>,
    pub date: Option<Datetime>,
    pub reps: Option<Repeat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repeat {
    pub lo:   Option<Datetime>,
    pub hi:   Option<Datetime>,
    pub days: Option<String>,
}
