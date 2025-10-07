# Implicit returns

Rust's implicit return is very similar to Kotlin (and other functional-inspired languages).
Both Rust and Kotlin use the same principle:

- The last expression in a block is automatically returned
- No semicolon for implicit return
- If you add a semicolon, it becomes a statement (returns ())

```rust
impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }  // No return keyword, no semicolon
    }
}
```