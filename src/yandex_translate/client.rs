extern crate json;
extern crate hyper;
extern crate hyper_native_tls;

use self::hyper::client::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;

use yandex_translate::result::YandexTranslateResult;

use std::io::Read;


#[derive(Debug)]
pub struct YandexTranslate {
    url: &'static str,
    api_key: String,
    client: Client,
}

impl YandexTranslate {

    pub fn new() -> YandexTranslate {

        let ssl = NativeTlsClient::new().unwrap();

        let connector = HttpsConnector::new(ssl);

        YandexTranslate {
            url: "https://translate.yandex.net/api/v1.5/tr.json/",
            api_key: String::new(),
            client: Client::with_connector(connector),
        }

    }

    pub fn set_apikey<D: Into<String>>(mut self, value: D) -> Self {
        self.api_key = value.into();
        self
    }

    pub fn translate_from_to(&self, what: Vec<&str>, ft: &str)
        -> YandexTranslateResult {

        let mut query: String = String::from(self.url);

        query = format!("translate?{}key={}&lang={}", query, self.api_key, ft);

        query = query + &YandexTranslate::vec_to_string(what, "&text=");

        self.execute(&query)

    }

    fn execute(&self, query: &str) -> YandexTranslateResult {

        let mut result: String = String::new();

        self.client
            .get(query)
            .send()
            .unwrap()
            .read_to_string(&mut result)
            .unwrap();

        match json::parse(&result) {
            Ok(json) => YandexTranslateResult::new(json),
            Err(message) => panic!("{}", message),
        }

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
