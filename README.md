# Rust Learning

A collection of exercises, conceptual demonstrations, and small projects used to learn the Rust programming language.

## Structure

### Chapters
- **Chapter 1**: Basic variables, immutability, shadowing, and raw pointer casting.
- **Chapter 2**: Functions, match expressions, and loop control flow.
- **Chapter 3**: Ownership, borrowing rules, and lifetimes.
- **Chapter 4**: Structs, Enums, Generics, and error handling with the `?` operator.
- **Chapter 5**: Smart pointers (`Box`), `Vec`, `String`, and `HashMap`.
- **Chapter 6**: Derive macros, trait implementation (`Display`), and custom error handling using `thiserror`.

### Advanced Topics
- Memory management with `Rc` and `RefCell`.
- Differences between `.clone()` and `.cloned()`.

### Projects
- **Snake Game**: A terminal-based game implemented with `crossterm` and `rand`.
- **RAII Pattern**: Implementation of a `TerminalGuard` to manage terminal state and cleanup.
- **Word Count**: A benchmark tool comparing `HashSet` and `HashMap` for processing file word frequencies.

## Dependencies
The projects in this repository utilize the following crates:
- `crossterm`
- `rand`
- `thiserror`
