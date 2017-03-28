extern crate yandex_translate;
extern crate clap;

use yandex_translate::yandex_translate::client::YandexTranslate;

use clap::{Arg, App};

use std::env;
use std::fs::{OpenOptions, File};
use std::io::{self, Read};

fn main() {

    let home_var = env::vars().find( | var | var.0 == "HOME" );

    let mut require_apikey = false;

    let mut conf_apikey: String = String::new();

    let mut path: String = match home_var {
        Some(temp_path) => {
            temp_path.1
        },
        None => String::new(),
    };

    let config = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.clone() + "/.yandex.conf");

    let file: Option<File> = match config {
        Ok(file) => Some(file),
        Err(_) => {
            println!("Config file undefined");
            None
        },
    };

    if file.is_some() {
        file.unwrap().read_to_string(&mut conf_apikey);
    }

    if conf_apikey.is_empty() {
        require_apikey = true;
    }

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
             .takes_value(true))
        .get_matches();

    let lang = mathc_args.value_of("lang").unwrap_or("en");

    let text = mathc_args.value_of("inline_text").unwrap_or("");

    let key: &str = mathc_args
        .value_of("api_key")
        .unwrap_or_else( || {
            &conf_apikey
        });

    let y_api = YandexTranslate::new();

    let request = y_api.set_apikey(key).translate_from_to(vec![text], lang);

    println!("{:?}", request.get_text());
    
 
}
