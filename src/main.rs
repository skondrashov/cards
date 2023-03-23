use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: Value,
}

fn main() {
    let mut cards = Vec::new();
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for value in [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ] {
            cards.push(Card { suit, value });
        }
    }
    println!("{:?}", cards);
    cards.shuffle(&mut thread_rng());
    println!("{:?}", cards);
}
