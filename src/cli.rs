use clap::{crate_authors, crate_description, crate_version, value_parser, Arg, Command};

pub fn clap() -> clap::Command {
    fn parse_f32_0_1(s: &str) -> Result<f32, String> {
        parse_f32_ranged(s, 0.0..=1.0)
    }

    fn parse_f32_ranged(
        s: &str,
        range: impl std::ops::RangeBounds<f32> + std::fmt::Debug,
    ) -> Result<f32, String> {
        match s.parse::<f32>() {
            Ok(num) if range.contains(&num) => Ok(num),
            _ => Err(format!(
                "Input must be a float within the range {:?}",
                range
            )),
        }
    }
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
            Arg::new("temp")
                .short('t')
                .long("temp")
                .help("which temperature to use")
                .env("PLEASE_TEMP")
                .default_value("0.0")
                .value_parser(clap::builder::ValueParser::new(parse_f32_0_1)),
        )
        .arg(
            Arg::new("task")
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(1..100),
        )
}
