extern crate json;

use self::json::JsonValue;

#[derive(Debug, Clone)]
pub struct YandexTranslateResult {
    data: JsonValue,
}

impl YandexTranslateResult {

    pub fn new(json: JsonValue) -> YandexTranslateResult {
        YandexTranslateResult { data: json }
    }

    pub fn get_code(&self) -> Result<u64, &'static str> {

        let data = self.data.clone();
        
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

                let data = self.data.clone();

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
