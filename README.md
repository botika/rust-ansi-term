# yansi-term 
> Adapted from [`rust-ansi-term`](https://github.com/ogham/rust-ansi-term)
>
> Refactor for use [`fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) and `FnOnce(&mut fmt::Formatter) -> fmt::Result` 


This is a library for controlling colours and formatting, such as red bold text or blue underlined text, on ANSI terminals.

### [View the Rustdoc](https://docs.rs/ansi_term/)


# Installation

This crate works with [Cargo](http://crates.io). Add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
ansi_term = "0.12"
```


## Basic usage

There are three main types in this crate that you need to be concerned with: `ANSIString`, `Style`, and `Colour`.

A `Style` holds stylistic information: foreground and background colours, whether the text should be bold, or blinking, or other properties.
The `Colour` enum represents the available colours.
And an `ANSIString` is a string paired with a `Style`.

`Color` is also available as an alias to `Colour`.

To format a string, call the `paint` method on a `Style` or a `Colour`, passing in the string you want to format as the argument.
For example, here’s how to get some red text:

```rust
use ansi_term::Colour::Red;

println!("This is in red: {}", Red.paint("a red string"));
```

It’s important to note that the `paint` method does *not* actually return a string with the ANSI control characters surrounding it.
Instead, it returns an `ANSIString` value that has a `Display` implementation that, when formatted, returns the characters.
This allows strings to be printed with a minimum of `String` allocations being performed behind the scenes.

If you *do* want to get at the escape codes, then you can convert the `ANSIString` to a string as you would any other `Display` value:

```rust
use ansi_term::Colour::Red;

let red_string = Red.paint("a red string").to_string();
```

**Note for Windows 10 users:** On Windows 10, the application must enable ANSI support first:

```rust,ignore
let enabled = ansi_term::enable_ansi_support();
```

## Bold, underline, background, and other styles

For anything more complex than plain foreground colour changes, you need to construct `Style` values themselves, rather than beginning with a `Colour`.
You can do this by chaining methods based on a new `Style`, created with `Style::new()`.
Each method creates a new style that has that specific property set.
For example:

```rust
use ansi_term::Style;

println!("How about some {} and {}?",
         Style::new().bold().paint("bold"),
         Style::new().underline().paint("underline"));
```

For brevity, these methods have also been implemented for `Colour` values, so you can give your styles a foreground colour without having to begin with an empty `Style` value:

```rust
use ansi_term::Colour::{Blue, Yellow};

println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
```

The complete list of styles you can use are:
`bold`, `dimmed`, `italic`, `underline`, `blink`, `reverse`, `hidden`, and `on` for background colours.

In some cases, you may find it easier to change the foreground on an existing `Style` rather than starting from the appropriate `Colour`.
You can do this using the `fg` method:

```rust
use ansi_term::Style;
use ansi_term::Colour::{Blue, Cyan, Yellow};

println!("Yellow on blue: {}", Style::new().on(Blue).fg(Yellow).paint("yow!"));
println!("Also yellow on blue: {}", Cyan.on(Blue).fg(Yellow).paint("zow!"));
```

You can turn a `Colour` into a `Style` with the `normal` method.
This will produce the exact same `ANSIString` as if you just used the `paint` method on the `Colour` directly, but it’s useful in certain cases: for example, you may have a method that returns `Styles`, and need to represent both the “red bold” and “red, but not bold” styles with values of the same type. The `Style` struct also has a `Default` implementation if you want to have a style with *nothing* set.

```rust
use ansi_term::Style;
use ansi_term::Colour::Red;

Red.normal().paint("yet another red string");
Style::default().paint("a completely regular string");
```


## Extended colours

You can access the extended range of 256 colours by using the `Colour::Fixed` variant, which takes an argument of the colour number to use.
This can be included wherever you would use a `Colour`:

```rust
use ansi_term::Colour::Fixed;

Fixed(134).paint("A sort of light purple");
Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup");
```

The first sixteen of these values are the same as the normal and bold standard colour variants.
There’s nothing stopping you from using these as `Fixed` colours instead, but there’s nothing to be gained by doing so either.

You can also access full 24-bit colour by using the `Colour::RGB` variant, which takes separate `u8` arguments for red, green, and blue:

```rust
use ansi_term::Colour::RGB;

RGB(70, 130, 180).paint("Steel blue");
```
