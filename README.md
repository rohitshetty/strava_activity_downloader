# Strava Activity Downloader

## What does this solve?

I want to sync my Strava activities to my blog (Which is made with Hugo). I want a cli tool that can list the activities, download the latest/selected activity, and download the gpx file, and metadata.

Following the unix way of doing things, I wanted this to be a standalone CLI tool instead of being part of blog's scripts/ folder.

Pretty sure there are several different tools that does this job already - but I want to build one to learn rust. Rust has been on my radar for a while to learn - so why not jump headfirst into this?

## Building

`cargo build`

## Changelog

_2024-05-12_
Integrate `clap` crate for adding command line subcommand and other parsing options

## Learn log

### 2024-05-12

**Attributes**

`#` is called attributes in rust. This is when used in form of `#[]` is called `OuterAttribute` This is applied on whatever follows this. `#![]` This is called `InnerAttribute` and gets applied to the inner enclosed item. This can be used to do stuff like disabling lint, conditional compilations, setting crate metadata, Implementing traits automatically from other crates etc. Read [^2] for more details

**Derive**

That brings us to `#[derive]`. This is used to add traits implementation to your struct.
Example

```rust
use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "strava_activity_downloader")]
#[command(about = "View and download strava activity", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
.........
fn main() {
    let args = Cli::parse();
}
```

Here for the struct of `Cli` we are adding traits from the Parser so we can use `Cli::parse()` etc without implementing ourselves. This works using metaprogramming. Metaprogramming is where you write code using code. During compilation time, compiler runs the derive definition and generate rust code to add the required traits. Refer to the [^1] for details on this.

**Attribute-like Macros**

These are used to add more attributes. `derive` works with struct or enum, whereas this would work with functions and other items too.
Example

```rust
use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "strava_activity_downloader")]
#[command(about = "View and download strava activity", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
.........
fn main() {
    let args = Cli::parse();
}
```

Here `#[command(name="...")]` is Attribute-like-Macro. This would add more attributes to `Cli` struct, guessing to display name and details in cli. Refer to [^3] for details on this too.

[^1]: https://doc.rust-lang.org/book/ch19-06-macros.html
[^2]: https://doc.rust-lang.org/rust-by-example/attribute.html
[^3]: https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros
