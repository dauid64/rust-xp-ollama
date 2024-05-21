use futures::StreamExt;
use rust_xp_ollama::{consts::{DEFAULT_SYSTEM_MOCK, MODEL}, Result};

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

pub async fn gen_stream_print(
    ollama: &Ollama,
    gen_req: GenerationRequest
) -> Result<()> {
    let mut stream = ollama.generate_stream(gen_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;

    let mut final_data_responses = Vec::new();

    while let Some(res) = stream.next().await {
        let res_list = res?;

        for res in res_list {
            let bytes = res.response.as_bytes();

            char_count += bytes.len();
            if char_count > 80 {
                stdout.write_all(b"\n").await?;
                char_count = 0;
            }

            stdout.write_all(bytes).await?;
            stdout.flush().await?;

            if let Some(final_data) = res.final_data {
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                final_data_responses.push(final_data);
                break;
            }
        }
    }

    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    
    Ok(())
}