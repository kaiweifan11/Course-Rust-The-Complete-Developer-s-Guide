#[derive(Debug)]
// enums provide some reusability to the codes we write
// eg. we can add a fn print_media and pass in any of the book, movie or audiobook
pub enum Media {
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
    pub fn description(&self) -> String {
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