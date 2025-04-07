// official online editor for Rust
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4a1210e67ec69e8c2ee760ca9eb2fe53

// define the namespace from imported crates so we dont have to keep typing :: in the main fn
// we can also just use rand::thread_rng use rand::seq::SliceRandom but we use the below shortcut
use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] // add attributes(derive) trait(Debug) to the struct 
struct Deck {
    // Vec (Vectors) are like arrays but can grow or shrink in size
    // Rust also has arrays but they have fixed lengths
    cards: Vec<String>,
}

// inherent implementations = class methods
impl Deck {
    // constructor
    // -> return type (i.e. Deck type)
    //fn new() -> Deck {
    fn new() -> Self { // returns the type mentioned in the impl block
        // List of "suits" - "hearts", "spades", ...
        // List of values - "ace", "two", ...
        let suits = vec!["Hearts", "Spades", "Diamonds"];
        let values = vec!["Ace", "Two", "Three"];

        // variables(bindings) are immutable by default, use mut
        // let cards = vec![];
        let mut cards = vec![];

        /* Arrays are only slightly more performant. 
        We only use array when we are sure it's not going to grow or shrink */

        // double nested loops
        for suit in suits {
            for value in values.iter() {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck declares a new binding(variable)
        // : Deck decalres it's type Deck
        // Deck { cards: vec![] } creates an instance of a struct (struct literal)
        // can also create an empty vector by using Vec::new()
        let deck: Deck = Deck { cards };
        return deck;
        // return statements can also be written as 
        // implicit return ( no need to type return )
        // must be last line and remove ";"
        // Deck { cards }
    }

    // method
    // must mark self as mutable also since we are mutating the original param
    fn shuffle(&mut self) {
        // crate == package
        // external crates have to be added using cargo add
        // crate listing at crates.io
        // documenation in docs.rs
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }


    // 3 types of numbers in Rust
    // signed - i8, ..., isize
    // unsigned - u8, ..., usize
    // floats - f32, f64
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }

    /*
        Associated functions vs methods
        - associated functions do not take the &self as first param. 
        - associated functions usually used in initialising the struct
        eg. full_deck(), with_n_cards(10), empty_deck()
        - methods are just class methods and must take in &self as first param
     */
}

fn main() {
    // variables(bindings) are immutable by default, use mut
    let mut deck = Deck::new();

    //deck.shuffle();

    let cards = deck.deal(3);
    println!("here's your hand: {:#?}", cards );

    // ! indicates macro
    // println!("Hello, world!");

    // use {} as placeholders and pass in 2nd argument deck 
    // can also use formatters eg. {:?} (debug) or {:.3} to format to 3 decimal places
    // or can do println!("Here's your deck: {deck}");
    // # helps printing in a more readable format
    println!("Here's your deck: {:#?}", deck);
}
