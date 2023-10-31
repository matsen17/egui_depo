use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::reader::Reader;

pub trait DepositoryItem {
    fn get_uuid(&self) -> &Uuid;
    fn get_isbn_code(&self) -> &str; 
    fn get_title(&self) -> &str;
    fn get_year(&self) -> i32;
}

impl DepositoryItem for Book {
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn get_title(&self) -> &str {
        &self.title
    }
    fn get_year(&self) -> i32 {
        self.year  
    }
    fn get_isbn_code(&self) -> &str {
        &self.isbn_code
    }
}

impl DepositoryItem for Magazine {
    fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    fn get_title(&self) -> &str {
        &self.title
    }
    fn get_year(&self) -> i32 {
        self.year   
    }
    fn get_isbn_code(&self) -> &str {
        &self.isbn_code
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub uuid: Uuid,
    pub isbn_code: String,
    pub title: String,
    pub year: i32
}

impl Book {
    pub fn new(isbn_code: String, title: String, year: i32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            isbn_code,
            title,
            year,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub uuid: Uuid,
    pub isbn_code: String,
    pub title: String,
    pub year: i32,
    pub super_hero: String
}

impl Magazine {
    pub fn new(isbn_code: String, title: String, year: i32, super_hero: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            isbn_code,
            title,
            year,
            super_hero,
        }
    }
}