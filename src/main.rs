use clap::ArgMatches;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

mod cli;

#[tokio::main]
async fn main() {
    let matches = cli::clap().get_matches();

    openai::set_key(matches.get_one::<String>("apikey").unwrap().clone());

    match matches.subcommand() {
        Some(("complete", matches)) => handle_complete(matches).await,
        Some((&_, matches)) => handle_task(matches).await,
        None => handle_task(&matches).await,
    }
}

async fn handle_task(matches: &ArgMatches) {
    let task: Vec<String> = matches
        .get_many("task")
        .map_or_else(std::vec::Vec::new, |v| v.cloned().collect());

    let task = task.join(" ");
    let prompt = "You are an assistant returning Linux shell commands that accomplish the following task. Don't add explanations or notes.";

    let result = openai_request(prompt, &task).await;

    println!("{}", result);
}

async fn handle_complete(matches: &ArgMatches) {
    use std::io::Read;

    let task: Vec<String> = matches
        .get_many("task")
        .map_or_else(std::vec::Vec::new, |v| v.cloned().collect());

    let mut task = task.join(" ");

    if task.is_empty() {
        task.push_str("Please fix this.");
    }

    // read from stdin
    let mut buffer = String::new();

    // stdin() returns a handle to the standard input for your process
    // and read_to_string() reads from it until EOF or an error occurs.
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let prompt = format!("{task} Don't add explanations or notes.");

    let result = openai_request(&prompt, &buffer).await;

    println!("{}", result);
}

async fn openai_request(prompt: &str, task: &str) -> String {
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
        .await
        .unwrap()
        .unwrap();

    chat_completion
        .choices
        .first()
        .unwrap()
        .message
        .clone()
        .content
        .trim()
        .to_string()
}
