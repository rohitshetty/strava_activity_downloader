## 2024-05-12

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

# 2024-05-16

**Figuring out talking to Strava**

You need to get the `refresh_token` and `access_token` with scope `activities:read_all`.

Strava lets you create apps on their platform. This provides `client_secret`, `client_id`, `access_id` (with default `read` scope). Open [^4] and read Part B on how to setup the strava app and get your `client_secret` and `client_id`. The default access_id is not enough for listing activities. We need to "Authenticate" using oAuth to get the new access_code with updated scope.
This is a two step process.

1. Making call to Strava to begin the oAuth dance to authenticate and get the `code`. We need the scope: `activity:read_permission`
2. Exchange the code with strava to get the new `access_code` along with `refresh_token` and other metadata.

Refer to [^4] Part C to know the details on executing 1 & 2 above.
If you face any issue with step 1, make sure to check Authorization callback domain in strava api section, and make sure it is localhost (or whatever you pass in the step 1 url).

Here is how it looks for me:

```
Call: http://www.strava.com/oauth/authorize?client_id=<client_id>&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=activity:read_permission

Returns: http://localhost/exchange_token?state=&code=<code>&scope=read,activity:read_all

Make: curl -X POST https://www.strava.com/oauth/token \
	-F client_id=<client_id> \
	-F client_secret=<client_secret> \
	-F code=<code> \
	-F grant_type=authorization_code

Returns: {"token_type":"Bearer","expires_at":1715829651,"expires_in":21600,"refresh_token":"xxxxxxxx","access_token":"xxxxxxxx","athlete":{"id":19297002,"username":"rohit_shetty","resource_state":2,"firstname":"Rohit","lastname":"Shetty","bio":"","city":"Bangalore","state":"KARNATAKA","country":null,"sex":"M","premium":false,"summit":false,"created_at":"2017-01-11T02:16:50Z","updated_at":"2024-04-09T17:23:02Z","badge_type_id":0,"weight":106.0,"profile_medium":"https://lh3.googleusercontent.com/a/ACg8ocI5kDDnbnUj2t01f_Id1PHwJdMAHoz6xa8wFQiABWE6VlCVcPFO=s96-c","profile":"https://lh3.googleusercontent.com/a/ACg8ocI5kDDnbnUj2t01f_Id1PHwJdMAHoz6xa8wFQiABWE6VlCVcPFO=s96-c","friend":null,"follower":null}}
```

**Making http request**
Using `reqwest` TBA

**Using confy to loaf configuration files**

- How to use confy - defining the structure TBA
- Creating default in location

**Rust Borrower and passing struct to function**
TBA

**Async Await using tokio**
TBA

[^1]: https://doc.rust-lang.org/book/ch19-06-macros.html
[^2]: https://doc.rust-lang.org/rust-by-example/attribute.html
[^3]: https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros
[^4]: https://developers.strava.com/docs/getting-started/
