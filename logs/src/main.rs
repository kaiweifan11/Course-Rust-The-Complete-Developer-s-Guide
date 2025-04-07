// library for reading files
use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    // NOTE: split in Rust returns a vec of str slices
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

// 3rd way to handle Result enum (i.e. errors)

fn main() -> Result<(), Error> {
    // Ok(())
    // Err(Error::other("something went wrong"))

    // ? Operator AKA Try Operator
    // - if the return value is Ok('val'), 
    // it auto unwraps and assign 'val' to text
    // - if the return value is Err(Error::other("something went wrong")),
    // it auto unwraps and return the error early 
    // and the rest of the codes will not be run 
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("/hgcgcc/errors.txt", error_logs.join("\n"))?;

    // if err, this line and below will not run. err will return early from main
    println!("{}", text.len()); 

    Ok(()) // main expects to return a Result, set a default Ok
}
