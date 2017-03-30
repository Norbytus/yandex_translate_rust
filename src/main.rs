extern crate yandex_translate;
extern crate clap;

use yandex_translate::yandex_translate::client::YandexTranslate;

use clap::{Arg, App};

use std::env;
use std::fs::{OpenOptions, File};
use std::io::{self, Read, Error, ErrorKind};

fn main() {

    let mut require_apikey = false;

    let mut conf_apikey: Option<String> = get_apikey();

    let mathc_args = App::new("Yandex translate program")
        .version("0.1")
        .author("Alexsander Startcev(Norbytus), norbyt93@gmail.com")
        .about("Simple programm work with Yandex translate API, write on Rust")
        .arg(Arg::with_name("lang")
            .short("l")
            .long("lang")
            .help("Language format in format 'From-To' example 'en-ru'")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("api_key")
             .short("k")
             .long("key")
             .help("Yandex translate API key")
             .required(require_apikey.clone())
             .takes_value(true))
        .arg(Arg::with_name("inline_text")
             .short("t")
             .long("text")
             .help("Text for translate")
             .required_unless_one(&["pipe"])
             .takes_value(true))
        .arg(Arg::with_name("pipe")
             .short("p")
             .long("pipe")
             .takes_value(false))
        .get_matches();

    let text = mathc_args.value_of("inline_text").unwrap_or("");
    println!("{:?}", text);
    /*let lang = mathc_args.value_of("lang").unwrap_or("en");


    let key: &str = mathc_args
        .value_of("api_key")
        .unwrap_or_else( || {
            &conf_apikey
        });

    let y_api = YandexTranslate::new();

    let request = y_api.set_apikey(key).translate_from_to(vec![text], lang);

    let answer = request.get_text().unwrap();

    println!("{:?}", answer);*/
}

fn get_homedir_path() -> Result<String, &'static str> {

    let env_arg = env::vars().find( | env_tuple | env_tuple.0 == "HOME");

    match env_arg {
        Some((_, path)) => Ok(path),
        _ => Err("Home dir don't found"),
    }

}

fn get_config_file_data() -> Result<File, Error> {

    let home_dir = get_homedir_path();

    if home_dir.is_err() {
        return Err(Error::new(ErrorKind::NotFound, "Not found home dir."))
    }

    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(home_dir.unwrap() + "/.yandex_translate/.yandex.conf");

    match config_file {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }

}

fn get_apikey() -> Option<String> {

    match get_config_file_data() {
        Ok(mut file) => {
            let mut data: String = String::new();
            file.read_to_string(&mut data);
            if data.is_empty() {
                None
            } else {
                Some(data)
            }
        },
        Err(e) => {
            println!("{}", e);
            None
        },
    }

}
