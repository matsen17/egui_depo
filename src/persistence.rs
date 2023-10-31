use std::ops::Deref;
use std::rc::Rc;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::book::Magazine;
use crate::book::Book;
use crate::book_depository::Item;
use crate::reader::Reader;

#[derive(Serialize, Deserialize)]
struct DataToSave {
    depository_books: Vec<Book>,
    depository_magazines: Vec<Magazine>,
    borrows: Vec<(Uuid, Uuid)>,
    readers: Vec<Reader>,
}

pub fn save_to_file(
    filename: &str,
    depository_books: &Vec<Rc<Book>>,
    depository_magazines: &Vec<Rc<Magazine>>,
    catalogue: &Vec<Item>,
    readers: &Vec<Reader>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = DataToSave {
        depository_books: depository_books.iter().map(|b| {(b.deref()).clone()}).collect(),
        depository_magazines: depository_magazines.iter().map(|m| {(m.deref()).clone()}).collect(),
        borrows: catalogue.iter().filter_map(|i| {
            if let Some(r) = i.reader {
                Some((i.item.get_uuid().clone(), r.clone()))
            }else{
                None
            }
        }).collect(),
        readers: readers.to_vec(),
    };

    let json = serde_json::to_string(&data)?;
    std::fs::write(filename, json)?;
    Ok(())
}

pub fn load_from_file(
    filename: &str,
) -> Result<(Vec<Book>, Vec<Magazine>, Vec<Reader>, Vec<(Uuid,Uuid)>), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(filename)?;
    let data: DataToSave = serde_json::from_str(&json)?;
    Ok((data.depository_books, data.depository_magazines, data.readers, data.borrows))
}