// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

use packages_crates_modules::eat_at_restaurant;
use std::collections::HashMap; 
use std::fmt::Result;
use std::io::Result as IoResult; //  as keyword allows us to bring in two items with the same name by renaming one of the items.
// While bringing in strcuts, enums, and other items with use, its idiomatic to specify the full path to the item. This makes it easier to tell where the item is defined.
use std::collections::*; // The glob operator * brings all public items defined in the collections module into the current scope. This is useful when you want to bring many items into scope and you don't want to list them all out manually.

use crate::garden::vegetables::Asparagus;

pub mod garden;
fn main( ) {
    let plant = Asparagus {};
println!("Im growing {plant:?}!");

eat_at_restaurant();

let mut map = HashMap::new();
map.insert(1,2);
println!("{:?}", map);
}

fn function() -> Result {
    Ok(()) // Ok is a variant of the Result enum that indicates the function ran successfully.
} // The Result type is a generic type, so we need to specify the type of the value returned in the Ok variant.

// fn function2() -> io::Result<()> {
//     Ok(())
// } // Same here as well, the io::Result type is a generic type, so we need to specify the type of the value returned in the Ok variant.

fn function3() -> IoResult<()> {
    Ok(())
} // This is the same as the previous function, but we've used the as keyword to rename the Result type from the io module to IoResult. This makes it clear that this Result type is from the io module.