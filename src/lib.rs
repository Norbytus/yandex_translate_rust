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

        pub fn get_code(&self) -> Result<u64, &'static str> {

            let data = self.data.clone().unwrap();
            
            if data.has_key("code") {

                let (_, code, _) = data["code"]
                    .as_number()
                    .unwrap()
                    .as_parts();

                match code {
                    200 => { Ok(code) },
                    401 => { Err("Invalid API key.") },
                    402 => { Err("Blocked API key.") },
                    404 => { Err("Exceeded the daily limit on the amount of translated text.") },
                    413 => { Err("Exceeded the maximum text size.") },
                    422 => { Err("The text cannot be translated.") },
                    501 => { Err("The specified translation direction is not supported.") },
                    _ => { Err("Error") },
                }

            } else {

                Err("Wrong JSON format")

            }

        }

        pub fn get_text(&self) -> Result<Vec<String>, &'static str> {

            let code = self.get_code();

            match code {
                Ok(_) => {

                    let data = self.data.clone().unwrap();

                    let mut result_vec: Vec<String> = Vec::new();

                    for text in data["text"].members() {

                        let text: String = text
                            .as_str()
                            .unwrap()
                            .parse::<String>()
                            .unwrap();

                        result_vec.push(text);

                    };

                    Ok(result_vec)

                },
                Err(e) => {

                    Err("Some error")

                }
            }

        }

    } 

}

