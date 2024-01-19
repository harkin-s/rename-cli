use std::error::Error;
use std::fs;

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_names = get_file_names("/Users/sean/Downloads/Our.Flag.Means.Death.S01.COMPLETE.720p.HMAX.WEBRip.x264-GalaxyTV[TGx]")?;

    println!("{:?}", file_names);
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    println!("Usage: {:?}", response.usage);

    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}

fn get_file_names(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file_names: Vec<String> = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name()
                    .into_string()
                    .ok()
                    .map(|s| format!("{}/{}", folder_path, s))
            })
        })
        .collect();

    Ok(file_names)
}
