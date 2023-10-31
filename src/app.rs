//! clap App for command cli
use clap::{Command, Arg, ArgAction};

pub const VERSION: &str = "0.13.1";

pub fn build_app() -> Command {
    Command::new("tk")
        .version(VERSION)
        .about("Task Keeper")
        .trailing_var_arg(true)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Verbose output")
                .required(false),
        )
        .arg(
            Arg::new("summary")
                .long("summary")
                .action(ArgAction::SetTrue)
                .help("List names of available tasks")
                .required(false),
        )
        .arg(
            Arg::new("doctor")
                .long("doctor")
                .action(ArgAction::SetTrue)
                .help("Check your system for potential problems to run tasks")
                .required(false),
        )
        .arg(
            Arg::new("no-dotenv")
                .long("no-dotenv")
                .action(ArgAction::SetTrue)
                .help("Disable to load .env file")
                .required(false),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .num_args(1)
                .value_parser(["shell", "make", "jbang", "just"])
                .help("Create a new task file by runner name")
                .required(false),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("List all tasks")
                .required(false),
        )
        .arg(
            Arg::new("runner")
                .long("runner")
                .short('r')
                .num_args(1)
                .help("Task Runner")
                .required(false),
        )
        .arg(
            Arg::new("from")
                .long("from")
                .num_args(1)
                .help("Source Runner")
                .required(false),
        )
        .arg(
            Arg::new("to")
                .long("to")
                .num_args(1)
                .help("Target Runner")
                .required(false),
        )
        .arg(Arg::new("tasks")
                 .required(false)
                 .help("Run task")
                 .index(1)
                 .num_args(1..),
        )
}
