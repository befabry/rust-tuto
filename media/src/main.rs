use std::fmt::format;

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    // Podcast { episode_number: u32 },
    Podcast(u32),
    Placeholder,
}

impl Media {
    // Initial approach using if-let conditionals
    // Demonstrates conditional matching before learning pattern matching
    fn description_using_conditionals(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media Description")
        }
    }

    // Idiomatic Rust approach using pattern matching
    // More concise and exhaustive at compile time
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // Vectors return Option<T> from get() - Rust's safe alternative to null
    // Pattern matching forces us to handle both cases explicitly
    match catalog.items.get(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }

    // Elegant Rust syntax: `if let` combines pattern matching with conditional execution
    // Creates a new variable `value` in the scope of the if-block only when Some(value) is found
    // Much cleaner than manually matching and handling each case separately
    if let Some(value) = catalog.get_by_index(100) {
        println!("Item {:#?}", value);
    } else {
        println!("No value !!!");
    }

    // DANGER: unwrap() will panic if the Option is None
    // Only use when you're 100% certain the value exists
    // Or in quick prototypes where crashing is acceptable
    let item = catalog.items.get(3);
    println!("unwrap : {:#?}", item.unwrap());

    // expect() is like unwrap() but with a custom panic message
    // Still crashes on None, but provides better debugging context
    println!(
        "expect : {:#?}",
        item.expect("there should be a value here")
    );

    // unwrap_or() provides a safe default value instead of panicking
    // If the Option is None, it returns your provided fallback
    let placeholder = Media::Placeholder;
    println!("expect : {:#?}", item.unwrap_or(&placeholder));
}
