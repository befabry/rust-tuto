use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of "suits", hearts, spades
        // List of "values" - ace, etc
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // No return statements, like Kotlin ?
        // Implicit return (important no semicolon ;)
        Deck { cards }
    }

    fn shufle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shufle();

    println!("Here's the deck: {:#?}", deck);

    println!("Here's your hand: {:#?}", deck.deal(3));
}
