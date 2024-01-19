use std::error::Error;
use std::fs;

use async_openai::{
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use serde_json::Map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_names = get_file_names("")?;

    println!("{:?}", file_names);
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestUserMessageArgs::default()
                .content("I would like to rename the files in this folder here are the names of the files:")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(file_names.join("\n"))
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Can you simplify the names of these files returning a list of key value pairs where the key is the old file name and the value is the new file name?")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Provide your answer in JSON format with no additional text.")
                .build()?
                .into(),
        ])
        .build()?;



    let response = client.chat().create(request).await?;

    let mut new_file_names: Map<String, serde_json::Value> = Map::new();

    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );

        let as_json: serde_json::Value = serde_json::from_str(choice.message.content.as_deref().unwrap_or("{}"))?;

        for (key, value) in as_json.as_object().unwrap() {
            println!("Old: {} : New: {}", key, value);
            new_file_names.insert(key.to_string(), value.clone()); 
        }
    }

    //

    Ok(())
}

fn get_file_names(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file_names: Vec<String> = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().into_string().ok().map(|s| format!("{}", s)))
        })
        .collect();

    Ok(file_names)
}
