// library for reading files
use std::fs;
use std::io::Error;

// Weird strings
// fn string_test(
//     a: String,
//     b: &String,
//     c: &str // string slice
// ) { } 

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

fn extract_errors2(text: &str) -> Vec<String> {
    // NOTE: split in Rust returns a vec of str slices
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    // match divide(5.0, 0.0) {
    //     Ok(result_of_division) => {
    //         println!("{}", result_of_division)
    //     }
    //     Err(what_went_wrong) => {
    //         println!("{}", what_went_wrong)
    //     }
    // }

    // match validate_email(String::from("asd")) {
    //     // .. means there is some variable in the return but we dont care about it
    //     Ok(..) => println!("email is valid"),
    //     Err(reason_this_failed_validation) => {
    //         println!("{}", reason_this_failed_validation)
    //     }
    // }

    let mut error_logs = vec![];

   match fs::read_to_string("logs.txt") {
    Ok(text_that_was_read) => {
        // println!("{}", text_that_was_read.len())
        // error_logs = extract_errors(&text_that_was_read); // causes a compile error
        // println!("{:#?}", error_logs);
        error_logs = extract_errors2(&text_that_was_read);
        
    } 
    // text_that_was_read goes out of scope
    // Its value will be dropped
    // error_logs still contains refs 
    // but the refs pointed to values in text_that_was_read 
    // which are now gone
    
    Err(why_this_failed) => {
        println!("Failed to read file: {}", why_this_failed)
    }
   }
   // causes a compile error because 
   // error_logs now point to text_that_was_read 
   // that no longer has value
   println!("{:#?}", error_logs); 

//    string_test(
//         //String::from("red"),             // a
//         "red".to_string(),              // same as a
//         &String::from("red"), 
//         //"red"                            // c
//         String::from("red").as_str()    // same as c    
//     );
   
}

// sometimes we have nothing to return in Ok 
// in such cases we can return a empty tuple ()
// fn validate_email(email: String) -> Result<(), Error>{
//     if email.contains("@") {
//         // Success!
//         Ok(())
//     } else {
//         Err(Error::other("emails must have an @"))
//     }

// }

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         Err(Error::other("cant divide by 0"))
//     } else {
//         Ok(a / b)
//     }
// }