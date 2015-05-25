extern crate pokerlookup;

use std::path::Path;
use pokerlookup::{LookupTable};

/// Generate the HandRank.dat file, which is later used in tests.
/// Assumes, that this is called via cargo run from the project root.
#[allow(dead_code)]
fn main() {
    let out_path = Path::new("gen/HandRanks.dat");
    let mut table = LookupTable::generate();
    table.save(out_path);
}
