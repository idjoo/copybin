use clap::ArgMatches;
use std::io::{Error, ErrorKind};
use std::path::Path;

fn detect_proglang(path: &Path) -> Result<String, Error> {
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
        "clj" => Ok("clojure".to_string()),
        "erl" => Ok("erlang".to_string()),
        "scala" => Ok("scala".to_string()),
        "ml" => Ok("ocaml".to_string()),
        _ => Err(Error::new(ErrorKind::Other, "Unsupported file type")),
    }
}

#[derive(Debug, Default)]
pub struct Flag {
    pub api_option: String,
    pub api_paste_name: String,
    pub api_paste_format: String,
    pub api_paste_private: String,
    pub api_paste_expire_date: String,
}

impl Flag {
    pub fn new() -> Self {
        Flag::default()
    }

    pub fn set_flags(a: &ArgMatches) -> Self {
        let mut flags = Self::new();

        flags.api_option = "paste".to_string();
        flags.api_paste_name = a.value_of("title").unwrap().to_string();

        let is_file = Path::new(a.value_of("input").unwrap_or("STDIN"));
        let is_auto = a.value_of("format").unwrap();
        if is_file != Path::new("STDIN") && is_auto == "auto" {
            flags.api_paste_format = detect_proglang(is_file).unwrap();
        } else {
            flags.api_paste_format = a.value_of("format").unwrap().to_string();
        }

        flags.api_paste_private = a.value_of("private").unwrap().to_string();
        flags.api_paste_expire_date = a.value_of("expire-date").unwrap().to_string();

        flags
    }
}
