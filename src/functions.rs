use pokereval::original::{eval_5cards_kev_array, eval_6cards_kev_array, eval_7cards_kev_array};

/*
fn card_to_string(card: &i32) -> String {
    let mut s = String::new();
    let rank : i32 = (*card >> 8) & 0xf;
    let mut suit : i32 = (*card >> 12) & 0xf;

    let rank_str = match rank {
        0 => "2",
        1 => "3",
        2 => "4",
        3 => "5",
        4 => "6",
        5 => "7",
        6 => "8",
        7 => "9",
        8 => "T",
        9 => "J",
        10 => "Q",
        11 => "K",
        12 => "A",
        _ => "?"
    };

    let suit_str = match suit {
        1 => "h",
        2 => "d",
        4 => "c",
        8 => "p",
        //x => {println!("{}", x); "?"}
        _ => "?"
    };

    s.push_str(rank_str);
    s.push_str(suit_str);
    s
}

fn print_workcards(cards: &[i32; 8], numevalcards: &i32) {
    //let mut s = String::new();

    for i in 0..*numevalcards+1 {
        //s.push_str(" ");
        //s += card_to_string(&cards[i]);
        print!(" {}", card_to_string(&cards[i as usize]));
    }
    print!("\n");
    //println!("{}", s);
}
*/

// adding a new card to this ID
pub fn make_id(id_in: i64, card: i32, numcards: &mut i32) -> i64 {
    let mut newcard = card;

    let mut suitcount : [i32;4+1] = [0;5];
    let mut rankcount : [i32;13+1] = [0;14];
    let mut workcards : [i32;8] = [0;8]; // intentionally keeping one as a 0 end
    let mut getout = 0;

    // can't have more than 6 cards
    for cardnum in 0..6 {
        workcards[cardnum+1] = ((id_in >> (8*cardnum)) & 0xff) as i32;
    }

    // my cards are 2c = 1, 2d = 2  ... As = 52
    newcard-=1; // make 0 based!

    workcards[0] = (((newcard >> 2) + 1) << 4) + (newcard & 3) + 1; // add next card formats card to rrrr00ss

    *numcards = 0;
    while workcards[*numcards as usize] != 0 {
        suitcount[(workcards[*numcards as usize] as usize) & 0xf] += 1; // need to see if suit is significant
        rankcount[((workcards[*numcards as usize] as usize) >> 4) & 0xf] += 1; // and rank to be sure we don't have 4!

        if *numcards != 0 {
            if workcards[0] == workcards[*numcards as usize] { // can't have the same card twice
                getout = 1; // if so need to get out after counting numcards
            }
        }

        *numcards += 1;
    }

    if getout != 0 { return 0; } // duplicated another card (ignore this one)

    let needsuited : i32 = *numcards - 2; // for suit to be significant - need to have n-2 of same suit
    if *numcards > 4 {
        for rank in 1..14 {
            if rankcount[rank] > 4 { // if I have more than 4 of a rank then I shouldn't do this one!!
                return 0; // can't have more than 4 of a rank so return an ID that can't be!
            }
        }
    }

    // However in the ID process I prefered that
    // 2s = 0x21, 3s = 0x31,.... Kc = 0xD4, Ac = 0xE4
    // This allows me to sort in Rank then Suit order

    // if we don't have at least 2 cards of the same suit for 4, we make this card suit 0.
    if needsuited > 1 {
        for cardnum in 0..*numcards { // for each card
            if suitcount[(workcards[cardnum as usize] as usize) & 0xf] < needsuited { // check suitcount to the number I need to have suits significant
                workcards[cardnum as usize] = workcards[cardnum as usize] & 0xf0; // if not enough - 0 out the suit - now this suit would be a 0 vs 1-4
            }
        }
    }

    // Sort Using XOR.  Network for N=7, using Bose-Nelson Algorithm
    macro_rules! SWAP {
        ($i:expr, $j:expr) => {
            if workcards[$i] < workcards[$j] {
                workcards[$i] = workcards[$i] ^ workcards[$j];
                workcards[$j] = workcards[$j] ^ workcards[$i];
                workcards[$i] = workcards[$i] ^ workcards[$j];
            }
        }
    }

    SWAP!(0, 4);     
    SWAP!(1, 5);     
    SWAP!(2, 6);     
    SWAP!(0, 2);     
    SWAP!(1, 3);     
    SWAP!(4, 6);     
    SWAP!(2, 4);     
    SWAP!(3, 5);     
    SWAP!(0, 1);     
    SWAP!(2, 3);     
    SWAP!(4, 5);     
    SWAP!(1, 4);     
    SWAP!(3, 6);     
    SWAP!(1, 2);     
    SWAP!(3, 4);     
    SWAP!(5, 6); 

    // long winded way to put the pieces into a __int64 
    // cards in bytes --66554433221100   
    // the resulting ID is a 64 bit value with each card represented by 8 bits.
    workcards[0] as i64 +
    ((workcards[1] as i64) << 8) +
    ((workcards[2] as i64) << 16) +
    ((workcards[3] as i64) << 24) +
    ((workcards[4] as i64) << 32) +
    ((workcards[5] as i64) << 40) +
    ((workcards[6] as i64) << 48)
}

