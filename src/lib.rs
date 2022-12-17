use std::{str::FromStr, path::PathBuf};

use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;
use bytes::Bytes;
use aws_sdk_polly::{
    model::{Engine as PollyEngine, Gender as PollyGender, LanguageCode, OutputFormat},
    Client as PollyClient,
};
use aws_sdk_translate::Client as TranslateClient;
use clap::ValueEnum;

pub use error::Error;

pub mod error;

#[derive(Debug, Clone, ValueEnum)]
pub enum Gender {
    Female,
    Male,
}

impl From<Gender> for PollyGender {
    fn from(val: Gender) -> Self {
        match val {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Engine {
    Neural,
    Standard,
}

impl From<Engine> for PollyEngine {
    fn from(val: Engine) -> Self {
        match val {
            Engine::Neural => Self::Neural,
            Engine::Standard => Self::Standard,
        }
    }
}

pub async fn translate(
    client: &TranslateClient,
    language_code: String,
    text: String,
) -> Result<Option<String>, Error> {
    //return Ok(Some(String::from(
    //    "Hallo! Mijn naam is Joanna. Ik lees elke tekst die je hier typt.",
    //)));

    let result = client
        .translate_text()
        .set_text(Some(text))
        .set_source_language_code(Some("auto".into()))
        .set_target_language_code(Some(language_code))
        .send()
        .await?;

    let result = result.translated_text().map(|result| result.into());

    Ok(result)
}

pub async fn text_to_speech(
    client: &PollyClient,
    language_code: String,
    name: Option<String>,
    gender: Option<Gender>,
    engine: Option<Engine>,
    text: String,
    output_file: PathBuf,
) -> Result<(), Error> {
    let language_code = LanguageCode::from_str(&language_code).unwrap();

    let mut voices_builder = client
        .describe_voices()
        .set_language_code(Some(language_code.clone()));

    if let Some(engine) = engine {
        voices_builder = voices_builder.engine(engine.into());
    }

    let voices = voices_builder.send().await?;

    let voice = if let Some(voices) = voices.voices() {
        let search_name = name.as_deref();

        let tmp: PollyGender;
        let search_gender = if let Some(gender) = gender {
            tmp = gender.into();
            Some(&tmp)
        } else {
            None
        };

        let mut ret_voice = None;
        for voice in voices {
            if voice.name() == search_name && voice.gender() == search_gender {
                ret_voice = Some(voice);
                break;
            }
        }

        ret_voice
    } else {
        None
    };

    if let Some(voice) = voice {
        if let Some(voice_id) = voice.id() {
            let ret = client
                .synthesize_speech()
                .set_text(Some(text))
                .set_language_code(Some(language_code))
                .output_format(OutputFormat::Mp3)
                .voice_id(voice_id.clone())
                .send()
                .await?;

            let mut file = File::create(output_file).await.expect("Unable to output file");
            let mut stream = ret.audio_stream;
            while let Some(bytes) = stream.next().await {
                let bytes: Bytes = bytes.map_err(Error::unhandled)?;
                file.write_all(&bytes).await.map_err(Error::unhandled)?;
            }

            file.flush().await.map_err(Error::unhandled)?;

        }
    }

    Ok(())
}

