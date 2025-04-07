fn print_elements(elements: &Vec<String>) {
    // there are 2 ways to loop for an iter
    
    // option 1 use a for loop
    // for loops automatically creates an iterator
    // automatically unwrap the Option that comes back
    // Breaks once 'next' returns None
    println!("For Loop");
    for element in elements {
        println!("{}", element);
    }

    // option 2 use adapters/consumers like for_each or map
    // || is to denote a closure
    // it's like inline functions in javascript
    // for_each is a consumer
    println!("Iter for_each");
    elements.iter().for_each(|el| {
        println!("{}", el);
    });

    // iterator adapters
    println!("Iter map");
    elements
        .iter()
        // use .map to change each "el" into "el el" 
        // .map is just a processing step, does not call iterations
        // not the same as javascript
        .map(|el| format!("{} {}", el, el))
         // if you just call .map without for_each
        // you will get a warning 
        // because iter adapters are "lazy" and will not move next until
        // you call next or a consumer calls next
        .for_each(|el| {
            println!("{}", el);
        }
    );
}

// modify the original vector in place
//fn shorten_strings(elements: &mut Vec<String>) {
fn shorten_strings(elements: &mut [String]) { // can also use vector slice
    // iter returns a readonly ref to elements
    elements
        //.iter() // causes error because el is readonly, use iter_mut instead
        .iter_mut()
        .for_each(|el| el.truncate(1)); 

}

// create a new vector
fn to_uppercase(elements: &[String]) -> Vec<String> {
    // .collect returns a vector of all the elements
    // .collect is a consumer
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect() 

    // .collect can be used for vectors, hashmap, doubly linked list
    // because at the top of the function we declare return type as Vec<String>
    // collect knows to return Vec<String>
    // another way to make sure is to add the type for collect
    // eg. .collect::<Vec<String>>()
    // or if you want collect to infer the type
    // eg. .collect::<Vec<_>>() // this works because .map returns String
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    // into_iter can also be used as iter whwn u pass &vec_a
    // into_iter can also be used as iter_mut when u pass a &mut vec_a
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>>{
    elements
        .iter()
        .map(
            // inner vector
            |el| el.chars() // chars return an iterator with each char
                .map(|c| c.to_string()) // chain another .map 
                .collect()
        )
        .collect() // finally collect into the outer vector
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        // .find is iterator consumer
        // returns an Option Some(ref value) if found and None if not found
        .find(|el| el.contains(search)) 
        // .map_or belongs to Option enum
        // if Option is None, returns default(first) value
        // if Option is Some, take the value out and run through closure(function)
        .map_or(
            String::from(fallback), // default 
            |el| el.to_string()) // el is a ref so need to convert to String

}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    // pointers in iter changes when we call next()
    // thats why we need to add mut
    // let mut colors_iter = colors.iter();

    // println!("{:#?}", colors_iter.next()); // next() returns Some(value)
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next()); // last one will return None

    // this still works after calling iter 
    // because iter is a different struct from colors
    // println!("{:#?}", colors);

    //print_elements(&colors);

    // shorten_strings(&mut colors);
    // println!("{:#?}", colors);

    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);

    // let mut destination =vec![];
    // move_elements(colors, &mut destination);
    // println!("{:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let found_color = find_color_or(
        &colors,
        "re",
        "Orange"
    );
    println!("{:#?}", found_color);

}

// Side note: Vector slices
// similar to &str string slices 
// vector slice can take in a subset of the original array
// it's more perfomant than actually creating a new struct with the subset values
// to use vector slice, in the params of calling function change it to 
// &[String] instead of &Vec[String] 
// &[String] is more flexible and can take in partial/full vector
// while &Vec[String] must take in full vector