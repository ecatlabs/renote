use clap::Arg;

pub fn create_query_arg<'help>() -> Arg<'help> {
    Arg::new("query")
        .help("Issue filter query")
        .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
        .short('q')
        .long("query")
        .value_delimiter(' ')
        .takes_value(true)
}
