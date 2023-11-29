# Create your own LS in Rust

## About the workshop

This workshop expects some familiarity with Rust, and introduces some ecosystem crates while trying to maintain idiomatic Rust code, but does not always succeed. 🥲

## 🔤 Get going

- Install rustup https://rustup.rs/
- This workshop was made on Rust 1.72.0 but should work on some earlier versions as well

## Assignments

### Emojis 😺

You'll see some emojis throughout the workshop. They have the following meaning:

- 🏆 Assignment: This is what you're going to do
- 💡 Tips: Some extra info that could be nice to solve an assignment, or simply just a tip
- 🧪 Testing: How you can test your solution to see that it works
- 🚨 Proposed solution: A proposal on how to solve the assignment, which may not be the way you did it.

---

### 🤔 0. Motivation

We want to create our own version of the terminal file explorer `ls` in Rust, just because we can. This way, we get to experience some of Rust's [standard library](https://std.rs), some ecosystem [crates](https://crates.io/) (packages), some newtype patterns and some error handling.

Be explorative, use go to definition in your editor to see how code is implemented, use https://docs.rs (or just use `cargo doc --open` locally) to see how to use packages if you want to learn more, and so on.

### 📦 1. Create the project

We need to create our project before we start. Find a good name, or just use what I came up with: `explore`.

🏆 Create a new Cargo project called `explore` using `cargo new`

<details>
<summary> 🚨 Solution</summary>

```shell
> cargo new explore
     Created binary (application) `explore` package
```

</details>

---

### 📜 2. Listing out files

To start off, let's use the current working directory as a starting point to list out all files and folders. After all, that's that `ls` does with no input arguments.

