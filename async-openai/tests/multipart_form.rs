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

    // https://github.com/64bit/async-openai/issues/140
    let transcribe_task = tokio::spawn(async move {
        transcribe(client).await
    });

    let result = tokio::join!(transcribe_task);
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