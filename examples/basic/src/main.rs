#![allow(dead_code)]

use camo::Camo;

#[derive(Camo)]
pub struct Book {
    title: String,
    author: String,
    chapters: Vec<Chapter>,
}

#[derive(Camo)]
pub struct Chapter {
    title: String,
    page_count: usize,
}

fn main() {
    let book = Book::camo();
    println!("{:#?}", book);
}
