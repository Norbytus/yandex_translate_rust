# Yandex translate

## Description
This project i write for learning rust. If you have some ideas or find some errors, i be happy if you did pull request.

## Install
```bash
git clone https://github.com/Norbytus/yandex_translate_rust.git
cd yandex_translate_rust
cargo build --release
mv target/release/yandex_translate /usr/local/bin/
```
## Config(Optional)
You can create dir .yandex_translate in your home dir and create inside file .yandex_translate.conf(with your API key)

## Requirments
Need rustc 1.17.0-nightly

## Hot to use
Show help
```bash
yandex_translate -h
```
Translate text example
```bash
yandex_translate -l en-ru -t "Hello world"
```
Translate text from pipe example
```bash
echo Hello wolrd | yandex_translate -l ru -p
```
## OS
Testing on FreeBSD and ArchLinux
