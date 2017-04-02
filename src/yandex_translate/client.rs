extern crate json;
extern crate hyper;
extern crate hyper_native_tls;

use self::hyper::client::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;

use yandex_translate::result::YandexTranslateResult;

use std::io::Read;

#[doc="this"]

///Client for yandex translate api
#[derive(Debug)]
pub struct YandexTranslate {
    ///Url for request
    url: &'static str,
    ///Api key
    api_key: String,
    ///Client object
    client: Client,
}

impl YandexTranslate {

    ///Constructor a new YandexTranslate
    /// # Example 
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate
    ///
    /// let request = YandexTranslate::new();
    /// ```
    pub fn new() -> YandexTranslate {

        let ssl = NativeTlsClient::new().unwrap();

        let connector = HttpsConnector::new(ssl);

        YandexTranslate {
            url: "https://translate.yandex.net/api/v1.5/tr.json/",
            api_key: String::new(),
            client: Client::with_connector(connector),
        }

    }

    ///Set field api key
    /// # Example
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate
    ///
    /// let request = YandexTranslate::new();
    /// request.set_apikey("Some_key");
    /// ```
    pub fn set_apikey<D: Into<String>>(mut self, value: D) -> Self {
        self.api_key = value.into();
        self
    }

    /// Translate text fron one language to another
    /// #Example
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate
    ///
    /// let request = YandexTranslate::new();
    /// let result = request.set__apikey("Some_key").translate_from_to(vec!["Hello"], "en-ru");
    /// ```
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
