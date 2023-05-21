use std::error::Error;
use std::io::Read;

use anyhow::anyhow;
use clap::ArgMatches;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

mod cli;

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
        result = openai_request(&prompt, &task).await?;
    } else {
        if task.is_empty() {
            task.push_str("Please fix this.");
        }
        prompt = format!("{task} Don't add explanations or notes.");

        // read from stdin
        let mut buffer = String::new();

        std::io::stdin().read_to_string(&mut buffer).unwrap();

        result = openai_request(&prompt, &buffer).await?;
    }

    println!("{}", result);
    Ok(())
}

async fn openai_request(prompt: &str, task: &str) -> Result<String, Box<dyn Error>> {
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

    let chat_completion = ChatCompletion::builder("gpt-4", messages.clone())
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
