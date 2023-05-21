use std::error::Error;
use std::io::Read;

use anyhow::anyhow;
use clap::{ArgMatches, ValueEnum};

use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

mod cli;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub(crate) enum Model {
    /// Use gpt-3.5-turbo
    #[default]
    #[clap(name = "gpt-3.5-turbo", alias = "gpt-3")]
    GPT3_5,
    /// Use gpt-4
    #[clap(name = "gpt-4")]
    GPT4,
}

#[tokio::main]
async fn main() {
    let matches = cli::clap().get_matches();

    openai::set_key(matches.get_one::<String>("apikey").unwrap().clone());

    if let Err(e) = handle_task(&matches).await {
        eprintln!("please: error: {e:#}");
        std::process::exit(1);
    }

    // match matches.subcommand() {
    //     Some(("complete", matches)) => handle_complete(matches).await,
    //     Some((&_, matches)) => handle_task(matches).await,
    //     None => handle_task(&matches).await,
    // }
}

async fn handle_task(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let model: Model = matches
        .get_one::<Model>("model")
        .map_or_else(Model::default, |m| *m);

    let task: Vec<String> = matches
        .get_many("task")
        .map_or_else(std::vec::Vec::new, |v| v.cloned().collect());

    let mut task = task.join(" ");

    let prompt: String;

    let result;
    if atty::is(atty::Stream::Stdin) {
        if task.is_empty() {
            return Err(anyhow!("running on tty, no task given").into());
        }
        prompt = "You are an assistant returning Linux shell commands that accomplish the following task. Don't add explanations or notes.".to_string();
        result = openai_request(model, &prompt, &task).await?;
    } else {
        if task.is_empty() {
            task.push_str("Please fix this.");
        }
        prompt = format!("{task} Don't add explanations or notes.");

        // read from stdin
        let mut buffer = String::new();

        std::io::stdin().read_to_string(&mut buffer).unwrap();

        result = openai_request(model, &prompt, &buffer).await?;
    }

    println!("{}", result);
    Ok(())
}

async fn openai_request(model: Model, prompt: &str, task: &str) -> Result<String, Box<dyn Error>> {
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: prompt.to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: task.to_string(),
        name: None,
    });

    let chat_completion = ChatCompletion::builder(
        model.to_possible_value().unwrap().get_name(),
        messages.clone(),
    )
    .create()
    .await??;

    let response = chat_completion
        .choices
        .first()
        .ok_or("No response received")?
        .message
        .clone()
        .content
        .trim()
        .to_string();

    Ok(response)
}
