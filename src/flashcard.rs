#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub struct Deck {
    pub cols: u8,
    pub content: Vec<String>,
    pub vars: HashMap<String,String>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cols: 0,
               content: Vec::new(),
               vars: HashMap::new(),
        }
    }

    pub fn new_from_file(file: &str) -> Option<Deck> {
        let d = match load_deck_from_file(file) {
            Ok(deck) => Some(deck),
            Err(s) => {
                println!("Deck::new_from_file() failed:\n   {}", s);
                None
            },
        };
        d
    }

    pub fn clear(&mut self) {
        self.cols = 0;
        self.content.clear();
        self.vars.clear();
    }

    pub fn get_card(&self, ind: usize) -> Option<Vec<String>> {
        if ind >= self.content.len() {return None;}
        let s:Vec<&str> = self.content[ind].split(':').collect();

        let thing = s.iter()
                     .map(|ss| ss.to_string())
                     .collect::<Vec<String>>();
        Some(thing)
    }

    pub fn get_card_data(&self, ind: usize) -> Option<String> {
        if ind >= self.content.len() {return None;}
        let r = &self.content[ind];
        Some(r.to_string())
    }
}

pub fn load_deck_from_file(file: &str) -> Result<Deck, String> {
    let f = File::open(file);
    if f.is_err() { return Err(format!("Bad filename: {}", file)); }

    let buf = BufReader::new(f.unwrap());

    let mut deck = Deck::new();
    for line in buf.lines().map(|l| l.unwrap()) {
        let token:char = line.chars().next().unwrap();
        match token {
            '#' => {
                let line = &line[1..];
                if let Some(space) = line.find(' ')
                {
                    let (k, v) = line.split_at(space);
                    deck.vars.insert(k.to_string(), v.trim().to_string());
                } else {
                    return Err(format!("Deck var error: {} in {}", line, file));
                }
            },
            _ => deck.content.push(line.into()),
        }
    }

    let mut columns:u8 = 0;
    for s in &deck.content {
        let cols = s.split(':').collect::<Vec<&str>>().len() as u8;
        if cols > columns { columns = cols; }
    }
    deck.cols = columns;

    Ok(deck)
}
