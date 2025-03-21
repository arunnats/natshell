use errors::CrateResult;
use tokio::{ // tokio is an asyc runtime module
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};
use command::Command;
use crate::helpers::pwd;

mod errors;
mod command;
mod helpers;

async fn handle_new_line(line: &str) -> CrateResult<Command> {
    // leverages the TryFrom trait from command.rs
    let command: Command = line.try_into()?;

    // match the commands here
    match command.clone() {
        Command::Exit => println!("byeeee"),
        Command::Echo(s) => println!("{}", s),
        Command::Ls => helpers::ls()?,
        Command::Pwd => println!("{}", helpers::pwd()?),
        Command::Cd(s) => helpers::cd(&s)?,
        Command::Touch(s) => helpers::touch(&s)?,
        Command::Rm(s) => helpers::rm(&s)?,
        Command::Cat(s) => {
            let contents = helpers::cat(&s)?;
            println!("{}", contents);
        }
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
        stdout.write(b"Welcome to Natshell!\n").await?;
        stdout.write(pwd()?.as_bytes()).await?;
        stdout.write(b"\nNatshell >").await?;
        stdout.flush().await?; // ensure output is shown immediately
        
        while let Ok(Some(line)) = reader.next_line().await { // continuously read user input
            let command = handle_new_line(&line).await;

            if let Ok(command) = &command { // if the command matched the enum
                match command {
                    Command::Exit => {
                        break;
                    }
                    _ => {}
                }
            } else {
                eprintln!("Error parsing command: {}", command.err().unwrap()); // else error
            }

            stdout.write_all(b"\nNatshell > ").await?;
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
