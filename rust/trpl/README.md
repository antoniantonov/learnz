# The Rust Programming Language

## rustup CLI commands
```bash
# Install the latest stable version of Rust
rustup install stable
```

## Useful cargo CLI commands
```bash
# Create a new project
cargo new project_name

# Build the project
cargo build

# Build and run the project
cargo run

# Check the project for errors
cargo check

# Generate and open the project documentation
cargo doc --open
```

## Chapters

### Chapter 3: Common Programming Concepts
- Interesting behavior - in release mode, we have wrap around behavior for integer overflow. In debug mode, we get a panic.

- Expressions vs. Statements
  - Expressions return a value, statements do not.
  - Expressions do not end with a semicolon, statements do.
  ```rust
    // This is an expression. If we add ; after the x + 1, it becomes a statement.    
    {
        let x = 3;
        x + 1
    }
```