extern crate yandex_translate;

use yandex_translate::yclient::YandexTranslate;

fn main() {

    let translate = YandexTranslate::new();

    let request = translate
        .set_apikey("trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e")
        .translate_from_to(vec!["Hello", "World"], "en-ru");
    println!("{:?}", request);

   //request.get_text();

}