We will use [walkdir](https://docs.rs/walkdir/latest/walkdir/) as it solves our problems of listing all files and directories, plus recursively.

🏆 Add Walkdir as a dependency using `cargo add` and then use a for-loop and `WalkDir::new` to start looping. Use `.path().display()` on an entry to get a formattable entry you can print to the terminal. Unwrap any errors you encounter

🧪 Run `cargo run` afterwards to see that you get an output of the files and folders in the code project (should be a lot of files, with git- and build/debug-stuff)

<details>
<summary> 🚨 Solution</summary>

First, let's add Walkdir:

```shell
> cargo add walkdir
    Updating crates.io index
      Adding walkdir v2.4.0 to dependencies.
```

Let's modify our `./src/main.rs` code to loop over all files and directories and print them:

```rust
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".") {
        println!("{}", entry.unwrap().path().display());
    }
}
```

</details>

---

### 🥅 3. Filtering out entries

If you did the previous assignment correctly, there should have been a lot of entries outputted. Including git files, folders, build artifacts from `target`, and more. This is kind of noisy, but nice that walkdir does that for us.

However, we can filter out some of the entries by specifying the wanted minimum and maximum depth walkdir should "walk" through our file hierarchy.

🏆 Specify the minimum and maximum depth to the `WalkDir`-builder to list out only things with depth 1.

🧪 Run `cargo run` afterwards to see that you get an output of only the following:

<details>
<summary>

Output of `explore` when done correctly

</summary>

```shell
> cargo run --quiet # prints no build info
./Cargo.toml
./target
./Cargo.lock
./.gitignore
./.git
./src
```

</details>

<details>
<summary> 🚨 Solution</summary>

Add `.min_depth(1).max_depth(1)` to the `WalkDir`-builder to filter out entries like `.` and all the files nested in folders.
Modify the for-loop to:

```rust
for entry in WalkDir::new(".").min_depth(1).max_depth(1)
```

</details>

---

### 🤼‍♂️ 4. Allowing us to specify min and max depth

Hardcoding things is not nice, so it would be nice to allow specifying minimum and maximum depth to traverse on the command line when running. E.g. so you can run `explore --min-depth 1 --max-depth 3` instead.

Instead of using the built in [`std::env::args`](https://doc.rust-lang.org/stable/std/env/fn.args.html)-function here, we'll skip right ahead to using a proper **c**ommand **l**ine **i**nput **p**arser: [Clap](https://docs.rs/clap/latest/clap/).

This library will add automatic help text, create autocomplete scripts for a bunch of shells, parse input for you, and more.

1. 🏆 Add `clap` using `cargo add` enabling the `derive` feature to allow it to generate a bunch of code for us.
2. 🏆 Add an `Options`/`Arguments`-struct to hold your parsed arguments, using `#[derive(Parser, Debug)]` to add the parser-functionality and `Debug` to allow us to debug-print the options. Read the clap documentation on how to add `min-depth` and `max-depth` as options.
3. 🏆 Use `Options::parse()` to extract the input arguments
4. 🏆 Modify the `.min_depth(1).max_depth(1)`-hardcoding to instead use the numbers from our options-struct (use a default value of 1)

<details>
<summary> 🚨 Solution 1</summary>
Run the following:

```shell
> cargo add clap -F derive
    Updating crates.io index
      Adding clap v4.4.8 to dependencies.
             Features:
             + color
             + derive
             + error-context
             + help
             + std
             + suggestions
             + usage
             - cargo
             - debug
             - deprecated
             - env
             - string
             - unicode
             - unstable-doc
             - unstable-styles
             - unstable-v5
             - wrap_help
```

</details>

<details>
<summary> 🚨 Solution 2</summary>

Create our `Options`-struct with our desired options

```rust
struct Options {
    // These cannot be negative, so we use `usize` as a type
    min_depth: usize,
    max_depth: usize
}
```

Add the derive-macro to generate parse-functionality on the structure

```rust
#[derive(Parser, Debug)]
struct Options {
```

Add info about what Clap should generate for us:

```rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
```

This will generate the author name (your name!), the version and the about description from the `Cargo.toml`-file to the help-output.

Now add the `#[arg]`-attribute to each field, saying we only want the long-format, e.g. `--min-depth 3` to be allowed, not `-m 3` or something:

```rust
#[arg(long, default_value_t = 1)]
min_depth: usize,

#[arg(long, default_value_t = 1)]
max_depth: usize,
```

</details>

<details>
<summary> 🚨 Solution 3</summary>

Simply add the following to our `main`-function:

```rust
let options = Options::parse();
```

</details>

<details>
<summary> 🚨 Solution 4</summary>

Get the min and max-values from the `options`-value and modify our `WalkDir`-builder:

```rust
for entry in WalkDir::new(".")
    .min_depth(options.min_depth)
    .max_depth(options.max_depth)
```

Now you're done!

</details>

🧪 Run `cargo run` afterwards to see that you get an output that is the same as in assignment 3!

🧪 Run `cargo run -- --min-depth 1 --max-depth 3` to see that you get the output of a couple folders down in addition to the root-level output. (Arguments can be passed through Cargo using `--` so that everything that follows is passed to our program)

<details>
<summary>Output of this test</summary>

```shell
> cargo run --quiet -- --min-depth 1 --max-depth 3
./Cargo.toml
./target
./target/.rustc_info.json
./target/CACHEDIR.TAG
./target/debug
./target/debug/.fingerprint
./target/debug/incremental
./target/debug/explore.d
./target/debug/explore
./target/debug/.cargo-lock
./target/debug/examples
./target/debug/deps
./target/debug/build
./Cargo.lock
./.gitignore
./.git
./.git/config
./.git/objects
./.git/objects/32
./.git/objects/33
./.git/objects/d7
./.git/objects/pack
./.git/objects/72
./.git/objects/88
./.git/objects/38
./.git/objects/info
./.git/objects/3a
./.git/objects/30
./.git/objects/5e
./.git/objects/39
./.git/objects/0a
./.git/objects/a0
./.git/objects/b7
./.git/objects/c3
./.git/objects/ea
./.git/objects/e7
./.git/objects/8d
./.git/HEAD
./.git/info
./.git/info/exclude
./.git/logs
./.git/logs/HEAD
./.git/logs/refs
./.git/description
./.git/hooks
./.git/hooks/README.sample
./.git/refs
./.git/refs/heads
./.git/refs/tags
./.git/index
./.git/COMMIT_EDITMSG
./src
./src/main.rs
```

</details>

<details>
<summary>

💡 *Help*ful tip

</summary>

Try to run `cargo run -- --help` to output the help info!

```shell
> cargo run -q -- --help
Usage: explore [OPTIONS]

Options:
--min-depth <MIN_DEPTH> [default: 1]
--max-depth <MAX_DEPTH> [default: 1]
-h, --help Print help
-V, --version Print version
```

</details>

---

### ✨ 5. Spice things up with colors

Plain white output is not for everyone, so add some colors using [colored](https://docs.rs/colored/latest/colored/) to color each output type differently. E.g. color files as white, directories blue and symbolic links yellow.

🏆 Add `colored` using `cargo add`. Use the convenience methods `is_file` and friends on `entry` to print out entries in different colors using colored's `Colorize`-trait. Auto-import in an IDE helps out importing the needed trait.

🧪 Run your program to output files in colors!

💡 `Colorize` is [implemented for string-types](https://docs.rs/colored/latest/colored/trait.Colorize.html#foreign-impls), but our `entry.unwrap().path().display()` returns a `Display`-type that is displayable (implements [`Display`](https://doc.rust-lang.org/stable/std/fmt/trait.Display.html)). Perhaps you need to have a string before coloring?

<details>
<summary> 🚨 Solution</summary>

Add `colored`:

```shell
> cargo add colored
```

In our main loop, extract the entry to its own variable. Use if-elses (since `is_file`-methods are methods we cannot use match patterns :'( ) to display the line in different colors using colored. Rewrite the body of the for-loop as:

```rust
let entry = entry.unwrap();
let path = entry.path();
let formatted_entry = if path.is_file() {
    path.display().to_string().white()
} else if path.is_dir() {
    path.display().to_string().blue()
} else {
    // We'll assume symlinks
    path.display().to_string().yellow()
};

println!("{}", formatted_entry);
```

</details>

---

### 🤏 6. Display sizes

Now that we have colors, we can move on to actual functionality. We do not know how large our entries are, so let's figure that out and display it. You can use your own formatting, but I'll display it something like this:

```shell
> cargo run -q
   123 B     entry.rs
  9001 B      main.rs
```

🏆 Use the metadata of an entry to get the size of it (ignore that directory entry sizes may be inode/file-info sizes) and display it before the entry name. Try to align it so that sizes are right-aligned (or left-aligned if you want to, but aligned regularly for prettier output).

💡 The standard library documentation includes info on [how to format and align](https://doc.rust-lang.org/stable/std/fmt/index.html#fillalignment).

<details>
<summary>🚨 Solution</summary>

Extract the size of the metadata and print it before the entry. I'll tab-separate them and align sizes to be 9 characters in width, and entries as 15 (this is not the proper way to do it but will work for now).

Change your printing logic to:

```rust
let size = entry.metadata().unwrap().len();

println!("{:>9} B\t{:>15}", size, formatted_entry);
```

</details>

---

### 👓 7. Make sizes human-readable

If we look at one of the lines of output from our program now:

```shell
> cargo run -q
     9471 B        ./Cargo.lock
```

We can see that sizes are large and in bytes. We do no formatting on the size itself, and for larger files, reading the amount in bytes is impractical. It would be nicer if it was read in kilobytes, megabytes, and so on, for larger sizes. Luckily, [bytesize](https://docs.rs/bytesize/latest/bytesize/) does this for us.

🏆 Add bytesize and use it to format sizes in a human-readable format.

💡 Tip: it's easier than you think

<details>
<summary>🚨 Solution</summary>

Add bytesize:

```shell
> cargo add bytesize
```

Then where we create our size, wrap the size in the `ByteSize` constructor, which implements format, and remove the `B` as bytesize now does that for us:

```rust
// Add the import as needed
let size = ByteSize(entry.metadata().unwrap().len());

println!("{:>9}\t{:>15}", size, formatted_entry);
```

Running this code will now output the same for smaller files, but this for the entry we saw earlier:

```shell
   9.7 KB          ./Cargo.lock
```

</details>

---

### ❓ 8. Handling errors?

Our program may encounter errors a couple of places. Currently, we use `.unwrap()` which simply crashes the entire program when encountering an error, or otherwise unwraps the "good" value for later use.
In the Rust ecosystem, users have collectively agreed over time on two rule of thumbs:

1. If you're handling errors in a library, you should create your own error types and use [thiserror](https://docs.rs/thiserror/latest/thiserror/) to describe them in error reports.
2. If you're writing an application-level program, you should use [anyhow](https://docs.rs/anyhow/latest/anyhow/). This let's you easily return errors from functions as long as they implement the [`std::error::Error`](https://doc.rust-lang.org/stable/std/error/trait.Error.html)-trait.

As always with rules like these, there are exceptions.
Another way to look at it is: are you supposed to be able to match and handle the error or are you going to just display it to a user?

For now, we'll go the easy way out and use anyhow as we are creating an application and we want to just display the errors to the users.

🏆 Add anyhow and use the `anyhow::Result` as a return type (or just `Result<(), anyhow::Error` as the former is an alias for the latter)

💡 Tip: Instead of using `match` at every error, perhaps you could simplify things with [`?`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)?

<details>
<summary>🚨 Solution</summary>

Add anyhow:

```shell
> cargo add anyhow
```

#### Info on the try-operator

We can use the try-operator `?` to return our errors early, or unwrap
the item inside if it is a success instead.

The `?`-operator is implemented for `Option` and `Result` and is currently under [development for stabilization](https://github.com/rust-lang/rust/issues/84277) in the form of a trait so that you could implement it for your own types. For `Option`s and `Result`s, the following snippets are the same:

```rust
fn thing(x: X) -> Result<(), Error> {
    match x.thing_that_can_fail() {
        Ok(()) => Ok(()),
        Err(e) => return e.into(),
    }
}
```

and:

```rust
fn thing(x: X) -> Result<(), Error> {
    Ok(x.thing_that_can_fail()?)
}
```

#### Back to our code

Change the return type of `main` to `anyhow::Result<()>` as we can simply return `()` ([unit](https://doc.rust-lang.org/stable/std/primitive.unit.html)) to denote nothing is returned in the OK-case.

```rust
fn main() -> anyhow::Result<()>
```

Replace our `.unwrap()`s with `?`:

```rust
let entry = entry?;
```

and

```rust
let size = ByteSize(entry.metadata()?.len());
```

And finally, return unit wrapped in an `Ok` at the end if nothing else failed:

```rust
    // At the end of `main`
    Ok(())
```

Now just run the code using `cargo run` to see that it still works the same!
I'm not sure on how to provoke any errors from the filesystem, but maybe running the code on directories you don't have permissions to read will work?

</details>

---

### 🤓 9. The numbers, what do they mean? Introducing headers

If you're like me, you might not always understand what `123 B` really means, so let's add some headers to our output to describe what the numbers and the entry names mean.

1. 🏆 Add an option `--headers` to your program which prints out `Size` and `Name` over the size and entry names.
2. 🏆 Spice up the headers and make them bold and underlined

💡 Tip: colored does more than coloring things!

<details>
<summary>🚨 Solution 1</summary>

Add `--headers` as a possible command-line argument to our options struct:

```rust
#[arg(long, default_value_t = false)]
headers: bool,
```

Before the main loop, add an if-statement which prints the headers first:

```rust
if options.headers {
    println!("{:>9}\t{:>15}", "Size", "Name");
}
```

You can now test the output to see the headers:

```shell
> cargo run -q -- --headers
     Size                  Name
    301 B          ./Cargo.toml
    160 B              ./target
   9.9 KB          ./Cargo.lock
      8 B          ./.gitignore
    384 B                ./.git
     96 B                 ./src
```

</details>

<details>
<summary>🚨 Solution 2</summary>

Modify the printing logic of the headers to

```rust
if options.headers {
    println!("{:>9}\t{:>15}", "Size".bold().underline(), "Name".bold().underline());
}
```

You should now see that they are underlined. However, because of our padding, it underlines the empty spaces as well. It's not really needed, but if you want to, you can fix it at the cost of uglier code:

```rust
println!(
    // 5 because "Size" is 4 characters and 5 = 9 - 4
    "{:>5}{}\t{:>11}{}",
    // "" because we'll pad first without underlining
    "",
    // Our underlined and bold header text, aligned
    "Size".bold().underline(),
    // The same for "Name", 11 = 15 - 4
    "",
    "Name".bold().underline()
);
```

Hey, I told you it ain't pretty, but it works.

</details>

---

### 😱 10. Not hardcoding the path

We talked about not hardcoding things all the way back in assignment 4, but we've been harcoding our path since the beginning in `WalkDir::new(".")`!

🏆 Add an argument `--path` which accepts a path to some directory and use that to list out entries, defaulting to `"."` if none is given.

💡 Tip: It's easiest to provide Clap with owned types in the `Options`-struct. Also remember that [`std::path::PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) exists for easy and robust cross-platform path-handling.

🧪 Test your application by using `cargo run -q -- -p ./src` or some other directory. Compare it to `cargo run -q`. They should be different!

<details>
<summary>🚨 Solution</summary>

Add the following to our `Options`-struct:

```rust
// It's easier to use `Option<PathBuf>` here and provide a default in the code.
// Otherwise we will have to provide Clap something that can be displayed for the help text
#[arg(short, long)]
path: Option<PathBuf>,
```

Change your `WalkDir::new(".")` to use the provided path or defaulting to the old one:

```rust
WalkDir::new(options.path.unwrap_or(".".into()))
```

Running our code now gives:

```shell
> cargo run -q
    301 B          ./Cargo.toml
    160 B              ./target
   9.9 KB          ./Cargo.lock
      8 B          ./.gitignore
    384 B                ./.git
     96 B                 ./src
```

Or with a different path:

```shell
> cargo run -q -- -p ./src
   1.4 KB         ./src/main.rs
```

</details>

---

### 🤫 11. Hiding the hidden files

I just noticed that our program outputs hidden files and directories by default. We probably don't want that, so let's add an option to toggle it, while defaulting to hiding hidden files.

🏆 Add `--hidden` to show hidden files and directories, like `.git`, defaulting to not showing them.

💡 Tip: walkdir has some functionality to filter out entries

🧪 Run `cargo run -q -- --hidden` and `cargo run -q` and see that the former has `.git` while the latter doesn't.

<details>
<summary>🚨 Solution</summary>

Add our option:

```rust
#[arg(long, default_value_t = false)]
hidden: bool,
```

Convert our `WalkDir`-builder into an iterator and use the walkdir-provided `.filter_entry` to filter entries. But first, make a helper function `is_hidden`:

```rust
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
```

And then the builder:

```rust
WalkDir::new(options.path.unwrap_or(".".into()))
    .min_depth(options.min_depth)
    .max_depth(options.max_depth)
    // 👇 The following is new
    .into_iter()
    // Either the option to show hidden
    .filter_entry(|entry| options.hidden || !is_hidden(entry))
```

</details>

---

### 💚 12. Coloring our sizes

How could we forget! Our sizes are just plain white, while our output otherwise looks fabulous! Let's color the size output.

🏆 Color the size output in green

💡 Tip: `ByteSize` is not a string, required by `Colorize`, but it can be converted to one. You've already done this once.

<details>
<summary>🚨 Solution</summary>

Use the `format!`-macro to convert the size to a string first, then color it green:

```rust
let size = format!("{}", ByteSize(entry.metadata()?.len())).green();
```

</details>

---

### 🧹 13. Cleaning up our mess

As you might have seen, we now have some functionality, but our main for-loop iterating over the entries is a bit cluttered. To clean this up, we'll move the formatting of the entry and the formatting of the size to their own abstractions.

Because we end up displaying them using the `println!`-macro, which in turn uses the [`std::fmt::Display`](https://doc.rust-lang.org/stable/std/fmt/trait.Display.html)-trait. As such, it would make sense to just implement display for the entry and the size. However, we cannot simply `impl Display for bytesize::ByteSize`, because of something called [coherence](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence). Coherence is a rule in Rust to prevent breaking changes while implementing traits. It is an extension of the _orphan rule_ from Haskell. Essentially, the rules dictate that we cannot implement a foreign trait on a foreign type. And since `Display` is foreign (comes from the crate `std`), and `ByteSize` is a foreign type (from `bytesize`), it is not allowed.

The common practice to fix this is to use the newtype pattern, or [new type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). The newtype pattern in Rust means using a type to wrap another type. In our case, we will use what is called a _tuple struct_, or an anonymous struct, since we're not naming the field. Just as an aside, let's look at the different types of anonymous/named types in Rust:

|        | Named                           | Anonymous             |
| ------ | ------------------------------- | --------------------- |
| Enum   | `enum Thing { Thang }`          |                       |
| Struct | `struct Thing { thang: Thang }` | `struct Thing(Thang)` |

As you can see, everything except anonymous enums exist in Rust today. Anonymous enums wouldn't make much sense anyway.

1. 🏆 Create a newtype `FormatEntry` which wraps a reference to an entry.
2. 🏆 Implement `std::fmt::Display` for that entry to retain the functionality we have now, and use the `FormatEntry`-type in our main loop instead of doing the formatting logic there directly.

Tips:

1. 💡 If you're getting issues with lifetimes, perhaps rust-analyzer code actions can help you out?
2. 💡 Use rust-analyzer code actions to implement missing members in the trait implementation. Also, the scary-but-not-so-scary type `&mut std::fmt::Formatter<'_>` has some [convenience methods](https://doc.rust-lang.org/stable/std/fmt/struct.Formatter.html#method.write_fmt) that are useful.

🧪 Run `cargo test -q` before and after your changes to ensure the output is the same

<details>
<summary>🚨 Solution 1</summary>

Create our newtype struct `FormatEntry` which holds a reference to a `DirEntry`:

```rust
struct FormatEntry(&DirEntry)
```

However, as you might see from the compiler output. Holding a reference inside a type requires the compiler to know the lifetime of the reference. The actual lifetime is inferred by the compiler, but it needs to know that the value is borrowed from the place `FormatEntry` is created from. To do this, we can use the rust-analyzer code action "consider introducing a named lifetime parameter 'a":

```rust
struct FormatEntry<'a>(&'a  DirEntry);
```

Even though it is really common, there's no need for `'a` to be named such, we can call it any identifier, so let's use something more descriptive, such as the location we borrow it from:

```rust
struct FormatEntry<'walk_dir_loop>(&'walk_dir_loop DirEntry);
```

</details>

<details>
<summary>🚨 Solution 2</summary>

We need to create our entry in the main loop. Replace the entire code part of formatting the entries with different colors and extracting the path with this:

```rust
let formatted_entry = FormatEntry(&entry);
```

This will spit out an error located at the printing location that

> `FormatEntry<'_>` doesn't implement `std::fmt::Display`

As this is how Rust converts values for being displayed as strings, we simply need to implement `Display`. Write:

```rust
impl<'walk_dir_loop> Display for FormatEntry<'walk_dir_loop> {}
```

Now it will complain that it is missing members that need to be implemented, namely `fn fmt`. Use rust-analyzer to implement the missing members so that it generates this:

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
}
```

Now, put all of our for-loop logic at the top of this function:

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Use `self.0` to access the zeroth element of the tuple struct
    let path = self.0.path();
    let formatted_entry = if path.is_file() {
        path.display().to_string().white()
    } else if path.is_dir() {
        path.display().to_string().blue()
    } else {
        // We'll assume symlinks
        path.display().to_string().yellow()
    };
    todo!()
}
```

Replace the `todo!`-macro with `f.write_fmt` to write formatted output. Use the `format_args!`-macro to generate the needed type (with alignment):

```rust
f.write_fmt(format_args!("{formatted_entry:>15}"))
```

Running it now should provide the same output as before!

</details>

---

### 🧯 14. Cleaning up size formatting

Now that the business logic of our code is a bit cleaner and the details are pushed away, let's do the same thing to the size formatting.

1. 🏆 Create a `FormatSize` newtype wrapper-struct that wraps a reference to a `DirEntry`, like before.
2. 🏆 Implement display for it

💡 Implementing display by copy-pasting like before may be more difficult than imagined... Should we go back to unwrapping? It's alright to unwrap again...

<details>
<summary>🚨 Solution 1</summary>

Create the struct like before:

```rust
struct FormatSize<'walk_dir_loop>(&'walk_dir_loop DirEntry);
```

</details>

<details>
<summary>🚨 Solution 2</summary>

Replace the for-loop logic for formatting the size with:

```rust
let size = FormatSize(&entry);
```

Implement Display for our code, copy-pasted from the for-loop (and using `self.0` instead of size):

```rust
impl<'walk_dir_loop> Display for FormatSize<'walk_dir_loop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:>9}", ByteSize(self.0.metadata()?.len())).green())
    }
}
```

Oh no! This gives an error for the `?`-operation, because:

> `?` couldn't convert the error to `std::fmt::Error`

This makes sense, as the metadata's error is not meant to be immediately displayed, but programmatically handled. Let's use `.unwrap()` instead of `?`.

## </details>

---

### 🚮 15. Handling our error once again

If you wrote your code as in the solution above, we replaced error-handling with `?` with `.unwrap()`. Unwrapping and panicking/crashing is unacceptable, so let's fix it again. We introduced an abstraction which cleaned up our business logic, but made error-handling worse as a result.

We'll modify some things, such as `FormatSize` holding a reference to a `DirEntry`, to it holding a `Metadata`-object. To do this, we'll need to _try_ to convert from a `DirEntry` to a `FormatSize`, returning a failure if it went wrong. This let's us handle the error properly.

We've used `.into()` on several occasions, which comes from the `Into`-trait, reverse of the [`From`](https://doc.rust-lang.org/stable/std/convert/trait.From.html)-trait. Now, we'll look at the fallible variants, `TryInto` and [`TryFrom`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html).

1. 🏆 Change `FormatSize` to hold a [`std::fs::Metadata`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html) and change the formatting logic for the `FormatEntry` to not extract the metadata.
2. 🏆 Implement `TryFrom<&DirEntry>` for `FormatSize`

<details>
<summary>🚨 Solution 1</summary>

Change the struct to hold the metadata reference and correct the display logic:

```rust
struct FormatSize(Metadata);

impl Display for FormatSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:>9}", ByteSize(self.0.len())).green())
    }
}
```

❗️ Note that this will not compile yet

Rewrite the call-site in our main loop to:

```rust
let size = FormatSize::try_from(&entry)?;
```

</details>

<details>
<summary>🚨 Solution 2</summary>

`impl TryFrom<&DirEntry> for FormatSize`:

```rust
impl<'walk_dir_loop> TryFrom<&'walk_dir_loop DirEntry> for FormatSize {
    type Error = anyhow::Error;

    fn try_from(entry: &DirEntry) -> Result<Self, Self::Error> {
        Ok(Self(entry.metadata()?))
    }
}
```

The `type Error = anyhow::Error;` here is an [associated type](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=associated%20type#specifying-placeholder-types-in-trait-definitions-with-associated-types). It sounds fancier than it is. In this case it is only so that `TryFrom` can reference `Self::Error` in the return type without knowing which error you are going to implement the trait for.

Cool!

</details>

---

### 😔 16. Realizing our overengineering mistakes

Well, it was fun while it lasted. You may have noticed during our last assignment that since we changed `FormatSize` to just hold a `Metadata`, we could just do the error-handling in the main loop and not have to implement `TryFrom` for it. While I love the trait system and think this solution is cooler, i tend to like [KISS](https://en.wikipedia.org/wiki/KISS_principle)ing my code more.

🏆 Remove the `TryFrom`-implementation and move error-handling out into the main loop...

🧪 It should work the same as before

<details>
<summary>🚨 Solution</summary>

Change the main loop logic to:

```rust
let size = FormatSize(entry.metadata()?);
```

And remove the `impl TryFrom` item.

</details>

---

### 🧂 17. Spicing up our error-messages

We've used anyhow for error-handling, but missed out on [contexts](https://docs.rs/anyhow/latest/anyhow/trait.Context.html)! Contexts give better contexts to errors for more easily figuring out why an error occurred.

🏆 Use the `anyhow::Context::context`-method to add context to errors

💡 Remember that non-static trait-methods that have a receiver (e.g. `self`, `&self`, `&mut self`, and so on) can be called as a regular method: `error.context("this failed in the context of blah and blah")?`.

<details>
<summary>🚨 Solution</summary>

Use ctrl-f to search for `?` in our code. Before the `?`-operator, add `.context("")` with a descriptive message in the quotes. This is only really two places where we can't really be super descriptive, but we will try:

```rust
let entry = entry.context("Error getting file entry")?;
let size = FormatSize(entry.metadata().context(format!(
    "Failed extracting metadata for {}. Perhaps you are missing permissions?",
    entry.path().display()
))?);
```

</details>

---

### ✂️ 18. Stripping off prefixes

Printing `./src`, `./Cargo.toml` and such is not really that pretty. Why do we do it? Let's strip that prefix from the entries:

🏆 Strip prefixes from entries so that `./target/debug/explore` becomes `explore`, `./src` becomes `src`, and so on.

💡 Tip: A [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) is also an iterator

🧪 Running `cargo run -q -- --hidden --headers -p src` should print an entry with the name `main.rs` and no prefix.

<details>
<summary>🚨 Solution</summary>

In our implementation of `Display` for `FormatEntry` we can modify this part:

```rust
let path = self.0.path();
```

Instead of printing the entire path, let's pick out only the last element. We also know that we will have at least one element, either `.`, or whatever directory path you gave as an input, so unwrapping the last element of an iterator (which normally gives us an option because the iterator might be empty), is actually safe:

```rust
let path = self.0.path();
// SAFETY: We can safely unwrap here as we know the path contains at least one part (e.g. `.` or `./thing`, or so on)
let name = path.iter().last().unwrap();
```

Unfortunately, because the standard library of Rust tries to be as cross-platform as possible, we get an opaque `&OsStr` back here. So this code fails to compile. The name may or may not be utf-8, which all Rust strings are and which colored requires. In our program I would say it is fair to assume we are working with files that can be displayed with utf-8. To do this, we can use [`OsStr::to_string_lossy`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.to_string_lossy):

```rust
// SAFETY: We can safely unwrap here as we know the path contains at least one part (e.g. `.` or `./thing`, or so on)
let name = path.iter().last().unwrap().to_string_lossy();
// Use `name` instead of path in the coloring below:
let formatted_entry = if path.is_file() {
    name.white()
} else if path.is_dir() {
    name.blue()
} else {
    name.yellow()
};
```

Fixing this another way is left as an exercise to the reader.

</details>

---

### 👉 19. Indenting our misindented mistakes

Ever since we stripped of the prefixes in last assignment, we also stripped off any leading folders.
This can easily be seen if we run:

```shell
> cargo run -q -- --max-depth 3
    301 B            Cargo.toml
    160 B                target
    177 B          CACHEDIR.TAG
    320 B                 debug
    672 B           incremental
    136 B             explore.d
   3.7 MB               explore
     64 B              examples
  15.6 KB                  deps
    320 B                 build
   9.9 KB            Cargo.lock
     96 B                   src
   2.6 KB               main.rs
```

Unfortunately, we've lost the ability to see which file belongs to which folder unambiguously. Let's fix it by indenting our files contained in each folder by one indentation for each step.

🏆 Indent each nested file by one level for each folder it's contained within. Use whatever indentation string sequence you want, for example the tab character `'\t'` or something else. Go crazy! It should look something like this:

```shell
> cargo run -q -- --max-depth 3
    320 B       Mon, 20 Nov 2023 15:44:08       Cargo.toml
    192 B       Tue, 28 Nov 2023 17:59:57       target
    177 B       Mon, 13 Nov 2023 13:32:33         CACHEDIR.TAG
    320 B       Tue, 28 Nov 2023 18:08:26         debug
    832 B       Mon, 20 Nov 2023 16:07:54           incremental
```

🧪 Run your code with `cargo run -q -- --max-depth 3` and see that the indentation levels match the above output.

💡 Tip: `DirEntry` has a method `.depth()` that may be very useful in combination with [repeating](https://doc.rust-lang.org/stable/std/primitive.str.html#method.repeat) some text.

<details>
<summary>🚨 Solution</summary>

We'll change our `impl Display for FormatEntry`-code to indent each entry using the result of `.depth()`. From the documentation we can see that `depth` does exactly what we want it to. Gives 0 as a response for the root-level entry of where `WalkDir` started out, increasing by one for every descendant after.

We'll first create our indentation sequence. Let's use a string reference, and since it won't change, we'll save it as a `const`:

```rust
const INDENTATION_SEQUENCE: &str = "⤷ ";
```

I went for a fancy arrow, but it is kind of ugly at the same time. You do you.
Now, `str::repeat` does exactly what we want, returning a `String` with our sequence repeated `n` times. Since `DirEntry::depth` gives us `0` for the root, it gives `1` for the first level, but we're only interested in everything from the first level and out, so we don't want an indentation level for the first children of entries. Let's use `self.0.depth() - 1` instead, then format the indent in a dimmed style:

```rust
let indent = INDENTATION_SEQUENCE
    .repeat(self.0.depth() - 1)
    .dimmed();

f.write_fmt(format_args!("{indent}{formatted_entry}"))
```

And that's it!

**However**. Do you notice something? We allowed for `--min-depth` as an input, defaulting to `1` if not provided. But we could provide `0` instead. `0 - 1` is `-1`, which might be okay, but is actually an underflow. During testing I did not encounter this even when I provoked an error, but let's handle it anyway, using `.saturating_sub`, a method which saturates the number to its type's limits (0 - 1 is just 0, the same for the max values + 1 is the max values).
Then our code becomes:

```rust
const INDENTATION_SEQUENCE: &str = "⤷ ";

let indent = INDENTATION_SEQUENCE
    .repeat(self.0.depth().saturating_sub(1))
    .dimmed();

f.write_fmt(format_args!("{indent}{formatted_entry}"))
```

</details>

---

### ⏰ 20. About time for a change!

We don't know when files and folders are created! What kind of file explorer is that. Add the modified date of each file/folder and present it using [chrono](https://docs.rs/chrono/latest/chrono/). Rust has support for dates in the standard library, but doesn't support formatting them (trust me, with the amount of weird stuff going on with dates/timestamps and the stability promises of a standard library, this is really a good thing).

1. 🏆 Add a flag for showing the date modified
2. 🏆 Add a header if the flag is provided
3. 🏆 Add chrono
4. 🏆 Fetch the modified timestamp from the metadata, assume they are UTC, and format them using the RFC2822 format

💡 Tip: Perhaps chrono has some way of converting _from_ the time modified?
💡 Tip: If you encounter a weird `+0000`, just _strip_ it away

<details>
<summary>🚨 Solution 1</summary>

Adding a flag, same procedure as always: modify the `Options`-struct to contain this:

```rust
#[arg(long, default_value_t = false)]
modified: bool,
```

</details>

<details>
<summary>🚨 Solution 2</summary>

Change the header-code to the following (the formatting stuff is kind of nasty, so I'll not get into it in detail):

```rust
if options.headers {
    let modified = if options.modified {
        format!(
            "{:>25}",
            format!("\t{}\t", "Modified at".bold().underline())
        )
    } else {
        "".into()
    };

    println!(
        "{:>5}{}{}\t{:>11}{}",
        "",
        "Size".bold().underline(),
        modified,
        "",
        "Name".bold().underline()
    );
}
```

</details>

<details>
<summary>🚨 Solution 3</summary>

Same as before, run:

```shell
> cargo add chrono
```

</details>

<details>
<summary>🚨 Solution 4</summary>

Now the juicy part.
Create a `FormatModifiedAt`-newtype as before, holding the `SystemTime` we get from `Metadata::modified`.

```rust
struct FormatModifiedAt(SystemTime);
```

Implement `Display` for it and generate the boilerplate:

```rust
impl Display for FormatModifiedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
```

Replace the `todo!`-macro call with the following:

```rust
let date = chrono::DateTime::<chrono::Utc>::from(self.0);
f.write_fmt(format_args!("{}", date.to_rfc2822()))
```

And in our main loop, let's extract the metadata as we use it more than one place and extract the modified at-date:

```rust
let metadata = entry.metadata().context(format!(
    "Failed extracting metadata for {}. Perhaps you are missing permissions?",
    entry.path().display()
))?;
let modified_at = metadata
    .modified()
    .context("Could not get date modified for the entry")?;
```

Change the size entry to use that metadata:

```rust
let size = FormatSize(metadata);
```

And finally change the printing to include the modified date if the flag is set, or an empty string otherwise:

```rust
let formatted_date = if options.modified {
    let modified = FormatModifiedAt(modified_at);
    format!("{:>25}\t", modified)
} else {
    "".into()
};
println!("{}\t{}{}", size, formatted_date, formatted_entry);
```

Running this will give us output on the form of:

```shell
> cargo run -- --max-depth 3 --headers --modified # stripping some output
     Size       Modified at                        Name
    319 B       Tue, 28 Nov 2023 19:04:04 +0000 Cargo.toml
```

Yuck, that ` +0000` looks ugly, and it's always zero because of UTC, so let's strip it away and format the date in blue!

```rust
f.write_fmt(format_args!(
    "{}",
    date.to_rfc2822()
        // SAFETY: We know it is in UTC so the stripping always works, probably 🤠
        .strip_suffix(" +0000")
        .unwrap()
        .blue()
))
```

Et voilà! The same output line is now:

```shell
    319 B       Tue, 28 Nov 2023 19:04:04       Cargo.toml
```

Beautiful, even though the code is not.

</details>

---

### 🏆 Conclusion

Congratulations! Now you've created a terminal file explorer on your own!
It has several features, including showing files, folders, coloring things, dates, sizes, supports multiple levels, and more! You can keep working on it if you want to, but this workshop is probably a bit too long already.

Below is a list of bonus exercises or features you might want to add at some point.

Hopefully you've learned something about Rust, its ecosystem, and the standard library 😀 Enough to go on with this file explorer if you want to.

#### ⬇️Install

All throughout this workshop we've been running our code using `cargo run` and passing arguments to our program using `-- --header --path . --hidden` and so on. This works fine while testing, but we've only been building a debug build, which has debug information and not a lot of optimizations are turned on.

To build a release binary we can use:

```shell
> cargo build --release
```

We can then find our binary in: `./target/release/explorer`.

If you want to use this program yourself, it can easily be done with `cargo install --path .`.
It places the resulting binary in `$HOME/.cargo/bin`, or whatever your cargo install location is.
As long as that folder is in your PATH, you can run it using `explore`! Just pass arguments to it as you would any program.

---

#### Bonus exercises

Although our explorer can do some stuff, there's still plenty of things it cannot do. Not all of them make sense to do, but we'll list them anyway.

- Use `stdout.is_terminal()` to check if your program is being used in a pipeline to only output filenames/directories. E.g. when running `explore -p /usr/bin | grep -I ls` it should not use colors and should not output formatted output, only entries.
- Add permission outputs. You know, is the file `.rw-r--r--`, the directory `drwxr-xr-x`, `0755`, and so on.
- Support owner. Is the file owned by the current user, root, etc.
- Support non-utf-8 filenames by displaying escape sequences. Currently we used `to_string_lossy` on filenames, which as the name implies is lossy.
- Support ignore-files to ignore files that are `.gitignore`d, `.dockerignore`d, and so on. [ignore](https://docs.rs/ignore/latest/ignore/) could probably help with that.
- Add support for regex/globs matches to include/exclude things, e.g. `explore --exclude "*.java"`
- Use the amazing [ratatui](https://github.com/ratatui-org/ratatui) to create an interactive file-explorer akin to the awesome [skim](https://github.com/lotabout/skim) to preview files on the right-hand side.
- Use [bat](https://github.com/sharkdp/bat) to preview the contents of files perhaps.
- Or something entirely different! Your imagination is the limit

Thanks! 👋
