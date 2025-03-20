mod errors;

use errors::CrateResult;
use tokio::{ // tokio is an asyc runtime module
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};

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
            // log user input
            println!("User entered: {}", line);
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
