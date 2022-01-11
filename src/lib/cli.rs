use clap::{App, Arg, ArgMatches, PossibleValue};

pub struct Cli<'a> {
    app: App<'a>,
}

impl<'a> Cli<'a> {
    pub fn new() -> Self {
        let app = App::new("copybin")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .version("0.1.0")
            .author("cocatrip, cocatrip@yahoo.com")
            .about("Copy contents of a file and upload to pastebin")
            .arg(
                Arg::new("input")
                    .help("Input file")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("title")
                    .help("Title of the paste")
                    .long("title")
                    .short('t')
                    .takes_value(true)
                    .required(false)
                    .default_value("Untitled"),
            )
            .arg(
                Arg::new("format")
                    .help("Language format of the file (syntax highlighting)")
                    .long("format")
                    .short('f')
                    .takes_value(true)
                    .required(false)
                    .default_value("auto"),
            )
            .arg(
                Arg::new("private")
                    .help("Set publicity of the paste")
                    .long("private")
                    .short('p')
                    .takes_value(true)
                    .possible_value(PossibleValue::new("0").help("Public"))
                    .possible_value(PossibleValue::new("1").help("Unlisted"))
                    .possible_value(PossibleValue::new("2").help("Private (in combination with api_user_key)"))
                    .required(false)
                    .default_value("0"),
            )
            .arg(
                Arg::new("expired-date")
                    .help("Specify expiration time of the paste")
                    .long("expired-date")
                    .short('e')
                    .takes_value(true)
                    .required(false)
                    .default_value("10M"),
            )
        ;Self { app }
    }

    pub fn parse(self) -> ArgMatches {
        self.app.get_matches()
    }
}
