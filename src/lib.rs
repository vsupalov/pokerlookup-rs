//! pokerlookup-rs contains the means to generate a large lookup table, which can be
//! subsequently used to evaluate large amounts of 5, 6 and 7 card poker hands really fast.

// TODO: mention GPL license? It's based on gpl code

extern crate cards;
extern crate holdem;
extern crate pokereval;

mod functions;

use functions::{make_id, do_eval};

use cards::card::{Suit, Value, Card};

use std::collections::{BTreeMap, VecDeque};

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem;

use std::slice;

pub struct LookupTable {
    hr : Vec<i32>,
}

const EXPECTED_HR_LENGTH : usize = 32487834;

//TODO: change to holdem HandRank?
pub type HandRank = u16;

// translate a card to the internal card representation - usize from 1 to 52 inclusive
pub fn translate_card(card: &Card) -> usize {
    let value = 4*match card.value {
        Value::Two => 0,
        Value::Three => 1,
        Value::Four => 2,
        Value::Five => 3,
        Value::Six => 4,
        Value::Seven => 5,
        Value::Eight => 6,
        Value::Nine => 7,
        Value::Ten => 8,
        Value::Jack => 9,
        Value::Queen => 10,
        Value::King => 11,
        Value::Ace => 12,
    };
    let suit = match card.suit {
        Suit::Hearts => 0,
        Suit::Spades => 1,
        Suit::Diamonds => 2,
        Suit::Clubs => 3,
    };

    (suit + value) + 1 //the cards start at 1 and go to 52
}

impl LookupTable {
    pub fn generate() -> LookupTable {
        // create the hr vec and fill is with 0 entries
        // TODO: could this be done more gracefully?
        let mut hr : Vec<i32>  = Vec::with_capacity(EXPECTED_HR_LENGTH);
        for _ in 0..EXPECTED_HR_LENGTH {
            hr.push(0);
        }

        let mut sub_hands : BTreeMap<u64, i64> = BTreeMap::new();
        let mut sub_hand_queue : VecDeque<u64> = VecDeque::new();
        let mut numcards : i32 = 0; // TODO: as far as i see, this goes from 1 to 7, set in make_id

        // seed the sub_hands with a 0
        let sub_hand : u64 = 0;
        sub_hand_queue.push_back(sub_hand);
        sub_hands.insert(sub_hand, 0);

        // find all possible sub_hands
        while !sub_hand_queue.is_empty() {
            let sub_hand = sub_hand_queue.pop_front().unwrap(); //DIFFERENCE: this differs from the original code. They use front first, and pop later

            // ORIG: start at 1 so I have a zero catching entry (just in case)
            for card in 1..53 {
                // DIFFERENCE: 'explicit numcards pass' differs
                let a_card_more : i64 = match make_id(sub_hand as i64, card, &mut numcards) {
                    None => continue, //impossible hand
                    Some(x) => x
                };

                let ret = sub_hands.insert(a_card_more as u64, 0);

                // queue the new hand id up, if it hasn't been yet if it's fresh and isn't a 7 card hand
                if ret == None && numcards < 6 {
                    sub_hand_queue.push_back(a_card_more as u64); 
                }
            }
        }

        println!("Subhands: {} (612978?)", sub_hands.len()); //TODO: remove this?

        // assign an id to each subhand, thanks to the map type they are sorted already for reproducible results
        let mut position = 0;
        for (_, val) in sub_hands.iter_mut() {
            *val = position;
            position += 1;
        }

        for (sub_hand, sub_hand_position) in sub_hands.iter() {
            for card in 1..53 {
                let max_hr = *sub_hand_position * 53 + card + 53;
                let a_card_more = match make_id(*sub_hand as i64, card as i32, &mut numcards) {
                    None => 0,
                    Some(x) => x
                };

                if numcards == 7 {
                    hr[max_hr as usize] = do_eval(a_card_more); //do_eval(*sub_hand as i64, &numcards) //DIFFERENCE: idem numcards
                    continue;
                }
                if a_card_more == 0 { 
                    continue;
                }

                let a_card_more_it = sub_hands.get(&(a_card_more as u64));
                if a_card_more_it == None {
                    panic!("Couldn't find hand! {} | {}", a_card_more, sub_hand);
                }

                let a_card_more_position = a_card_more_it.unwrap();
                hr[max_hr as usize] = (a_card_more_position * 53 + 53) as i32;
            }

            if numcards == 6 || numcards == 7 { //TODO: should this really be 6 and 7?
                hr[(*sub_hand_position * 53 + 53) as usize] = do_eval(*sub_hand as i64); //DIFFERENCE: idem numcards
            }
        }

        LookupTable{ hr: hr }
    }

