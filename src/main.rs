use std::env;
use std::error::Error;
use std::io::{stdout, Read, Write};

use anyhow::anyhow;
use clap::ArgMatches;

use openai::{
    chat::{ChatCompletion, ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole},
    Credentials,
};
use tokio::sync::mpsc::Receiver;

mod cli;

#[tokio::main]
async fn main() {
    let matches = cli::clap().get_matches();

    if let Some(apikey) = matches.get_one::<String>("apikey") {
        env::set_var("OPENAPI_KEY", apikey);
    }

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
    let model = matches.get_one::<String>("model").unwrap();

    let task: Vec<String> = matches
        .get_many("task")
        .map_or_else(std::vec::Vec::new, |v| v.cloned().collect());

    let mut task = task.join(" ");

    let prompt: String;
    let temperature = *matches.get_one("temp").unwrap();

    if atty::is(atty::Stream::Stdin) {
        if task.is_empty() {
            return Err(anyhow!("running on tty, no task given").into());
        }
        prompt = "You are an assistant returning Linux shell commands that accomplish the following task. Don't add explanations or notes.".to_string();
        openai_request(&model, &prompt, &task, temperature).await?;
    } else {
        if task.is_empty() {
            task.push_str("Please fix this.");
        }
        prompt = format!("{task} Don't add explanations or notes.");

        // read from stdin
        let mut buffer = String::new();

        std::io::stdin().read_to_string(&mut buffer).unwrap();

        openai_request(&model, &prompt, &buffer, temperature).await?;
    }

    Ok(())
}

async fn openai_request(
    model: &str,
    prompt: &str,
    task: &str,
    temperature: f32,
) -> Result<(), Box<dyn Error>> {
    let credentials = Credentials::from_env();

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(prompt.to_string()),
        function_call: None,
        name: None,
        tool_call_id: None,
        tool_calls: Some(Vec::new()),
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(task.to_string()),
        name: None,
        function_call: None,
        tool_call_id: None,
        tool_calls: Some(Vec::new()),
    });

    let chat_stream = ChatCompletionDelta::builder(model, messages.clone())
        .credentials(credentials)
        .temperature(temperature)
        .create_stream()
        .await?;

    listen_for_tokens(chat_stream).await;
    Ok(())
}

async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) -> ChatCompletion {
    let mut merged: Option<ChatCompletionDelta> = None;
    while let Some(delta) = chat_stream.recv().await {
        let choice = &delta.choices[0];
        if let Some(content) = &choice.delta.content {
            print!("{}", content);
        }
        if choice.finish_reason.is_some() {
            // The message being streamed has been fully received.
            println!();
        }
        stdout().flush().unwrap();
        // Merge completion into accrued.
        match merged.as_mut() {
            Some(c) => {
                c.merge(delta).unwrap();
            }
            None => merged = Some(delta),
        };
    }
    merged.unwrap().into()
}
