use std::fs;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

use dirs;
use futures::stream::TryStreamExt;
use reqwest::{Body, Client};
use tokio;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::lib::cred::Cred;
use crate::lib::flag::Flags;

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn detect_proglang(path: &Path) -> Result<String, Error> {
    let ext = path.extension().unwrap().to_str().unwrap();
    match ext {
        "rs" => Ok("rust".to_string()),
        "py" => Ok("python".to_string()),
        "js" => Ok("javascript".to_string()),
        "go" => Ok("golang".to_string()),
        "c" => Ok("c".to_string()),
        "cpp" => Ok("cpp".to_string()),
        "java" => Ok("java".to_string()),
        "cs" => Ok("csharp".to_string()),
        "php" => Ok("php".to_string()),
        "rb" => Ok("ruby".to_string()),
        "hs" => Ok("haskell".to_string()),
        "pl" => Ok("perl".to_string()),
        "sh" => Ok("bash".to_string()),
        "hs" => Ok("haskell".to_string()),
        "clj" => Ok("clojure".to_string()),
        "erl" => Ok("erlang".to_string()),
        "scala" => Ok("scala".to_string()),
        "ml" => Ok("ocaml".to_string()),
        _ => Err(Error::new(ErrorKind::Other, "Unsupported file type")),
    }
}

async fn login_to_pastebin(cred: &Cred) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut userkey = String::new();
    let data = [
        ("api_dev_key", cred.devkey.as_str()),
        ("api_user_name", cred.username.as_str()),
        ("api_user_password", cred.password.as_str()),
    ];

    let res = client
        .post("https://pastebin.com/api/api_login.php")
        .form(&data)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let mut body = res.bytes().await?;
            let buf = String::from_utf8_lossy(&body);
            userkey.push_str(&buf);
        }
        _ => {
            println!("{}", res.status());
        }
    }
    Ok(userkey)
}

#[tokio::main]
pub async fn upload_to_pastebin(
    input: &str,
    args: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let flag = Flags::set_flags(args);
    let cred = Cred::load();

    let userkey = login_to_pastebin(&cred).await.unwrap();

    let url = "https://pastebin.com/api/api_post.php";
    let data = [
        // required
        ("api_dev_key", cred.devkey.as_str()),
        ("api_option", "paste"),
        ("api_paste_code", input),
        // optional
        ("api_user_key", &userkey),
        ("api_paste_name", args.value_of("title").unwrap()),
        ("api_paste_format", ""),
        ("api_paste_private", ""),
        ("api_paste_expire_date", "10M"),
    ];

    let res = client.post(url).form(&data).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let mut body = res.bytes().await?;
            let buf = String::from_utf8_lossy(&body);
            println!("url: {}", buf);
        }
        _ => {
            println!("{}", res.status());
        }
    }

    Ok(())
}
