/*
    MODULES - 3 Ways to create modules
    1) Create mod inside existing file
    mod content {
        // By default anything you place inside the module is private
        // set to pub to access it from outside the module
        pub enum Media {} 
        /* Media impl */
        pub struct Catalog {}
        /* Catalog impl */
    }

    let catalog = content::Catalog::new();

    2) Create mod in a seperate file but in the same directory
    eg. inside src/content.rs
    pub enum Media {} 
    /* Media impl */
    pub struct Catalog {}
    /* Catalog impl */

    mod content /* inside src/main.rs */

    use content::Catalog
    
    let catalog = Catalog::new() 

    3) Folder structure with each enum/struct having it's own file
    This is most used in major projects
    eg. 
    /content/media.rs
        pub enum Media {} 
        /* Media impl */
    /content/catalog.rs
        pub struct Catalog {}
        /* Catalog impl */
    /content/mod.rs
        pub mod media // mod = import, pub = export ^_^'''
        pub mod catalog
    /main.rs
        mod content

        let catalog = content::catalog::Catalog::new()

*/
mod content;

use content::media::Media;
use content::catalog::Catalog;

#[derive(Debug)]
// <'a> is lifetime annotation
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}

fn main() {
    let audiobook = Media::Audiobook { title: String::from("An Audiobook") };
    let good_movie = Media::Movie { 
        director: String::from("Good Director"),
        title: String::from("Good Movie") 
    };
    let bad_book = Media::Book { 
        author: String::from("Bad Director"),
        title: String::from("Bad Book") 
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    // catalog.add(placeholder);


    // println!("{:#?}", catalog );
    // Vectors have this function .items.get()
    // println!("{:#?}", catalog.items.get(0) ); // prints some
    // println!("{:#?}", catalog.items.get(110) ); // prints none

    // Rust does not have null or undefined or void
    // instead Rust uses a built-in enum called "Option"
    // Option has 2 variants, "some" and "none"
    /*
        enum Option {
            Some(value),
            none
        }
     */
    // How to handle Option enum
    // match catalog.items.get(110) {
    //     // can use shortform Some(value) => {
    //     Option::Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     Option::None => {
    //         println!("Nothing at that index");
    //     }
    // }

    // we can create our own enum MightHaveAValue to handle 
    // let item = catalog.get_by_index(40);
    // println!("{:#?}", item);

    // match catalog.get_by_index(110) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("Nothing at that index");
    //     }
    // }

    // pattern matching
    // if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(99990) {
    //     println!("Item in pattern match: {:#?}", value)
    // } else {
    //     println!("No value!!!!!!!!!!");
    // }

    // With Option instead
    match catalog.get_by_index_with_option(110) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }

    if let Some(value) = catalog.get_by_index_with_option(99990) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value!!!!!!!!!!");
    }

    // Other ways to handle Options (Not highly recommended unless use case)
    // Unwrap
    // Not recommended to use
    let item = catalog.get_by_index_with_option(0);
    //println!("{:#?}", item.unwrap()); // but if item is None, you get panic

    // Expect 
    // Not ideal, use only when we WANT the app to crash when there's no value
    // displays error msg "expect there to be an item here!" 
    // when item is none
    //println!("{:#?}", item.expect("expect there to be an item here!"));

    // Unwrap_or
    // does not panic when item is none
    // instead returns a fallback value
    // println!("{:#?}", item.unwrap_or(&placeholder));

}
