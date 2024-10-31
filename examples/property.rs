use std::collections::HashMap;

use ragout::components::property::{PMap, Properties};

fn main() {
    let mut map = Properties::new();
    map.assign("this is a key", "this value");

    println!("{:?}", map);
}
