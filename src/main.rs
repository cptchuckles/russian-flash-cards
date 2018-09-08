use std::io::{self, Write};
use std::collections::HashMap;

mod flashcard;
use flashcard::*;


fn main() {
    println!("enter file name to load:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("nigga what");
    let file = input.trim();
    
    let deck = Deck::new_from_file(&file)
        .expect("Could not read file");
    
    //debug stuff:
    println!("Deck reads ok, cards: {}", deck.len());
    print_deck(&deck);
    println!("============================");
    
    let mut vars:HashMap<String,String> = HashMap::new();
    
    let mut i:usize = 0;
    while let Some(mut line) = deck.get_card_data(i) {
        let token:char = line.remove(0);
        match token {
            '#' => {
                let space = line.find(' ').expect("whoah there nigga");
                let (k, v) = line.split_at(space);
                let v = String::from(v.trim());
                vars.insert(String::from(k), String::from(v));
            },
            _ => (),
        }
        i += 1;
    }
    
    println!("Session variables:");
    for (k, v) in vars.iter() {
        println!("{}: {}", &k, &v);
    }
}

fn print_deck(deck: &Deck) {
    let mut i:usize = 0;
    while let Some(card) = deck.get_card_data(i) {
        let mut t = card.split(' ');
        println!("{} - {}", t.next().unwrap(), t.next().unwrap());
        i += 1;
    }
}
