use futures::StreamExt;
use rust_xp_ollama::{consts::{DEFAULT_SYSTEM_MOCK, MODEL}, gen::gen_stream_print, Result};

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() -> Result<()> {
    // By default localhost:11434
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (Be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt)
        .system(DEFAULT_SYSTEM_MOCK.to_string());

    // let res = ollama.generate(gen_req).await?;
    // println!("->> res {}", res.response);

    gen_stream_print(&ollama, gen_req).await?;

    Ok(())
}