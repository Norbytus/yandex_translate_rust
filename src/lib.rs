pub mod yclient {

    extern crate json;
    extern crate hyper;
    extern crate hyper_native_tls;

    use std::io::Read;
    use std::convert::Into;

    use self::hyper::client::Client;
    use self::hyper::net::HttpsConnector;
    use self::hyper_native_tls::NativeTlsClient;

    use self::json::JsonValue;

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
                url: "https://translate.yandex.net/api/v1.5/tr.json/translate?",
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
    
            query = format!("{}key={}&lang={}", query, self.api_key, ft);
    
            query = query + &YandexTranslate::vec_to_string(what, "&text=");
    
            self.execute(&query)
    
        }
    
        fn execute(&self, query: &str) -> YandexTranslateResult {
    
            let mut result: String = String::new();

            let request = self.client
                .get(query)
                .send()
                .unwrap()
                .read_to_string(&mut result)
                .unwrap();
    
            match json::parse(&result) {
                Ok(json) => return YandexTranslateResult::new(json),
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

    #[derive(Debug, Clone)]
    pub struct YandexTranslateResult {
        data: Option<JsonValue>,
    }

    impl YandexTranslateResult {

        fn new(json: JsonValue) -> YandexTranslateResult {
            YandexTranslateResult { data: Some(json) }
        }

        pub fn get_code(&self) -> Option<u64> {

           let mut data = self.data.clone().unwrap();

           let mut code = None;

           for val in data.entries() {
               match val {
                   ("code", &JsonValue::Number(num)) => {
                        let ( _, temp, _ ) = num.as_parts();
                        code = Some(temp);
                   },
                   _ => {},
               }
           }

            code

        }

        pub fn get_text(&self) {

           let mut data = self.data.clone().unwrap();

           //let mut text = None;

           for val in data.entries() {
               match val {
                   ("text", &JsonValue::Array(ref vec)) => {
                       //println!("{:?}", vec));
                   },
                   _ => {},
               }
           }
        
        }

    } 

}

