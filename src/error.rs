use std::error::Error as StdError;

use aws_sdk_translate::types::SdkError;

#[derive(thiserror::Error, Debug)]
#[error("unhandled error")]
pub struct Error {
    #[from]
    source: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    pub fn unhandled(source: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

impl<T> From<SdkError<T>> for Error
where
    T: StdError + Send + Sync + 'static,
{
    fn from(source: SdkError<T>) -> Self {
        // TODO: Catch permission errors

        dbg!(&source);

        Self::unhandled(source)
    }
}

impl From<aws_sdk_translate::Error> for Error {
    fn from(source: aws_sdk_translate::Error) -> Self {
        Self::unhandled(source)
    }
}

impl From<aws_sdk_polly::Error> for Error {
    fn from(source: aws_sdk_polly::Error) -> Self {
        Self::unhandled(source)
    }
}
