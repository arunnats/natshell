use std::fs;
use crate::errors::CrateResult;

// returns the current working directory as a string
pub fn pwd() -> CrateResult<String> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.display().to_string())
}

// lists the contents of the current directory
pub fn ls() -> CrateResult<()> {
    let entries = fs::read_dir(".")?; // read the current directory
    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy()); // print each entry
    }
    Ok(())
}

// changes the current working directory to the specified path
pub fn cd(path: &str) -> CrateResult<()> {
    std::env::set_current_dir(path)?; // change directory
    Ok(())
}

// creates a new empty file at the specified path
pub fn touch(path: &str) -> CrateResult<()> {
    fs::File::create(path)?; // Create the file
    Ok(())
}

// removes the file at the specified path
pub fn rm(path: &str) -> CrateResult<()> {
    fs::remove_file(path)?; // Remove the file
    Ok(())
}

// reads and returns the contents of the file at the specified path
pub fn cat(path: &str) -> CrateResult<String> {
    let pwd = pwd()?; // get the current working directory
    let joined_path = std::path::Path::new(&pwd).join(path); // vreate the full path
    let contents = fs::read_to_string(joined_path)?; // read the file contents
    Ok(contents)
}
