# Inherent implementations

## Methods

Methods are like class functions/instance methods. It works with a specific instance (can read/mutate self)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method - operates on a specific instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that can mutate the instance
    fn scale(&mut self, factor: f64) {
        self.width = (self.width as f64 * factor) as u32;
        self.height = (self.height as f64 * factor) as u32;
    }
}

// Usage:
let mut rect = Rectangle { width: 10, height: 20 };
println!("Area: {}", rect.area());  // Called on instance
rect.scale(1.5);  // Mutates the instance
```

## Associated Functions

Associated functions are like static methods. It is not tied to any instance (like factory functions, utilities).

```rust
impl Rectangle {
    // Associated function - not tied to any instance
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Another associated function
    fn default() -> Self {
        Rectangle {
            width: 1,
            height: 1,
        }
    }
}

// Usage:
let square = Rectangle::square(5);  // Called using ::
let default_rect = Rectangle::default();
```