extern crate json;

use self::json::JsonValue;

use yandex_translate::code_errors::Code;

#[derive(Debug, Clone)]
pub struct YandexTranslateResult {
    data: JsonValue,
}

impl YandexTranslateResult {

    pub fn new(json: JsonValue) -> YandexTranslateResult {
        YandexTranslateResult { data: json }
    }

    pub fn get_code(&self) -> Result<Code, Code> {

        let data = self.data.clone();
        
        if data.has_key("code") {

            let (_, code, _) = data["code"]
                .as_number()
                .unwrap()
                .as_parts();

            match code {
                200 => { Ok( Code::Success ) },
                401 => { Err( Code::InvalidKey{ code: "Invalid API key." } ) },
                402 => { Err( Code::BlockedKey{ code: "Blocked API key." } ) },
                404 => { Err( Code::DailyLimit{ code: "Exceeded the daily limit on the amount of translated text." } ) },
                413 => { Err( Code::MaximumTextSize{ code: "Exceeded the maximum text size." } ) },
                422 => { Err(Code::CannotTranslate{ code: "The text cannot be translated." } )},
                501 => { Err(Code::NotSupportedLang{ code: "The specified translation direction is not supported." } )},
                _ => { Err(Code::Error) },
            }

        } else {

            Err(Code::JsonWrongFormat)

        }

    }

    pub fn get_text(&self) -> Result<Vec<String>, Code> {

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

                Err(e)

            }
        }

    }

} 


