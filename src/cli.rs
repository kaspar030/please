use clap::{crate_authors, crate_description, crate_version, Arg, Command};

fn task() -> Arg {
    Arg::new("task")
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .num_args(1..100)
}

pub fn clap() -> clap::Command {
    Command::new(clap::crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .subcommand_negates_reqs(true)
        .arg(
            Arg::new("apikey")
                .short('k')
                .long("openai-api-key")
                .help("specify OpenAI API key")
                .env("OPENAI_KEY")
                .hide_env_values(true)
                .required(true)
                .value_parser(clap::value_parser!(String))
                .num_args(1),
        )
        .arg(task().required(true))
        .subcommand(Command::new("complete").arg(task()))
}
