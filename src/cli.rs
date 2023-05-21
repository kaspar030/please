use clap::{crate_authors, crate_description, crate_version, value_parser, Arg, Command};

pub fn clap() -> clap::Command {
    Command::new(clap::crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
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
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .help("which LLM to use")
                .env("PLEASE_MODEL")
                .value_parser(value_parser!(super::Model)),
        )
        .arg(
            Arg::new("task")
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(1..100),
        )
}
