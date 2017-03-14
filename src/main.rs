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

    /*let json_answer = json::parse(&answer).unwrap();
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
    client: Client,
}

impl YandexTranslate {

    fn new() -> YandexTranslate {

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        YandexTranslate {
            url: "https://translate.yandex.net/api/v1.5/tr.json/translate?",
            api_key: String::new(),
            client: Client::with_connector(connector),
        }

    }

    fn set_apikey<D: Into<String>>(mut self, value: D) -> Self {
        self.api_key = value.into();
        self
    }

    fn translate_from_to(&self, what: &str, from: &str, to: &str) {

        let mut result: String = String::new();

        let mut query: String = String::from(self.url);

        let query_params: Vec<&str> = vec![ "key", &self.api_key, "from", from, "to", to];

        query = query + &YandexTranslate::vec_to_string(query_params);

        let request = self.client
            .get(&query)
            .send()
            .unwrap()
            .read_to_string(&mut result)
            .unwrap();

    }

    fn vec_to_string(vec: Vec<&str>) -> String {

        let result: String = vec.iter()
            .enumerate()
            .fold(String::new(), |mut r, (i, v)| {

                match i % 2 {
                    0 => {
                        r.push_str(v);
                        r.push_str("=");
                    },
                    _ => {
                        r.push_str(v);
                        r.push_str("&");
                    },
                }

                r

            });

        result

    }

}

enum YandexTranslateError {

}
