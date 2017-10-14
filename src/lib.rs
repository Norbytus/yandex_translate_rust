#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[path = "yandex_translate/client.rs"]
pub mod client;

#[path = "yandex_translate/answer.rs"]
pub mod answer;
