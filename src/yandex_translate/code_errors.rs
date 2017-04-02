use std::fmt;

const SUCCESS: &str = "Success.";
const INVALID_KEY: &str = "Invalid API key.";
const BLOCKED_KEY: &str = "Blocked API key.";
const DAILY_LIMIT: &str = "Exceeded the daily limit on the amount of translated text.";
const MAXIMUM_TEXT_SIZE: &str = "Exceeded the maximum text size.";
const CANNOT_TRANSLATE: &str = "The text cannot be translated.";
const NOT_SUPPORTED_LANG: &str = "The specified translation direction is not supported.";
const JSON_WRONG_FORMAT: &str = "Json wrong format.";
const ERROR: &str = "Unknown error.";

#[derive(Debug)]
pub enum Code {
    Success,
    InvalidKey,
    BlockedKey,
    DailyLimit,
    MaximumTextSize,
    CannotTranslate,
    NotSupportedLang,
    JsonWrongFormat,
    Error,
}

impl fmt::Display for Code {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let printable = match *self {
            Code::Success => SUCCESS,
            Code::InvalidKey => INVALID_KEY,
            Code::BlockedKey => BLOCKED_KEY,
            Code::DailyLimit => DAILY_LIMIT,
            Code::MaximumTextSize => MAXIMUM_TEXT_SIZE,
            Code::CannotTranslate => CANNOT_TRANSLATE,
            Code::NotSupportedLang => NOT_SUPPORTED_LANG,
            Code::JsonWrongFormat => JSON_WRONG_FORMAT,
            Code::Error => ERROR,
        };

        write!(f, "{}", printable)

    }

}
