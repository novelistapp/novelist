extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod state;
use state::Novel;


fn main() {
    Novel::create(".", "Starlike", "Katharina Ariane").unwrap();
}
