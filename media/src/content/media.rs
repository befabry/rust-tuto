#[derive(Debug)]
pub enum Media {
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
    pub fn description_using_conditionals(&self) -> String {
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
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(episode_number) => format!("Podcast: {}", episode_number),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}
