use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    completions::Completion,
    set_key,
};

mod cli;

#[tokio::main]
async fn main() {
    let m = cli::clap().get_matches();

    openai::set_key(m.get_one::<String>("apikey").unwrap().clone());

    let task: Vec<String> = m
        .get_many("task")
        .map_or_else(std::vec::Vec::new, |v| v.cloned().collect());

    let task = task.join(" ");

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content:
            "You are an assistant returning Linux shell commands that accomplish the following task. Don't add explainations or notes."
                .to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: task,
        name: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-4", messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    println!("{}", &returned_message.content.trim());
}
