#pokerlookup-rs

A library which provides the means to generate a large lookup table (about 124MB), which can be dumped to and loaded from a file. 5, 6 and 7 card poker hands
can be evaluated using this table with impressive throughput.

The crate is called `pokerlookup` and you can depend on it via cargo:

```ini
[dependencies.pokerlookup]
git = "https://github.com/th4t/pokerlookup-rs.git"
```

## Setup
Before running tests, execute the following command from the project root directory, to generate a HandRank.dat file.
This will take a while. Not too long.

```
# or this, to get the results way faster:
$ cargo build --release
$ ./target/release/generate

```
Arguably, you could also try *cargo run*, but this will be painfully slow due to the debug build mode. Only if you want to wait a good while.

Afterwards the tests should run through successfully, evaluating all possible 5-card poker hands
```
$ cargo test
```

## Checksum

The md5 sum of the generated *HandRanks.dat* file should be *5003cf3e6d5c9b8ee77094e168bfe73f*.

## License
As the original cpp code for this was licensed under GPL, this crate carries a similar license.

## Related Crates
* [cards-rs](https://github.com/th4t/cards-rs)
* [holdem-rs](https://github.com/th4t/holdem-rs)
* [pokereval-rs](https://github.com/th4t/pokereval-rs)
* **pokerlookup-rs**
