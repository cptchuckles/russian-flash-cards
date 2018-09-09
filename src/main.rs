extern crate rand;
mod flashcard;
use std::io;
use flashcard::*;
use rand::prelude::*;


fn main() {
    
    if let Some(deck) = prompt_for_deck() {    
        //debug stuff:
        println!("Deck reads ok, cards: {}", deck.content.len());
        print_deck(&deck);
        println!("============================");
        
        println!("Session variables:");
        for (k, v) in deck.vars.iter() {
            println!("{}: {}", k, v);
        }
    }
}

fn print_deck(deck: &Deck) {
    let mut i:usize = 0;
    println!("Deck max columns: {}", deck.cols);
    while let Some(card) = deck.get_card(i) {
        println!("{} - {}", card[0], card[1]);
        i += 1;
    }
}

fn prompt_for_deck() -> Option<Deck> {
    println!("enter file name to load:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("nigga what");
    let file = input.trim();
    
    Deck::new_from_file(&file)
}


