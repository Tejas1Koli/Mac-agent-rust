
use rig::providers::ollama;
use rig::client::{CompletionClient};
use rig::completion::{Chat, Message};

mod tool;
use tool::AppleScriptTool;

mod config;
use crate::config::Config;
//use rig::completion::Prompt;
mod preamble;
use preamble::PREAMBLE;




#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let config = Config::from_env();
    let client = ollama::Client::new(config.ollama_host.as_str());
    let preamble = format!("{}", PREAMBLE);
   

    let agent = client?
        .agent(config.model.as_str())                      // change to gemma2:4b if needed
        .preamble(&preamble)
        .default_max_turns(10)
        .tool(AppleScriptTool)
        .build();

    println!("\
    ┌─────────────────────────────────────────────────────┐
    │           macOS Agent  •  by Tejas Koli             │
    │                                                     │
    │  Controls your system via AppleScript.              │
    │  Under development — use with caution.              │
    │  github - Tejas1Koli                                │
    └─────────────────────────────────────────────────────┘
    ");

    println!("macOS agent ready (gemma4:e2b via Ollama)");
    println!("Type a prompt, or 'quit' to exit\n");

    let mut history: Vec<Message> = Vec::new();

    loop {
    let mut input = String::new();
    
    print!("User: ");
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut input)?;

    let task = input.trim();
    match task {
        "quit" | "exit" => break,
        "reset" => { history.clear(); println!("History cleared.\n"); continue; }
        "" => continue,
        _ => {}
    }

    match agent.chat(task, &mut history ).await {
        Ok(response) => {
            println!("Agent: {}\n", response);
            history.push(Message::user(task));
            history.push(Message::assistant(&response));

            print!("History size: {}\n", history.len());
        }
        Err(e) => {
            println!("Error: {:?}\n", e);
            history.push(Message::user(task));
            print!("History size: {}\n", history.len());
        }
        

        }
    }

    Ok(())
}