use std::path::PathBuf;

use tts_translator::{text_to_speech, translate, Engine, Error, Gender};

use aws_config::retry::RetryConfig;
use aws_sdk_polly::Client as PollyClient;
use aws_sdk_translate::Client as TranslateClient;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Language code to translate to.
    ///
    /// Supported language codes can be found here: https://docs.aws.amazon.com/polly/latest/dg/SupportedLanguage.html
    #[arg(short, long, default_value = "nl-NL")]
    language_code: String,

    /// Text to be translated
    #[arg(short, long, default_value = "Hello, World!")]
    text: String,

    /// Name of the tts voice
    ///
    /// If the voice is not available for the selected `language_code`/`engine` the program will crash
    ///
    /// Supported voices can be found here: https://docs.aws.amazon.com/polly/latest/dg/voicelist.html
    #[arg(short, long)]
    name: Option<String>,

    /// Gender of the tts voice
    ///
    /// If the gender is not available for the selected `language_code`/`engine` the program will crash
    ///
    /// Supported combinations can be found here: https://docs.aws.amazon.com/polly/latest/dg/voicelist.html
    #[arg(value_enum, short, long)]
    gender: Option<Gender>,

    /// Engine of the tts voice
    ///
    /// If combination of parameters is not possible the program will crash
    ///
    /// Supported combinations can be found here: https://docs.aws.amazon.com/polly/latest/dg/voicelist.html
    #[arg(value_enum, short, long, default_value = "neural")]
    engine: Option<Engine>,

    /// File to output the voice recording (in mp3 format)
    #[arg(short, long, default_value = "./output.mp3")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let shared_config = aws_config::from_env()
        .retry_config(RetryConfig::standard().with_max_attempts(2))
        .load()
        .await;

    let translate_client = TranslateClient::new(&shared_config);
    let polly_client = PollyClient::new(&shared_config);

    if let Some(translation) =
        translate(&translate_client, args.language_code.clone(), args.text).await?
    {
        println!("{}", &translation);

        text_to_speech(
            &polly_client,
            args.language_code,
            args.name,
            args.gender,
            args.engine,
            translation,
            args.output_file,
        )
        .await?;
    }

    Ok(())
}
