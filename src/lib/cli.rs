use clap::{App, Arg, ArgGroup, ArgMatches, PossibleValue};

pub struct Cli<'a> {
    app: App<'a>,
}

impl<'a> Cli<'a> {
    pub fn new() -> Self {
        let app = App::new("copybin")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .version("0.1.1")
            .author("cocatrip, cocatrip@yahoo.com")
            .about("Copy contents of a file and upload to pastebin")
            .arg(
                Arg::new("input")
                    .help("Input [file|stdin]")
                    .takes_value(true),
            )
            .arg(
                Arg::new("title")
                    .help("Specify the title of the paste")
                    .long("title")
                    .short('t')
                    .takes_value(true)
                    .default_value("Untitled"),
            )
            .arg(
                Arg::new("format")
                    .help("Language format of the file (syntax highlighting)")
                    .long("format")
                    .short('f')
                    .takes_value(true)
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
                    .possible_value(
                        PossibleValue::new("2").help("Private (in combination with api_user_key)"),
                    )
                    .default_value("0"),
            )
            .arg(
                Arg::new("expire-date")
                    .help("Specify expiration time of the paste")
                    .long("expire-date")
                    .short('e')
                    .takes_value(true)
                    .possible_value(PossibleValue::new("N").help("Never"))
                    .possible_value(PossibleValue::new("10M").help("10 Minutes"))
                    .possible_value(PossibleValue::new("1H").help("1 Hour"))
                    .possible_value(PossibleValue::new("1D").help("1 Day"))
                    .possible_value(PossibleValue::new("1W").help("1 Week"))
                    .possible_value(PossibleValue::new("2W").help("2 Weeks"))
                    .possible_value(PossibleValue::new("1M").help("1 Month"))
                    .possible_value(PossibleValue::new("6M").help("6 Months"))
                    .possible_value(PossibleValue::new("1Y").help("1 Year"))
                    .default_value("10M"),
            )
            .group(
                ArgGroup::new("options")
                    .args(&["format", "private", "expire-date", "title"])
                    .required(false),
            );
        Self { app }
    }

    pub fn parse(self) -> ArgMatches {
        self.app.get_matches()
    }
}
