# Beginner Projects suggested by _ChatGPT_

- [x] **Guessing Game**: Create a simple command-line game where the program generates a random number and the player has to guess it. The program should provide hints such as "too high" or "too low" to guide the player.

- [ ] **Todo List**: Build a basic todo list application where users can add, update, and remove tasks. You can store the tasks in memory or use a file-based approach to persist the data.

- [x] **Word Count**: Write a program that reads a text file and counts the occurrences of each word. It can display the results in a sorted manner, showing the most frequent words first.

- [x] **Temperature Converter**: Develop a program that converts temperatures between Celsius, Fahrenheit, and Kelvin. Allow the user to enter a temperature in one unit and convert it to the other units.

- [ ] **URL Shortener**: Create a URL shortening service that takes long URLs as input and generates short, unique URLs. The program should be able to redirect users to the original long URL when they visit the shortened URL.

- [ ] **File Analyzer**: Write a program that analyzes a given file and provides various statistics such as the total number of lines, words, and characters. You can also calculate the frequency of each word and display the most common ones.

- [x] **Currency Converter**: Build a currency converter that allows users to convert between different currencies. Retrieve exchange rates from an API or use predefined rates.

- [ ] **Simple Web Scraper**: Develop a program that extracts specific information from a website. For example, scrape news headlines or stock prices from a website and display them.

# What I Learned so far

## Variables

- Defined with `let` keyword.
- Variable names should be `snake_case`
- _Immutable_ by default, ie. can not change the value. use `mut` keyword to make it immutable
- Constant is defined by `const`. Name should be `UPPER_SNAKE_CASE`. Must give type at declaration. `const A_CONSTANT: i32 = 30`
- _Shadowing_: Redefine a variable again with `let` keyword.

```rust
let a = 10;
// some code
let a = 20; // shadowing
```

## Functions

- Declares by `fn` keyword. Return value is denoted by `->`
- Return value can be done two way. With `return` keyword & without.

```rust
fn way_one() {
  return "Hello"
}
fn way_two() {
  "Hello"
}
```

- If an argument of function is optional, use `Option<T>`. When using that function, must use `Some()`.
- To check a an `Option` _(enum)_ has a value, use `is_some()`
- To get the value of type `T`, use `unwrap()` or `expect()`

```rust
fn a_function(x: Option<i32>) -> i32 {
  if x.is_some() {
    x
  }
}
a_function(Some(43));
```

## Ownership and Borrowing

- When a variable is declared, it takes the ownership of the data.

## Library

- To create a library run `cargo new /path/to/lib --lib`
- To use that dependency add the lib as `lib_name = { path = "/path/to/lib" }` in `Cargo.toml` file
- Functions of the lib _(that would be used in other files)_ should have `pub` keyword in front of them.

## HTTP Calls

- Use `reqwest` crate.
- Function calling http requests must return `Request<Value<T>, Error>`. Generally used `ok(())`.
- For `async await` use `tokio` crate

## Tokio

- To use a `async await`, use `tokio`.
- It is used ase `#[tokio::main]` before the function. **NEED FURTHER ANALYSIS**
