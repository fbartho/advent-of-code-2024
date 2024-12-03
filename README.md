# AdventOfCode 2024

[AdventOfCode.com](https://AdventOfCode.com) is a fun little yearly coding challenge. The problems start generally approachable, but were designed by people with twisty minds, and naive solutions can run into challenges. People can use this to learn a new programming language, or to flex their skills on their daily language. It's a fun way to form a community among developers.

Stack:
- Rust

## Layout

- [`assignments/`](./assignments/) - the Puzzle Assignments from AdventOfCode.com
- [`input/`](./input/) - the data-file associated with each puzzle
- [`src/puzzle/`](./src/puzzle/) - the source code for any given puzzle
-	[`src/prelude.rs`](./src/prelude.rs) - automatically import default types/crates into every puzzle
- [`src/main.rs`](./src/main.rs) [`src/lib.rs`](./src/lib.rs) - CLI Command & Subcommands
- [`src/puzzle.rs`](./src/puzzle.rs) - Type every daily puzzle conforms to

## Usage

- `cargo run -- -d1 -p1 download`
- `cargo run -- -d1 -p1 puzzle`
- `RUST_LOG=info RUST_BACKTRACE=1 cargo watch -x "run -- -d1 -p1 puzzle"`
- `cargo run -- -d1 -p1 puzzle --submit`

### Authentication

- Go to AdventOfCode, login to your account.
- Open DevTools, find the Storage->Cookies
- Find the `session` cookie, and copy its value
- `pbpaste > ~/.adventofcode.session`
