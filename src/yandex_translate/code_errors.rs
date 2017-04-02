use std::fmt;

#[derive(Debug)]
pub enum Code {
    Success,
    InvalidKey { code: &'static str },
    BlockedKey { code: &'static str },
    DailyLimit { code: &'static str },
    MaximumTextSize { code: &'static str },
    CannotTranslate { code: &'static str },
    NotSupportedLang { code: &'static str },
    JsonWrongFormat,
    Error,
}

impl fmt::Display for Code {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let printable = match *self {
            Code::Success => "Success",
            Code::InvalidKey{ code } => code,
            Code::BlockedKey{ code } => code,
            Code::DailyLimit{ code } => code,
            Code::MaximumTextSize{ code } => code,
            Code::CannotTranslate{ code } => code,
            Code::NotSupportedLang{ code } => code,
            Code::JsonWrongFormat => "JsonWrongFormat",
            Code::Error => "Error",
        };

        write!(f, "{}", printable)

    }

}
