extern crate yandex_translate;

use yandex_translate::yclient::YandexTranslate;

use std::env;

fn main() {

    let lang = read_env(("-l", "--lang"));

    let text = read_env(("-t", "--text"));

    if !lang.is_some() || !text.is_some() {
        panic!("Not language or text parameters");
    }

    let translate = YandexTranslate::new();

    let request = translate
        .set_apikey("trnsl.1.1.20170312T094041Z.4da8d12c2c6c961e.4bd73640b569f7bfb32b545e188ea1d79dd9cd0e")
        .translate_from_to(vec![&text.unwrap()], &lang.unwrap());

    let mut result_text: String = String::new();

    match request.get_text() {
        Ok(text) => {
            result_text = text.iter()
                .fold(String::new(), |mut s, v| {
                    s.push_str(v);
                    s
                });
        },
        Err(e) => panic!("{}", e)
    }

    println!("{:?}", result_text);

}

fn read_env(args: (&str, &str)) -> Option<String>{

    let mut args_iter = env::args().into_iter();

    let param = args_iter.find( |arg| arg == args.0 || arg == args.1 );

    let mut param_value: Option<String> = None;

    if param.is_some() {
        param_value = args_iter.next();
    }

    param_value

}
