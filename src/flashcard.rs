#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub struct Deck {
    cols: u8,
    splitter: char,
    content: Vec<String>,
    pub vars: HashMap<String,String>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cols: 0,
               splitter: ' ',
               content: Vec::new(),
               vars: HashMap::new(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.content.len()
    }
    
    pub fn splitter(&self) -> String {
        let mut r = String::new();
        r.push(self.splitter);
        r
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
    
    pub fn empty(&mut self) {
        &self.content.clear();
    }
    
    pub fn push_card(&mut self, card: String) {
        self.content.push(card);
    }
    
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    
    pub fn get_card(&self, ind: usize) -> Option<Vec<String>> {
        if ind >= self.content.len() {return None;}
        let s:Vec<&str> = self.content[ind].split(' ').collect();
        let mut r: Vec<String> = Vec::with_capacity(s.len());
        for ss in s {
            r.push(String::from(ss));
        }
        Some(r)
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
    for mut line in buf.lines().map(|l| l.unwrap()) {
        let token:char = line.chars().next().unwrap();
        match token {
            '#' => {
                if let Some(space) = line.find(' ')
                {
                    let (k, v) = line.split_at(space);
                    let v = String::from(v.trim());
                    deck.vars.insert(String::from(k), String::from(v));
                } else {
                    return Err(format!("Deck var error: {} in {}", line, file));
                }
            },
            _ => deck.push_card(line.into()),
        }
    }
    
    Ok(deck)
}