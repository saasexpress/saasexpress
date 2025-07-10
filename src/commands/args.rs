use std::path::PathBuf;

use clap::{ArgAction, ArgMatches, Command, arg, command, value_parser};

pub(crate) fn parse_commands() -> ArgMatches {
    command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -i --stdin ... "Read graphs from stdin"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -s --samples ... "Load predefined sample graphs"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -w --watch ... "Turn watching graph folder for hot reload"
            )
            .default_value("true")
            .action(ArgAction::SetTrue),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("get")
                .about("Retrieve reusable graphs from saasexpress registry")
                .arg(arg!([URL]).action(ArgAction::Append)),
        )
        .get_matches()
}
