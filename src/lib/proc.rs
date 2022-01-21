use reqwest::Client;
use tokio;

use crate::lib::cred::Cred;
use crate::lib::flag::Flag;
use colored::Colorize;

fn _type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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
            let body = res.bytes().await?;
            let buf = String::from_utf8_lossy(&body);
            userkey.push_str(&buf);
        }
        _ => {
            println!("{}: {}", "Warn".yellow(), res.status());
        }
    }
    Ok(userkey)
}

#[tokio::main]
pub async fn upload_to_pastebin(
    input: &str,
    args: &clap::ArgMatches,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut pastebin_url = String::new();
    let client = reqwest::Client::new();
    let cred = Cred::load();
    let flag = Flag::set_flags(args);

    if cred.username.is_empty() || cred.password.is_empty() {
        if flag.api_paste_private == "2" {
            println!(
                "{}: You need to set your credentials in the config file.",
                "Err".red()
            );
            std::process::exit(1);
        }
    }

    let userkey = login_to_pastebin(&cred).await.unwrap();

    let url = "https://pastebin.com/api/api_post.php";
    let data = [
        // required
        ("api_dev_key", cred.devkey.as_str()),
        ("api_option", &flag.api_option),
        ("api_paste_code", input),
        // optional
        ("api_user_key", &userkey),
        ("api_paste_name", &flag.api_paste_name),
        ("api_paste_format", &flag.api_paste_format),
        ("api_paste_private", &flag.api_paste_private),
        ("api_paste_expire_date", &flag.api_paste_expire_date),
    ];

    let res = client.post(url).form(&data).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.bytes().await?;
            let buf = String::from_utf8_lossy(&body);
            pastebin_url.push_str(&buf);
        }
        _ => {
            print!("{}: {}", "Err".red(), res.status());
        }
    }
    Ok(pastebin_url)
}
