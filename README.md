# aws-tts-translator

## Build
1. Install [rust](https://www.rust-lang.org/tools/install)
2. `cargo build --release`

## Run
1. Login [aws cli](https://aws.amazon.com/cli/)
2. Make sure  you have permissions to use following services in the cli
    - [translate](https://aws.amazon.com/translate/)
    - [polly](https://aws.amazon.com/polly/)
3. `cargo run --release -- --help`
4. `cargo run --release --`