const PRIMES : [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

pub fn eval_5hand(cards: &[i32; 8]) -> i32 {
    let c1 = cards[0] as u32;
    let c2 = cards[1] as u32;
    let c3 = cards[2] as u32;
    let c4 = cards[3] as u32;
    let c5 = cards[4] as u32;

    let hand = [&c1, &c2, &c3, &c4, &c5];
    eval_5cards_kev_array(&hand) as i32
}

pub fn eval_6hand(cards: &[i32; 8]) -> i32 {
    let c1 = cards[0] as u32;
    let c2 = cards[1] as u32;
    let c3 = cards[2] as u32;
    let c4 = cards[3] as u32;
    let c5 = cards[4] as u32;
    let c6 = cards[5] as u32;

    let hand = [&c1, &c2, &c3, &c4, &c5, &c6];
    eval_6cards_kev_array(&hand) as i32
}

pub fn eval_7hand(cards: &[i32; 8]) -> i32 {
    let c1 = cards[0] as u32;
    let c2 = cards[1] as u32;
    let c3 = cards[2] as u32;
    let c4 = cards[3] as u32;
    let c5 = cards[4] as u32;
    let c6 = cards[5] as u32;
    let c7 = cards[6] as u32;

    let hand = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    eval_7cards_kev_array(&hand) as i32
}

pub fn do_eval(id_in: i64) -> i32 {
    // I guess I have some explaining to do here...  I used the Cactus Kevs Eval ref http://www.suffecool.net/poker/evaluator.html
    // I Love the pokersource for speed, but I needed to do some tweaking to get it my way
    // and Cactus Kevs stuff was easy to tweak ;-)
    let mut mainsuit: i32 = 20; // just something that will never hit...  need to eliminate the main suit from the iterator
    let mut suititerator: i32 = 1; // changed as per Ray Wotton's comment at http://archives1.twoplustwo.com/showflat.php?Cat=0&Number=8513906&page=0&fpart=18&vc=1

    let mut workcards : [i32; 8] = [0;8]; // intentially keeping one as a 0 end
    let mut holdcards : [i32; 8] = [0;8];
    let mut numevalcards : i32 = 0;

    if id_in == 0 {
        let handrank : i32 = 0;
        return handrank;
    }

    // if I have a good ID then do it...
    // convert all 7 cards (0s are ok)
    for cardnum in 0..7 {
        
        holdcards[cardnum as usize] = ((id_in >> (8*cardnum)) & 0xff) as i32;
        if holdcards[cardnum] == 0 {
            break; // once I hit a 0 I know I am done
        }
        numevalcards+=1; // if not 0 then count the card

        let suit : i32 = holdcards[cardnum] & 0xf; // find out what suit (if any) was significant
        if suit != 0 {
            mainsuit = suit; // and remember it
        }
    }

    for cardnum in 0..numevalcards {
        // convert to cactus kevs way!!  ref http://www.suffecool.net/poker/evaluator.html
        //   +--------+--------+--------+--------+
        //   |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
        //   +--------+--------+--------+--------+
        //   p = prime number of rank (deuce=2,trey=3,four=5,five=7,...,ace=41)
        //   r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
        //   cdhs = suit of card
        //   b = bit turned on depending on rank of card
        let workcard : i32 = holdcards[cardnum as usize];
        let rank : i32 = (workcard >> 4) - 1;
        let mut suit : i32 = workcard & 0xf;

        if suit == 0 {
            suit = suititerator;
            suititerator += 1;
            if suititerator == 5 { suititerator = 1; }

            if suit == mainsuit {
                suit = suititerator;
                suititerator += 1;
                if suititerator == 5 { suititerator = 1; }
            }

        }
        // now make Cactus Kev's Card
        workcards[cardnum as usize] = PRIMES[rank as usize] | (rank << 8) | (1 << (suit+11)) | (1 << (16 + rank));
    }

    //DIFFERENCE: here the original handrank is transformed to something else

    // return handrank
    match numevalcards {
        5 => eval_5hand(&workcards),
        6 => eval_6hand(&workcards),
        7 => eval_7hand(&workcards),
        _ => panic!("Problem in do_eval!") //NOTE: numcard was printed here
    }
}
