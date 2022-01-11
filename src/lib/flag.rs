use clap::ArgMatches;

/// Struct representting the argument flags.
///
/// # Respective flags represented by the fields:
/// ```
/// --count, -c
/// --line-number, -n
/// --color
/// --ignore-case, -i
/// --invert-match, -v
/// --after-context, -A,
/// --before-context, -B,
/// --context, -C,
/// ```

#[derive(Debug, Default)]
pub struct Flags {
    pub dev_key: bool,
    pub user_name: bool,
    pub user_password: bool,
    pub paste_name: bool,
    pub paste_format: bool,
    pub paste_private: bool,
    pub paste_expired_date: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags::default()
    }

    pub fn set_flags(a: &ArgMatches) -> Self {
        let mut flags = Self::new();

        flags.paste_name = a.is_present("title");
        flags.paste_format = a.is_present("format");
        flags.paste_private = a.is_present("private");
        flags.paste_expired_date = a.is_present("expired-date");

        flags
    }
}
