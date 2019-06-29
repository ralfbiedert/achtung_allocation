use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let mut f = File::create("src/config.rs").unwrap();
    let mut b = BufWriter::new(f);
    
    let entries = option_env!("ACHTUNG_ALLOCATION_MAX_EVENTS").unwrap_or("1_000_000");
    
    
    writeln!(b, "/// Maximal number of events that can be recorded.");
    writeln!(b, "///");
    writeln!(b, "/// A number of calls might reset this limit clear up used events. You can configure this limit");
    writeln!(b, "/// by setting `ACHTUNG_ALLOCATION_MAX_EVENTS` before you *build* your crate!");
    writeln!(b, "pub const MAX_EVENTS: usize = {};", entries);
}