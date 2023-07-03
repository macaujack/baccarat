use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

const SUITS: [Suit; 4] = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self.suit {
            Suit::Diamond => 'd',
            Suit::Club => 'c',
            Suit::Heart => 'h',
            Suit::Spade => 's',
        };
        let value = match self.value {
            1 => 'A',
            v @ 2..=9 => ('0' as u8 + v) as char,
            10 => 'T',
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            _ => unreachable!(),
        };
        write!(f, "{}{}", suit, value)
    }
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Card {
        Card { suit, value, }
    }
    
    pub fn from_index(index: usize) -> Card {
        if index >= 52 {
            panic!("Index must be < 52");
        }
        Card {
            suit: SUITS[index / 13],
            value: (index % 13 + 1) as u8,
        }
    }
    
    pub fn to_value_index(&self) -> usize {
        (self.value - 1) as usize
    }
    
    pub fn to_index(&self) -> usize {
        self.suit as usize * 13 + (self.value - 1) as usize
    }
}

#[derive(Debug, Clone)]
pub struct Shoe {
    number_of_decks: u32,
    cut_card_index: usize,
    cards: Vec<Card>,
    index: usize,
    
    counter: Counter,
}

impl Shoe {
    pub fn new(number_of_decks: u32, cut_card_proportion: f64) -> Self {
        let mut cards = Vec::with_capacity(number_of_decks as usize * 52);
        for _ in 0..number_of_decks {
            for suit in SUITS {
                for value in 1..=13 {
                    cards.push(Card::new(suit, value));
                }
            }
        }
        
        Shoe {
            number_of_decks,
            cut_card_index: ((number_of_decks * 52) as f64 * cut_card_proportion) as usize,
            cards,
            index: 0,
            
            counter: Counter::new(number_of_decks),
        }
    }
    
    pub fn shuffle_with_firsts(&mut self, firsts: &[Card]) {
        self.index = 0;
        self.counter = Counter::new(self.number_of_decks);
        let mut card_count = [0; 52];
        
        for (i, card) in firsts.iter().enumerate() {
            let count = &mut card_count[card.to_index()];
            *count += 1;
            if *count > self.number_of_decks {
                panic!("Invalid firsts");
            }
            self.cards[i] = *card;
        }
        
        let mut idx = firsts.len();
        for suit in SUITS {
            for value in 1..=13 {
                let card = Card::new(suit, value);
                let card_index = card.to_index();
                for _ in card_count[card_index]..self.number_of_decks {
                    self.cards[idx] = card;
                    idx += 1;
                }
            }
        }
        
        self.cards[firsts.len()..].shuffle(&mut rand::thread_rng());
    }
    
    pub fn retry_without_shuffle(&mut self) {
        self.index = 0;
        self.counter = Counter::new(self.number_of_decks);
    }
    
    pub fn deal_card(&mut self) -> Card {
        let card = self.cards[self.index];
        self.index += 1;
        self.counter.remove_card(card);
        card
    }
    
    pub fn shuffle(&mut self) {
        self.shuffle_with_firsts(&[]);
    }
    
    pub fn get_number_of_decks(&self) -> u32 {
        self.number_of_decks
    }
    
    pub fn get_next_cards(&self) -> &[Card] {
        &self.cards[self.index..]
    }
    
    pub fn get_value_count(&self) -> &[u32; 13] {
        self.counter.get_value_count()
    }
    
    pub fn get_card_count(&self) -> &[u32; 52] {
        self.counter.get_card_count()
    }
    
    pub fn get_index(&self) -> usize {
        self.index
    }
    
    pub fn is_cut_card_reached(&self) -> bool {
        self.index >= self.cut_card_index
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    value_count: [u32; 13],
    card_count: [u32; 52],
}

impl Counter {
    pub fn new(number_of_decks: u32) -> Self {
        Counter {
            value_count: [4 * number_of_decks; 13],
            card_count: [number_of_decks; 52],
        }
    }
    
    pub fn add_card(&mut self, card: Card) {
        self.value_count[card.to_value_index()] += 1;
        self.card_count[card.to_index()] += 1;
    }
    
    pub fn remove_card(&mut self, card: Card) {
        self.value_count[card.to_value_index()] -= 1;
        self.card_count[card.to_index()] -= 1;
    }
    
    pub fn get_value_count(&self) -> &[u32; 13] {
        &self.value_count
    }
    
    pub fn get_card_count(&self) -> &[u32; 52] {
        &self.card_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_shoe() {
        let shoe = Shoe::new(8, 0.9);
        assert_eq!(shoe.cards[0], Card::new(Suit::Diamond, 1));
        assert_eq!(shoe.cards[51], Card::new(Suit::Spade, 13));
        assert_eq!(shoe.cards[52], Card::new(Suit::Diamond, 1));
    }
    
    #[test]
    fn test_shuffle_with_firsts() {
        let firsts = [
            Card::new(Suit::Spade, 9),
            Card::new(Suit::Heart, 3),
            Card::new(Suit::Club, 13),
            Card::new(Suit::Diamond, 5),
        ];
        let mut shoe = Shoe::new(8, 0.9);
        shoe.shuffle_with_firsts(&firsts);
        
        assert_eq!(shoe.index, 0);
        
        // Check if cards' numbers are correct.
        assert_eq!(shoe.cards.len(), shoe.number_of_decks as usize * 52);
        let mut count = [0; 52];
        for card in &shoe.cards {
            count[card.to_index()] += 1;
            if count[card.to_index()] > shoe.number_of_decks {
                panic!("Card count is invalid")
            }
        }
        
        // Check if first few cards are fixed.
        for (i, card) in firsts.iter().enumerate() {
            assert_eq!(shoe.cards[i], *card);
        }
        
        // Check if the rest is shuffled.
        let first_random_card = shoe.cards[firsts.len()];
        let mut ok = false;
        for _ in 0..100 {
            shoe.shuffle_with_firsts(&firsts);
            if shoe.cards[firsts.len()] != first_random_card {
                ok = true;
                break;
            }
        }
        assert!(ok);
    }
    
    #[test]
    fn test_checking_cut_card_reached() {
        let mut shoe = Shoe::new(8, 0.9);
        let cut_card_index = ((shoe.number_of_decks * 52) as f64 * 0.9) as usize;
        assert!(!shoe.is_cut_card_reached());
        for _ in 0..cut_card_index - 1 {
            shoe.deal_card();
            assert!(!shoe.is_cut_card_reached());
        }
        shoe.deal_card();
        assert!(shoe.is_cut_card_reached());
        
        shoe.shuffle();
        assert_eq!(shoe.index, 0);

        assert!(!shoe.is_cut_card_reached());
        for _ in 0..cut_card_index - 1 {
            shoe.deal_card();
            assert!(!shoe.is_cut_card_reached());
        }
        shoe.deal_card();
        assert!(shoe.is_cut_card_reached());
    }
    
    #[test]
    #[ignore]
    fn print_first_few_cards_in_shuffled_shoe() {
        let firsts = [
            Card::new(Suit::Spade, 9),
            Card::new(Suit::Heart, 3),
            Card::new(Suit::Club, 13),
            Card::new(Suit::Diamond, 5),
        ];
        let mut shoe = Shoe::new(8, 0.9);
        print!("Not shuffled:");
        for i in 0..10 {
            print!(" {:#?}", shoe.cards[i]);
        }
        println!();
        
        shoe.shuffle_with_firsts(&firsts);
        print!("Shuffled:");
        for i in 0..10 {
            print!(" {:#?}", shoe.cards[i]);
        }
        println!();
    }
}
