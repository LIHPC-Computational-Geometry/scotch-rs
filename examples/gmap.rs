//! Rewrite of Scotch's gmap executable.
//!
//! Original sources are located at `src/scotch/gmap.c` in Scotch's repository.

use scotch::Architecture;
use scotch::Graph;
use scotch::Strategy;
use std::convert::TryFrom as _;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "usage: {} nparts input_source_graph_file output_mapping_file",
            args[0]
        );
        process::exit(1);
    }

    let partnbr: usize = args[1].parse()?;
    scotch::Num::try_from(partnbr).expect("nparts is outside of Scotch's integer range");

    // TODO clock init

    let architecture = Architecture::complete(partnbr as scotch::Num);

    let mut graph = Graph::from_file(&args[2], -1)?;
    let (vertnbr, _) = graph.size();
    let mut parttab: Vec<scotch::Num> = vec![0; vertnbr as usize];

    // TODO clock reset

    graph
        .mapping(&architecture, &mut parttab)
        .compute(&mut Strategy::new())?
        .write_to_file(&args[3])?;

    // TODO clock reset

    Ok(())
}
