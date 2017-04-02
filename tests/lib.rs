extern crate yandex_translate;

use yandex_translate::yandex_translate::client::YandexTranslate;
use yandex_translate::yandex_translate::result::YandexTranslateResult;

const API_KEY: &str = "trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e";

#[test]
fn create_struct() {

    let translate = YandexTranslate::new();
    println!("{:?}", translate);

}

#[test]
fn set_api_key() {

    let translate = YandexTranslate::new();
    println!("{:?}", translate.set_apikey(API_KEY));

}

#[test]
fn get_code() {

    let translate = YandexTranslate::new();

    let request = translate.set_apikey(API_KEY)
        .translate_from_to(vec!["Hello"], "en-ru");

    println!("{:?}", request.get_code());

}

#[test]
fn get_text() {

    let translate = YandexTranslate::new();
    let request = translate.set_apikey(API_KEY)
        .translate_from_to(vec!["Hello"], "en-ru");
    request.get_text();

}
