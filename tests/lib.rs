extern crate pokerlookup;
extern crate pokereval;
extern crate holdem;
extern crate cards;

use std::path::Path;

use pokerlookup::{LookupTable, translate_card};
use cards::card::{Card, Suit, Value};
use holdem::{hand_rank_to_class, HandRankClass};

use pokereval::eval_5cards;

use std::collections::{HashMap};

use cards::deck::{Deck};
use holdem::{HandRank};

static HAND_RANK_PATH : &'static str = "gen/HandRanks.dat";

#[test]
#[ignore]
fn generate_and_save() {
    let out_path = Path::new("/tmp/HandRanks.dat");
    let mut table = LookupTable::generate();
    table.save(out_path);

    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);
    let c6 = Card(Value::Three, Suit::Diamonds);
    let c7 = Card(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = table.eval_7cards(cards);

    assert_eq!(hand_rank_to_class(&(rank as u16)), HandRankClass::FourOfAKind);
}

#[test]
#[ignore]
fn read_and_copy() {
    let in_path = Path::new("/tmp/HandRanks.dat");
    let mut table = LookupTable::load(&in_path);
    let out_path = Path::new("/tmp/HandRanks_new.dat");
    table.save(&out_path);
    //TODO: check files for sameness?
}

#[test]
fn evaluate() {
    let in_path = Path::new(HAND_RANK_PATH);
    let table = LookupTable::load(&in_path);

    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);
    let c6 = Card(Value::Three, Suit::Diamonds);
    let c7 = Card(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = table.eval_7cards(cards);

    assert_eq!(hand_rank_to_class(&(rank as u16)), HandRankClass::FourOfAKind);

    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Four, Suit::Diamonds);
    let c4 = Card(Value::Six, Suit::Clubs);
    let c5 = Card(Value::King, Suit::Hearts);
    let c6 = Card(Value::Three, Suit::Diamonds);
    let c7 = Card(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = table.eval_7cards(cards);

    assert_eq!(hand_rank_to_class(&(rank as u16)), HandRankClass::TwoPair);
}

#[test]
fn evaluate_all_possible_5_card_combinations_for_stats() {
    let in_path = Path::new(HAND_RANK_PATH);
    let table = LookupTable::load(&in_path);

    let mut deck = Deck::new_unshuffled();
    let mut cards : [usize; 52] = [0; 52];

    // this could be made faster, by creating a function that works on raw-card-representations and translating
    // the deck cards into it
    for i in 0..52 {
        let card = deck.draw().ok().unwrap();
        cards[i] = translate_card(&card) as usize;
    }

    let mut rank_class_count : HashMap<HandRankClass, usize> = HashMap::new();
    let mut rank_count : HashMap<HandRank, bool> = HashMap::new();

    // 2,598,960 unique poker hands
    for i1 in 0..52 {
        for i2 in (i1+1)..52 {
            for i3 in (i2+1)..52 {
                for i4 in (i3+1)..52 {
                    for i5 in (i4+1)..52 {
                        let c1 = &cards[i1];
                        let c2 = &cards[i2];
                        let c3 = &cards[i3];
                        let c4 = &cards[i4];
                        let c5 = &cards[i5];

                        let rank = table.eval_5cards_raw([c1, c2, c3, c4, c5]);

                        // mark the rank in the map
                        rank_count.entry(rank as u16).or_insert(true);
                    }
                }
            }
        }
    }

    // count distinct ranks for each rank class
    for key in rank_count.keys() {
        let rank_class = hand_rank_to_class(key);

        let count = rank_class_count.entry(rank_class).or_insert(0);
        *count += 1;
    }

    // this is a bit redundant
    // there should be 7462 unique ranks, in accordance with the hand_rank_to_class function
    assert_eq!(rank_count.len(), 7462);

    assert_eq!(*rank_class_count.get(&HandRankClass::HighCard).unwrap(), 1277);
    assert_eq!(*rank_class_count.get(&HandRankClass::OnePair).unwrap(), 2860);
    assert_eq!(*rank_class_count.get(&HandRankClass::TwoPair).unwrap(), 858);
    assert_eq!(*rank_class_count.get(&HandRankClass::ThreeOfAKind).unwrap(), 858);
    assert_eq!(*rank_class_count.get(&HandRankClass::Straight).unwrap(), 10);
    assert_eq!(*rank_class_count.get(&HandRankClass::Flush).unwrap(), 1277);
    assert_eq!(*rank_class_count.get(&HandRankClass::FullHouse).unwrap(), 156);
    assert_eq!(*rank_class_count.get(&HandRankClass::FourOfAKind).unwrap(), 156);
    assert_eq!(*rank_class_count.get(&HandRankClass::StraightFlush).unwrap(), 10);
}

#[test]
fn evaluate_all_possible_5_card_combinations_agree() {
    let in_path = Path::new(HAND_RANK_PATH);
    let table = LookupTable::load(&in_path);

    let mut deck = Deck::new_unshuffled();
    let mut cards_lookup : [usize; 52] = [0; 52];
    let mut cards : Vec<Card> = Vec::new();

    // this could be made faster, by creating a function that works on raw-card-representations and translating
    // the deck cards into it
    for i in 0..52 {
        let card = deck.draw().ok().unwrap();
        cards_lookup[i] = translate_card(&card) as usize;
        cards.push(card);
    }

    // 2,598,960 unique poker hands
    for i1 in 0..52 {
        for i2 in (i1+1)..52 {
            for i3 in (i2+1)..52 {
                for i4 in (i3+1)..52 {
                    for i5 in (i4+1)..52 {
                        let c1 = &cards_lookup[i1];
                        let c2 = &cards_lookup[i2];
                        let c3 = &cards_lookup[i3];
                        let c4 = &cards_lookup[i4];
                        let c5 = &cards_lookup[i5];

                        let rank_lookup = table.eval_5cards_raw([c1, c2, c3, c4, c5]);

                        let c1 = &cards[i1];
                        let c2 = &cards[i2];
                        let c3 = &cards[i3];
                        let c4 = &cards[i4];
                        let c5 = &cards[i5];
    
                        let hand = [c1, c2, c3, c4, c5];
                        let rank_eval = eval_5cards(&hand);

                        assert_eq!(rank_eval, rank_lookup);
                    }
                }
            }
        }
    }
}