    // load lookup table from file
    pub fn load(path: &Path) -> LookupTable {
        let mut file = File::open(path).unwrap(); //TODO: less lazy

        let mut buffer: Vec<u8> = Vec::new();
        // Returns amount of bytes read and append the result to the buffer
        let result = file.read_to_end(&mut buffer).unwrap();
        //println!("Read {} bytes", result);

        if (result / 4) != EXPECTED_HR_LENGTH {
            panic!("The amount of read bytes differs from the expected amount");
        }

        // convert the u8 data to a i32 vector
        let hr : Vec<i32> = unsafe {
            let ptr = buffer.as_mut_ptr() as *mut i32;
            mem::forget(buffer);
            Vec::from_raw_parts(ptr, EXPECTED_HR_LENGTH, EXPECTED_HR_LENGTH)
        };

        LookupTable{ hr: hr }
    }

    // basically just dump the lookup table in binary form
    pub fn save(&mut self, path: &Path) { //TODO: result instead of panic?
        let mut file = File::create(path).unwrap(); //TODO: less lazy
        let buffer: &[u8] = unsafe {
            let ptr = self.hr.as_mut_ptr() as *mut u8;
            slice::from_raw_parts(ptr, EXPECTED_HR_LENGTH*4)
        };
        file.write_all(buffer).unwrap();
    }

    pub fn eval_5cards_raw(&self, cards: [&usize; 5]) -> HandRank {
        let mut a = self.hr[53+*cards[0]] as usize;
        a = self.hr[a+*cards[1]] as usize;
        a = self.hr[a+*cards[2]] as usize;
        a = self.hr[a+*cards[3]] as usize;
        a = self.hr[a+*cards[4]] as usize;
        a = self.hr[a] as usize;
        a as HandRank
    }

    pub fn eval_5cards(&self, cards: [&Card; 5]) -> HandRank {
        let c1 = translate_card(cards[0]);
        let c2 = translate_card(cards[1]);
        let c3 = translate_card(cards[2]);
        let c4 = translate_card(cards[3]);
        let c5 = translate_card(cards[4]);

        let mut a = self.hr[53+c1] as usize;
        a = self.hr[a+c2] as usize;
        a = self.hr[a+c3] as usize;
        a = self.hr[a+c4] as usize;
        a = self.hr[a+c5] as usize;
        a = self.hr[a] as usize;
        a as HandRank
    }

    pub fn eval_7cards(&self, cards: [&Card; 7]) -> HandRank {
        let c1 = translate_card(cards[0]);
        let c2 = translate_card(cards[1]);
        let c3 = translate_card(cards[2]);
        let c4 = translate_card(cards[3]);
        let c5 = translate_card(cards[4]);
        let c6 = translate_card(cards[5]);
        let c7 = translate_card(cards[6]);

        let mut a = self.hr[53+c1] as usize;
        a = self.hr[a+c2] as usize;
        a = self.hr[a+c3] as usize;
        a = self.hr[a+c4] as usize;
        a = self.hr[a+c5] as usize;
        a = self.hr[a+c6] as usize;
        a = self.hr[a+c7] as usize;
        a as HandRank
    }
}
