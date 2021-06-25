use scotch::Architecture;
use scotch::Graph;
use scotch::Strategy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xadj = [0, 2, 5, 8, 11, 13, 16, 20, 24, 28, 31, 33, 36, 39, 42, 44];
    let adjncy = [
        1, 5, 0, 2, 6, 1, 3, 7, 2, 4, 8, 3, 9, 0, 6, 10, 1, 5, 7, 11, 2, 6, 8, 12, 3, 7, 9, 13, 4,
        8, 14, 5, 11, 6, 10, 12, 7, 11, 13, 8, 12, 14, 9, 13,
    ];
    let partnbr = 2;
    let mut parttab = vec![0x00; 15];

    let architecture = Architecture::complete(partnbr);

    let mut graph = Graph::build(&scotch::graph::Data::new(
        0,
        &xadj,
        &[],
        &[],
        &[],
        &adjncy,
        &[],
    ))
    .unwrap();
    graph.check().unwrap();

    // TODO clock reset

    graph
        .mapping(&architecture, &mut parttab)
        .compute(&mut Strategy::new())?
        .write_to_file("out.map")?;

    // TODO clock reset

    Ok(())
}
