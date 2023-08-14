use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};
use crate::position::{BoardPosition, Position};

const BOOK_BIN: &'static [u8] = include_bytes!("book/book.bin");

#[derive(Serialize, Deserialize)]
pub struct OpeningBook {
    small: OpeningBookHelper,
    big: OpeningBookHelper,
}

impl OpeningBook {
    pub fn new() -> Self {
        OpeningBook::load_from_file().unwrap()
    }

    pub fn get(&self, pos: &BoardPosition) -> Option<i8> {
        if pos.get_moves_played() <= 5 {
            return self.small.get(pos);
        }
        if pos.get_moves_played() <= 7 {
            return self.big.get(pos);
        }
        None
    }

    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let book = bincode::deserialize(BOOK_BIN).expect("Failed to deserialize");
        Ok(book)
    }
}


#[derive(Serialize, Deserialize)]
struct OpeningBookHelper {
    data: HashMap<u64, i8>,
}

impl OpeningBookHelper {
    pub(crate) fn new(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Unable to open the file");
        let reader = BufReader::new(file);
        let mut data = HashMap::new();

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let parts: Vec<&str> = line.split(":").collect();
            let key = BoardPosition::from_str(parts[0]).unwrap().canonical_key();
            let value = parts[1].parse::<i8>().expect("Error parsing i8");
            data.insert(key, value);
        }

        OpeningBookHelper {
            data
        }
    }


    pub fn get(&self, pos: &BoardPosition) -> Option<i8> {
        if pos.get_moves_played() == 0 {
            return Some(1);
        }
        let key = pos.canonical_key();
        self.data.get(&key).map(|v| *v)
    }
}

