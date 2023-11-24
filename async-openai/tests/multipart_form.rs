use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::{types::CreateTranscriptionRequestArgs, Client};

#[tokio::test]
async fn multipart_form_test() {
    let client = Client::new();
    println!("current dir {:?}", std::env::current_dir().unwrap());

    // https://github.com/64bit/async-openai/issues/140
    let transcribe_client = client.clone();
    let transcribe_task = tokio::spawn(async move { transcribe(transcribe_client).await });

    // ok
    let chat_client = client.clone();
    let chat_task = tokio::spawn(async move { chat(chat_client).await });

    let result = tokio::join!(transcribe_task, chat_task);
    println!("chat: {}", result.1.unwrap().unwrap());
    println!("\n\n");
    println!("transcribe: {:?}", result.0);
}

async fn transcribe(client: Client<OpenAIConfig>) -> Result<String, OpenAIError> {
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "../examples/audio-transcribe/audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3"
        )
        .model("whisper-1")
        .response_format(async_openai::types::AudioResponseFormat::VerboseJson)
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{:?}", response);

    Ok(response.text)
}

async fn chat(client: Client<OpenAIConfig>) -> Result<String, OpenAIError> {
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

    let mut sb: Vec<String> = Vec::new();
    sb.push("\nResponse:".to_string());
    for choice in response.choices {
        sb.push(format!(
            "    {}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        ));
    }

    Ok(sb.join("\n"))
}
