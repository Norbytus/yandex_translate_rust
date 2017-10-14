use serde_json::{self, Error as SError};
use std::result::Result;

///Enum with answer from request
#[derive(Debug)]
pub enum Answer {
    Translate(Translate),
    ErrorYt(ErrorYt),
}

impl Answer {

    ///Method return Answer with Translate or ErrorYt
    pub fn get_translate(data: String) -> Answer {

        let translate: Result<Translate, SError> = serde_json::from_str(&data);

        let answer: Answer = match translate {
            Ok(translate) => Answer::Translate(translate),
            Err(_) => {
                let error: ErrorYt = serde_json::from_str(&data).unwrap_or(ErrorYt::new());
                Answer::ErrorYt(error)
            }
        };

        answer

    }

}

///Object with translate answer from server
#[derive(Serialize, Deserialize, Debug)]
pub struct Translate {
    ///Answer code
    code: u32,
    ///Lang translate
    lang: String,
    ///Result text
    text: Vec<String>,
}

impl Translate {
    ///Return lang description
    fn get_lang(&self) -> String {
        self.lang.clone()
    }
}

impl GetInfo for Translate {
    ///Return Code answer
    fn get_code(&self) -> u32 {
        self.code
    }

    ///Return translated text
    fn get_message(&self) -> String {
        let result = self.text.iter()
            .fold(String::new(), |mut r, v| {
                r.push_str(&format!("{}", v));
                r
            });
        result
    }
}

///Object with erro answer from server
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorYt {
    ///Answer Code
    code: u32,
    ///Error description
    message: String,
}

impl ErrorYt {
    fn new() -> Self {
        Self {
            code: 0,
            message: String::new(),
        }
    }
}

impl GetInfo for ErrorYt {
    ///Return error code
    fn get_code(&self) -> u32 {
        self.code
    }

    ///Return error description
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

///Base methods for text and code field
pub trait GetInfo {
    fn get_code(&self) -> u32;

    fn get_message(&self) -> String;
}
