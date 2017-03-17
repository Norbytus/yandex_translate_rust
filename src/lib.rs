pub mod yclient {

    extern crate json;
    extern crate hyper;
    extern crate hyper_native_tls;
    
    use std::io::Read;
    use std::fmt::Debug;
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
    
        pub fn translate_from_to(&self, what: &str, from: &str, to: &str) -> YandexTranslateResult {
    
            let mut query: String = String::from(self.url);
    
            let mut lang: String = String::new();
            lang.push_str(from);
            lang.push_str("-");
            lang.push_str(to);
    
            let query_params: Vec<&str> = vec![
                "key",
                &self.api_key,
                "lang",
                &lang,
                "text",
                what ];
    
            query = query + &YandexTranslate::vec_to_string(query_params);
    
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
    
    #[derive(Debug)]
    pub struct YandexTranslateResult {
        data: Option<JsonValue>,
    }
    
    impl YandexTranslateResult {
    
            fn new(json: JsonValue) -> YandexTranslateResult {
                YandexTranslateResult { data: Some(json) }
            }
    
            fn get_code(&self) {
                let temp = self.data.clone();
                //for row in temp.unwrap().entries() {
                //    match row {
                //        ("code", &JsonValue::Number(code)) => return Some(code.into()),
                //        _  => return None,
                //    }
                //}
            }
    
    }

}
