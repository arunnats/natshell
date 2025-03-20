use anyhow::anyhow; // maco that creates an anhow:Error from a string or another error

// define and enum
#[derive(Clone, Debug)] // clone allows enum to be duplicated
pub enum Command {
    Exit,
    Echo(String),
    Ls,
    Pwd,
    Cd(String),
    Touch(String),
    Rm(String),
    Cat(String),
}

// used to convert string to Command enum
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> { // takes string value and returns itself if successful, returns anyhow:Error if invalid
        let split_value: Vec<&str> = value.split_whitespace().collect(); // split by whitespace

        // match the first word
        match split_value[0] {
            "exit" => Ok(Command::Exit), // convert to Command:Exit
            "ls" => Ok(Command::Ls), // convert to Command:Ls
            "echo" => {
                if split_value.len() < 2 { // check if more than 1 word in string, ie if input is given to echo
                    Err(anyhow!("echo command requires an argument"))
                } else {
                    Ok(Command::Echo(split_value[1..].join(" ")))
                }
            }
            "pwd" => Ok(Command::Pwd), // convert to Command:PWd
            "cd" => {
                if split_value.len() < 2 { // check if more than 1 word in string, ie if argument is present
                    Err(anyhow!("cd command requires an argument"))
                } else {
                    Ok(Command::Cd(split_value[1..].join(" ")))
                }
            }
            "touch" => {
                if split_value.len() < 2 { // check if more than 1 word in string, ie if argument is present
                    Err(anyhow!("touch command requires an argument"))
                } else {
                    Ok(Command::Touch(split_value[1..].join(" ")))
                }
            }
            "rm" => {
                if split_value.len() < 2 { // check if more than 1 word in string, ie if argument is present
                    Err(anyhow!("rm command requires an argument"))
                } else {
                    Ok(Command::Rm(split_value[1..].join(" ")))
                }
            }
            "cat" => { 
                println!("{}", split_value[1..].join(" "));
                if split_value.len() < 2 { // check if more than 1 word in string, ie if argument is present
                    Err(anyhow!("cat command requires an argument"))
                } else {
                    Ok(Command::Cat(split_value[1..].join(" ")))
                }
            }
            _ => Err(anyhow!("Unknown command")), // unknown command
        }
    }
}