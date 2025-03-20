use errors::CrateResult;
use tokio::{ // tokio is an asyc runtime module
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};
use command::Command;

mod errors;
mod command;

async fn handle_new_line(line: &str) -> CrateResult<Command> {
    // leverages the TryFrom trait from command.rs
    let command: Command = line.try_into()?;

    match command.clone() {
        // Placeholder
        _ => {}
    }
    Ok(command)
}


fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    // run the REPL loop in an async task
    tokio::spawn(async {
        // Initialize stdin and stdout
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let mut reader = tokio::io::BufReader::new(stdin).lines(); // a new buffererd reader to read input line by line
        let mut stdout = tokio::io::BufWriter::new(stdout); // buffered output for improved performance

        // output message and wait for user input
        stdout.write_all(b"Natshell > ").await?;
        stdout.flush().await?; // Ensure output is shown immediately
        
        while let Ok(Some(line)) = reader.next_line().await { // continuously read user input
            let command = handle_new_line(&line).await;

            if let Ok(command) = &command { // if the command matched the enum
                match command {
                    _ => {}
                }
            } else {
                eprintln!("Error parsing command: {}", command.err().unwrap()); // else error
            }

            stdout.write_all(b"Natshell > ").await?;
            stdout.flush().await?; // Ensure output is shown immediately
        }
        Ok(())
    })
}

// we need to make main async to run the REPL but to make main async, we use tokio
#[tokio::main]
async fn main() {
    let user_input_handler = spawn_user_input_handler().await;

    // log error
    if let Ok(Err(e)) = user_input_handler {
        eprintln!("Error: {}", e);
    }
}
