use std::{fs::File, io::Read};

mod droid;

fn main() {
    let mut file = File::open("src/droid.rs").expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    for (i, item) in syntax.items.iter().enumerate() {
        println!("{}", i);
        println!("{:?}", item);
    }
    // Debug impl is available if Syn is built with "extra-traits" feature.
    // println!("{:#?}", syntax);
}
