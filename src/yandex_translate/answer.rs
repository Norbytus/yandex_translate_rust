use serde_json::{self, Error as SError};
use std::result::Result;
use std::collections::HashMap;

///Enum with answer from request
#[derive(Debug)]
pub enum Answer {
    Translate(Translate),
    SupportLang(Langs),
    ErrorYt(ErrorYt),
}

impl Answer {

    ///Method return Answer with Translate or ErrorYt
    pub fn get_translate(data: String) -> Answer {

        let translate: Result<Translate, SError> = serde_json::from_str(&data);

        match translate {
            Ok(translate) => Answer::Translate(translate),
            Err(_) => {
                let error: ErrorYt = serde_json::from_str(&data).unwrap_or(ErrorYt::new());
                Answer::ErrorYt(error)
            }
        }

    }

    ///Method return Answer with supported language
    pub fn get_lang(data: String) -> Answer {

        let lang: Result<Langs, SError> = serde_json::from_str(&data);

        match lang {
            Ok(lang) => Answer::SupportLang(lang),
            Err(_) => {
                let error: ErrorYt = serde_json::from_str(&data).unwrap_or(ErrorYt::new());
                Answer::ErrorYt(error)
            }
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Langs {
    dirs: Vec<String>,
    langs: HashMap<String, String>,
}

impl Langs {
    ///Return Full language name by his code
    pub fn get_full_name_by_code(&self, lang_code: &str) -> Option<String> {
        match self.langs.get(lang_code) {
            Some(lang) => Some(lang.to_string()),
            None => None,
        }
    }

    ///Return vector supported language
    pub fn get_supports_lang(&self) -> Vec<String> {
        self.dirs.clone()
    }

    ///Check supported lang by his code
    pub fn is_support(&self, code: &str) -> bool {
        self.langs.contains_key(code)
    }
}
