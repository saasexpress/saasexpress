use clap::ArgMatches;

pub(crate) fn get(matches: &ArgMatches) {
    let args = matches
        .get_many::<String>("URL")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    println!("args: {:?}", args);
}
