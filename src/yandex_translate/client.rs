extern crate hyper;
extern crate hyper_native_tls;

use self::hyper::client::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;

use std::io::Read;

use super::answer::Answer;

///Base url for request
const BASE_URL: &'static str = "https://translate.yandex.net/api/v1.5/tr.json/";


//Client for query
pub struct YandexTranslate {
    ///API_KEY
    api_key: String,
    ///Client object
    client: Client,
}

impl YandexTranslate {

    ///Create new object
    ///# Example
    ///```rust
    ///use yandex_translate::client::YandexTranslate;
    ///
    ///let request = YandexTranslate::new();
    ///```
    pub fn new() -> YandexTranslate {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        YandexTranslate {
            api_key: String::new(),
            client: Client::with_connector(connector),
        }
    }

    pub fn get_apikey(&self) -> &str {
        &self.api_key
    }

    ///Set api key
    ///# Example
    ///```rust
    ///use yandex_translate::client::YandexTranslate;
    ///
    ///let request = YandexTranslate::new().set_apikey("<API_KEY>");
    ///```
    pub fn set_apikey<D: Into<String>>(mut self, value: D) -> Self {
        self.api_key = value.into();
        self
    }

    ///Translate text
    ///# Example
    ///```rust
    ///use yandex_translate::client::YandexTranslate;
    ///
    ///let request = YandexTranslate::new().set_apikey("<API_KEY>");
    ///let translate = request.translate(vec!["Hello World!"], "en-ru");
    ///```
    pub fn translate(&self, what: Vec<&str>, ft: &str) -> Answer {
        let mut query: String = String::from(BASE_URL);
        query = format!("{}translate?key={}&lang={}", query, self.api_key, ft);
        query = query + &YandexTranslate::vec_to_string(what, "&text=");
        self.execute(&query)
    }

    fn execute(&self, query: &str) -> Answer {
        let mut result: String = String::new();

        self.client
            .get(query)
            .send()
            .unwrap()
            .read_to_string(&mut result)
            .unwrap();

        Answer::get_translate(result)
    }

    fn vec_to_string(vec: Vec<&str>, delimetr: &str) -> String {
        let result: String = vec.iter()
            .fold(String::new(), |mut r, v| {
                r.push_str(&format!("{}{}", delimetr, v));
                r
            });

        result
    }

}
