use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}
// Remember to add all fn as pub too
impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    // pub fn get_by_index(&self, index: usize) -> MightHaveAValue {
    //     if self.items.len() > index {
    //         MightHaveAValue::ThereIsAValue(&self.items[index])
    //     } else {
    //         // Rust doesnt have null or undefined 
    //         // so we need to return Option
    //         MightHaveAValue::NoValueAvailable
    //     }
        
    // }

    pub fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            // Rust doesnt have null or undefined 
            // so we need to return Option
            None
        }
        
    }
}