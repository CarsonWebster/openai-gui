#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use async_openai as openai;
use openai::{types::CreateCompletionRequest, Client, Completion};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
#[tauri::command]
async fn complete(prompt: &str) -> String {
    // let client = Client::new().with_api_key("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");
    let client = Client::new();

    let completion_request = CreateCompletionRequest {
        model: "text-davinci-003".to_owned(),
        prompt: Some(prompt.to_owned()),
        max_tokens: Some(100),
        ..Default::default()
    };
    let completion_response = Completion::create(&client, completion_request).await.unwrap();
    println!("{:?}", completion_response);
    let response = completion_response.choices[0].text.clone();
    format!("{}", response)
}

#[tauri::command]
fn edit(prompt: &str) -> String {
    format!("Edit with prompt: {}", prompt)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, complete, edit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
