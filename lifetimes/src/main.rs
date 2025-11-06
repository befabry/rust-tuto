fn main() {
    let languages = vec![
        String::from("Rust"),
        String::from("Go"),
        String::from("Typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("{}", result);

    let last = last_language(&languages);
    println!("{}", last);

    let longest = longest_language("go", "typescript");
    println!("{}", longest);
}

/// Finds the next language after current one, with explicit lifetime
///
/// - `'a` lifetime tells Rust: "returned reference lives as long as the input slice"
/// - Without `'a`, Rust can't guarantee the returned &str won't outlive the input data
/// - The lifetime connects input and output - returned &str is valid while languages is
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

/// Gets last language - lifetime elision works here
///
/// - Rust automatically infers: output &str lives as long as input &[String]  
/// - "Lifetime elision" rules handle simple cases like this
/// - Equivalent to: `fn last_language<'a>(languages: &'a [String]) -> &'a str`
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

/// Returns the longest of two string references with tied lifetimes
///
/// - `'a` connects both inputs AND output
/// - Means: "both inputs must live at least as long as 'a, and output lives 'a"
/// - Prevents returning reference that might outlive its source data
/// - Both inputs must have SAME lifetime constraint
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}
