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
This will take a while. Quite a while indeed. Be patient.

```
$ cargo run
```

Afterwards the tests should run through successfully, evaluating all possible 5-card poker hands
```
$ cargo test
```

## License
As the original cpp code for this was licensed under GPL, this crate carries a similar license.

## Related Crates
* [cards-rs](https://github.com/th4t/cards-rs)
* [holdem-rs](https://github.com/th4t/holdem-rs)
* [pokereval-rs](https://github.com/th4t/pokereval-rs)
* pokerlookup-rs
