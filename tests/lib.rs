extern crate yandex_translate;

use yandex_translate::yclient::YandexTranslate;

const API_KEY: &str = "trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e";

#[test]
fn create_struct() {

    let translate = YandexTranslate::new();

}

#[test]
fn set_api_key() {

    let translate = YandexTranslate::new();
    translate.set_apikey(API_KEY);

}
