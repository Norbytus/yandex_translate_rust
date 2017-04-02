extern crate json;

use self::json::JsonValue;

use yandex_translate::code_errors::Code;

///Struct with answer from server in Json format
#[derive(Debug, Clone)]
pub struct YandexTranslateResult {
    data: JsonValue,
}

impl YandexTranslateResult {

    ///Constructor
    /// # Example
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate;
    ///
    /// let request = YandexTranslate::new();
    /// let result = request.set_apikey("Some_key").translate_from_to(vec!["Hello"], "en-ru");
    ///
    /// ```
    pub fn new(json: JsonValue) -> YandexTranslateResult {
        YandexTranslateResult { data: json }
    }

    /// Show code answer from server
    /// # Example
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate;
    ///
    /// let request = YandexTranslate::new();
    /// let result = request.set_apikey("Some_code").translate_from_to(vec!["Hello"], "en-ru");
    /// result.get_code();
    ///
    /// ```
    pub fn get_code(&self) -> Code {

        let data = self.data.clone();

        if data.has_key("code") {

            let (_, code, _) = data["code"]
                .as_number()
                .unwrap()
                .as_parts();

            match code {
                200 => Code::Success,
                401 => Code::InvalidKey,
                402 => Code::BlockedKey,
                404 => Code::DailyLimit,
                413 => Code::MaximumTextSize,
                422 => Code::CannotTranslate,
                501 => Code::NotSupportedLang,
                _ => Code::Error,
            }

        } else {

            Code::JsonWrongFormat

        }

    }

    /// Get text answer from server if code answer Success(200)
    /// # Example
    /// ```rust
    /// use yandex_translate::yandex_translate::client::YandexTranslate;
    ///
    /// let request = YandexTranslate::new();
    /// let result = request.set_apikey("Some_code").translate_from_to(vec!["Hello"], "en-ru");
    /// result.get_text();
    ///
    /// ```
    pub fn get_text(&self) -> Result<Vec<String>, Code> {

        let code = self.get_code();

        match code {
            Code::Success => {

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
            _ => {

                Err(code)

            }
        }

    }

} 


