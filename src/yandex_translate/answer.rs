use serde_json::{self, Error as SError};
use std::result::Result;

#[derive(Debug)]
pub enum Answer {
    Translate(Translate),
    ErrorYt(ErrorYt),
}

impl Answer {

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Translate {
    code: u32,
    lang: String,
    text: Vec<String>,
}

impl Translate {
    fn get_lang(&self) -> String {
        self.lang.clone()
    }
}

impl GetInfo for Translate {
    fn get_code(&self) -> u32 {
        self.code
    }

    fn get_message(&self) -> String {
        let result = self.text.iter()
            .fold(String::new(), |mut r, v| {
                r.push_str(&format!("{}", v));
                r
            });
        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorYt {
    code: u32,
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
    fn get_code(&self) -> u32 {
        self.code
    }

    fn get_message(&self) -> String {
        self.message.clone()
    }
}

pub trait GetInfo {
    fn get_code(&self) -> u32;

    fn get_message(&self) -> String;
}
