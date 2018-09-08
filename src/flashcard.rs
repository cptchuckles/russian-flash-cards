#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

pub struct Deck {
    cols: u8,
    splitter: char,
    content: Vec<String>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cols: 0,
               splitter: ' ',
               content: Vec::new(),
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
    
    pub fn new_from_file(file: &str) -> io::Result<Deck> {
        let mut d = Deck::new();
        load_deck_from_file(&mut d, file)?;
        Ok(d)
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

pub fn load_deck_from_file(deck: &mut Deck, file: &str) -> io::Result<()> {
    let f = File::open(file)?;
    let buf = BufReader::new(f);
    
    if !deck.is_empty() { deck.empty(); }
    for line in buf.lines().map(|l| l.unwrap()) {
        &deck.push_card(line.into());
    }
    
    Ok(())
}