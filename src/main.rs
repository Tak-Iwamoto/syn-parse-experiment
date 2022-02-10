use std::{fs::File, io::Read};

use syn::ext::IdentExt;

mod droid;

fn main() {
    let mut file = File::open("src/droid.rs").expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    for item in syntax.items.iter() {
        if let syn::Item::Struct(st_item) = &item {
            for field in &st_item.fields {
                let ident = field.ident.clone().unwrap().unraw().to_string();
                println!("{}", &ident);
                if let syn::Type::Path(ty_path) = &field.ty {
                    let path_segment = ty_path.path.segments.first().unwrap();
                    let ident = path_segment.ident.unraw().to_string();
                    println!("{}", ident);
                }
            }
        }
    }
}
