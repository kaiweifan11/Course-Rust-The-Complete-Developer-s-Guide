// library for reading files
use std::fs;

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

fn main() {
    /* too many match */
    // match fs::read_to_string("logs.txt") {
    //     Ok(text_that_was_read) => {
    //         let error_logs = extract_errors(&text_that_was_read);
    //         // write to a file
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(reason_write_failed) =>{
    //                 println!("Writing of errors.txt failed: {}", reason_write_failed);
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read file: {}", why_this_failed)
    //     }
    // }

    // alternative to condense the above code by using .expect
    let text = fs::read_to_string("logs.txt")
        .expect("failed to read logs.txt");

    let error_logs = extract_errors(&text);

    fs::write("errors.txt", error_logs.join("\n"))
        .expect("failed to write errors.txt");

}
