//! clap App for command cli
use clap::{Command, Arg, AppSettings};

pub const VERSION: &str = "0.1.0";

pub fn build_app() -> Command<'static> {
    Command::new("tk")
        .version(VERSION)
        .about("Task Keeper")
        .trailing_var_arg(true)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .help("Verbose output")
                .required(false),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .takes_value(false)
                .help("List all tasks")
                .required(false),
        )
        .arg(
            Arg::new("runner")
                .long("runner")
                .short('r')
                .takes_value(false)
                .help("Task Runner")
                .required(false),
        )
        .arg(
            Arg::new("from")
                .long("from")
                .takes_value(false)
                .help("Source Runner")
                .required(false),
        )
        .arg(
            Arg::new("to")
                .long("to")
                .takes_value(false)
                .help("Target Runner")
                .required(false),
        )
        .arg(Arg::new("tasks")
            .required(false)
            .help("Run task")
            .index(1)
            .multiple_values(true),
        )
}
