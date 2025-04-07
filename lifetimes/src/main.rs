// if you have a function that takes in 2 or more refs and return a ref
// Rust assumes that the return ref will point to data referred to 
// by one of the arguments
// But Rust cannot tell which one you'll be returning
// this is because Rust cannot tell if the first or second arg will/not go out of scope
// eg . fn next_language(languages: &[String], current: &str) -> &str {

// So we need to add lifetime annotations
// <'a> "a" is just a placeholder, you can put anything u want
// put &'a first ref and return type are of type a
// it's to tell Rust the return type is same from languages
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

// You can still add annotations but not required
// Also called Elision or Elide ( meaning to omit )
// - functions that takes one ref
// - functions that takes one ref and any number of values(not ref)
// - functions that take &self and any number of refs 
//   Rust assumes for the returned ref will point at &self
//   if you want to refer to other ref then you have to add lifetime annotations
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

// If you dont know beforehand which you're going to return 
// put 'a for all the arguments
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages =vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    // result is a ref to a value in languages
    let result = next_language(&languages, "go");
    println!("{}", result);

    let result = last_language(&languages);
    println!("{}", result);

    let result = longest_language("typescript", "go");
    println!("{}", result);
} // languages out of scope when main ends
