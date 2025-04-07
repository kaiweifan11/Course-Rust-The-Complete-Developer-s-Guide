#[derive(Debug)]
// enums provide some reusability to the codes we write
// eg. we can add a fn print_media and pass in any of the book, movie or audiobook

enum Media {
    // variances
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String  },

    // it is possible to not add keys 
    // Podcast { episode_number: u32},
    Podcast(u32),
    Placeholder // does not have any data tied to it
}

impl Media {
    fn description(&self) -> String {
        // we need to figure what media is, Book? Movie? AudioBook
        // if not Rust will not allow us to access it's properties, even if it's the same for all 3 eg. title
        
        // one way is to use if else to check the type
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self  {
        //     format!("Movie {} {}", title, director)
        // } else if let Media::Audiobook { title } = self  {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        // match is like switch case
        // used for enums
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            // because episode_number was not declared in the enum, you can call it anything
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }

        // use enums when all the impl and functions are similar
        // sometimes we can decide to use struct because listing all params in the match is just too long 
        // eg. Media::Book { title, author, isbn, pages, bookmarks, words, paragraphs, .... } 
    }   
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            // Rust doesnt have null or undefined 
            // so we need to return Option
            MightHaveAValue::NoValueAvailable
        }
        
    }

    fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            // Rust doesnt have null or undefined 
            // so we need to return Option
            None
        }
        
    }
}

#[derive(Debug)]
// <'a> is lifetime annotation
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}

fn print_media(media: Media) {
    println!("{:#?}", media);
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
    let item = catalog.get_by_index(40);
    println!("{:#?}", item);

    match catalog.get_by_index(110) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("Nothing at that index");
        }
    }

    // pattern matching
    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(99990) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value!!!!!!!!!!");
    }

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
