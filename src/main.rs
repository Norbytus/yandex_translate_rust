extern crate yandex_translate;

use yandex_translate::yclient::YandexTranslate;

use std::env;

fn main() {

    read_env();

    let translate = YandexTranslate::new();

    let request = translate
        .set_apikey("trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e")
        .translate_from_to(vec!["Hello", "World"], "en-ru");

    request.get_text();

}

fn read_env() {

    let mut args_iter = env::args().into_iter();

    let lang = args_iter.find( |arg| arg == "-l" || arg == "--lang" );

    let mut lang_param: Option<String> = None;

    if lang.is_some() {
        lang_param = args_iter.next();
    }

}
