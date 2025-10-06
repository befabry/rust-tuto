#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // List of "suits", hearts, spades
    // List of values - 'ace', etc

    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };
    println!("Here's your deck: {:#?}", deck);
}
