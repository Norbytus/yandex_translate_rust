extern crate yandex_translate;

use yandex_translate::client::YandexTranslate;

const API_KEY: &'static str = "<API_KEY>";

#[test]
fn test_create_yandex_client() {

    YandexTranslate::new();

}

#[test]
fn test_set_api_key() {

    let y_api = YandexTranslate::new().set_apikey(API_KEY);

    assert_eq!(API_KEY, y_api.get_apikey());

}

#[test]
#[ignore]
fn test_translate() {

    use yandex_translate::answer::Answer;

    let y_api = YandexTranslate::new().set_apikey(API_KEY);

    let text = y_api.translate(vec!["Hello"], "en-ru");

    match text {
        Answer::Translate(_) => {},
        Answer::ErrorYt(err) => panic!("{:?}", err),
    }

}
