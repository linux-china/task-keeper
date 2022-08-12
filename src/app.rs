//! clap App for command cli
use clap::{Command, Arg};

pub const VERSION: &str = "0.5.2";

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
            Arg::new("summary")
                .long("summary")
                .takes_value(false)
                .help("List names of available tasks")
                .required(false),
        )
        .arg(
            Arg::new("doctor")
                .long("doctor")
                .takes_value(false)
                .help("Check your system for potential problems to run tasks")
                .required(false),
        )
        .arg(
            Arg::new("no-dotenv")
                .long("no-dotenv")
                .takes_value(false)
                .help("Disable to load .env file")
                .required(false),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .takes_value(true)
                .value_names(&["shell", "make", "jbang", "just"])
                .help("Create a new task file by runner name")
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
