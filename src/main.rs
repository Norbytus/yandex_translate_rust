extern crate yandex_translate;
extern crate clap;

use yandex_translate::yandex_translate::client::YandexTranslate;

use clap::{Arg, App};

use std::env;
use std::fs::{OpenOptions, File};
use std::io::{self, Read, Write, Error, ErrorKind};
use std::convert::Into;

fn main() {

    let conf_apikey: Option<String> = get_apikey();

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
             .required(conf_apikey.is_none())
             .takes_value(true))
        .arg(Arg::with_name("inline_text")
             .short("t")
             .long("text")
             .help("Text for translate")
             .multiple(true)
             .required_unless_one(&["pipe"])
             .takes_value(true))
        .arg(Arg::with_name("pipe")
             .short("p")
             .long("pipe")
             .help("If this flag exsist other flag like t will be ignored.")
             .takes_value(false)
             .empty_values(true))
        .get_matches();

    let mut text: String = mathc_args
        .value_of("inline_text")
        .unwrap_or("")
        .into();

    if mathc_args.is_present("pipe") { text = get_text_from_pipe(); }

    let lang = mathc_args.value_of("lang").unwrap_or("en");

    let key_flag: String = mathc_args
        .value_of("api_key")
        .unwrap_or(&conf_apikey.unwrap())
        .into();

    let y_api = YandexTranslate::new();

    let request = y_api.set_apikey(key_flag).translate_from_to(vec![&text], lang);

    let answer = request.get_text();

    let result = match answer {
        Ok(vec) => {
            vec.into_iter().fold(String::new(), |mut temp, word| {
                temp = format!(" {} {} ", temp, word);
                temp
            })
        },
        Err(e) => {
            format!("{}", e)
        }
    };

    io::stdout().write(result.trim().to_string().as_bytes());

}

fn get_text_from_pipe() -> String {

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => String::new(),
    }

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

            file.read_to_string(&mut data).unwrap_or(0);

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
