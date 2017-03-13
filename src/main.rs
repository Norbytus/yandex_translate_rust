extern crate json;
extern crate hyper;
extern crate hyper_native_tls;

use std::io::Read;
use std::time::Duration;
use std::convert::Into;
use std::path::Path;
use std::fmt::Debug;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use json::JsonValue;

fn main() {

    /*let url: String = String::from("https://translate.yandex.net/api/v1.5/tr.json/translate?key=trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e&text=Welcome%20to%20the%20new%20world&lang=en-ru");

    let ssl = NativeTlsClient::new().unwrap();

    let connector = HttpsConnector::new(ssl);

    let mut client_value: Client = Client::with_connector(connector);

    let mut answer: String = String::new();

    let request = client_value
        .get(&url)
        .send()
        .unwrap()
        .read_to_string(&mut answer)
        .unwrap();

    let json_answer = json::parse(&answer).unwrap();
    for data in json_answer.entries() {
        match data {
            ("code", &JsonValue::Number(code)) => println!("{:?}", code),
            _ => println!("Error"),
        }
    }*/


    let mut translate = YandexTranslate::new()
        .set_apikey("trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e")
        .translate_from_to("Hello world", "en", "ru");

}

struct YandexTranslate {
    url: &'static str,
    api_key: String,
}

impl YandexTranslate {

    fn new() -> YandexTranslate {
        YandexTranslate {
            url: "https://translate.yandex.net/api/v1.5/tr.json/translate?",
            api_key: String::new(),
        }
    }

    fn set_apikey<D: Into<String>>(mut self, value: D) -> Self {
        self.api_key = value.into();
        self
    }

    fn translate_from_to(&self, what: &str, from: &str, to: &str) {

        let mut query: String = String::from(self.url);

        YandexTranslate::vec_to_string(vec!["key", &self.api_key, "from", from, "to", to]);


    }

    fn read_from_file(mut self, path: &str) {

    }

    fn vec_to_string(vec: Vec<&str>) -> String {

        let result: String = vec.iter()
            .fold(String::new(), |mut r, i| {
                r.push_str(i);
                r
            });

        result

    }

}

enum YandexTranslateError {

}
