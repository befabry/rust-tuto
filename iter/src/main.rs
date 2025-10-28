fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    //iter(&colors);
    // basic_for_loop(&colors);
    //foreach_loop(&colors);
    // map_elements(&colors);
    // shorten_strings(&mut colors);
    let uppercase = to_uppercase(&colors);
    println!("{:#?}", uppercase);
}

// Transforms strings to uppercase and collects into a new Vec
//
// - `map()` transforms each element but returns a lazy iterator
// - `collect()` is a CONSUMER that executes the pipeline and creates a new collection
// - Allocates a new Vec<String> on the heap with the transformed data
// - Returns owned data - caller gets full ownership of the new vector
//
// # Memory Behavior:
// - Original slice: &[String] (borrowed, unchanged)
// - Returned value: Vec<String> (new owned collection)
// - Each string is duplicated in uppercase form
fn to_uppercase(elements: &[String]) -> Vec<String> {
    // collect guess the type from the return type
    // collect::Vec<String>() can be specified like this as well
    elements.iter().map(|el| el.to_uppercase()).collect()
}

// Shortens all strings to their first character using iter_mut()
//
// - `iter_mut()` borrows each element MUTABLY (&mut String)
// - Does NOT take ownership - original vector keeps ownership
// - `truncate(1)` modifies each string in-place on the heap
// - Changes are made directly to the original data, no new allocation
//
// # Memory Behavior:
// - Input: &mut [String] (mutable borrow of the slice)
// - Operation: Modifies existing String data in-place
// - Output: Original vector contains shortened strings
// - No new collections created - memory efficient
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

// Demonstrates iterator adapters with map()
//
// - `map()` is an iterator adapter - it transforms each element
// - Adapters DON'T consume the iterator or call next() immediately
// - They add processing steps to the iterator chain
// - Returns a new iterator that will apply the transformation when consumed
//
// # Iterator Pipeline:
// iter() → map() → for_each()
//   │        │        │
//   │        │        └── Consumer (executes the chain)
//   │        └── Adapter (transforms &str to String)
//   └── Creates base iterator
//
// Demonstrates using vector slices for more flexible function signatures
//
// - `&[String]` is a SLICE of Strings - a view into contiguous sequence
// - Works with Vec<String>, arrays, and other sliceable collections
// - More flexible than `&Vec<String>` - accepts multiple collection types
// - Borrows the data, doesn't take ownership
//
// # What you can pass to this function:
// - `&vec_of_strings` (reference to Vec<String>)
// - `&array_of_strings` (reference to [String; N])
// - `&slice_of_strings` (reference to &[String] itself)
// - Any type that can be coerced to &[String]
fn map_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

// Iterates using for_each with a closure
//
// - `||` creates a closure (anonymous function) that takes each element
// - Iterators are "lazy" - nothing happens until consumed
// - `for_each()` is a "consumer" that executes the closure on each item
// - Functional programming style, chains operations together
fn foreach_loop(elements: &Vec<String>) {
    // || creates an anonymous function
    // an iterator is "lazy" until consumed, for_each consumes
    elements.iter().for_each(|el| println!("{}", el));
}

// Iterates using traditional for loop syntax
//
// - Most familiar syntax for developers from other languages
// - The for loop automatically calls .into_iter() and handles the iteration
// - More imperative style - focuses on "how" to do the iteration
// - Generally preferred for simple iterations in Rust
fn basic_for_loop(elements: &Vec<String>) {
    for element in elements {
        println!("{:#?}", element);
    }
}

// How Rust iterators "iter" work - lazy, stateful traversal of collections
fn iter(colors: &Vec<String>) {
    // iter() creates an iterator that yields IMMUTABLE references (&String)
    // The iterator borrows from the original vector, doesn't consume it
    let mut colors_iter = colors.iter();

    // next() returns Option<&String> - Some(item) or None when exhausted
    // Each call advances the internal state of the iterator
    println!("{:#?}", colors_iter.next()); // Some("red")
    println!("{:#?}", colors_iter.next()); // Some("green")
    println!("{:#?}", colors_iter.next()); // Some("blue") 
    println!("{:#?}", colors_iter.next()); // None

    // The original vector is still usable since iter() didn't consume it
    println!("Original colors: {:?}", colors); // Still works!
}
