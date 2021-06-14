use scotch::Graph;
use scotch::Strategy;
use std::convert::TryFrom as _;
use std::env;
use std::ffi;
use std::fs;
use std::io::Write as _;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO SCOTCH_errorProg

    let args: Vec<_> = env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        eprintln!(
            "usage: {} nparts input_source_graph_file output_mapping_file [strategy]",
            args[0]
        );
        process::exit(1);
    }

    let partnbr: usize = args[1].parse()?;
    scotch::Num::try_from(partnbr).expect("nparts is outside of Scotch's integer range");

    let mut strategy = Strategy::new();
    if args.len() == 5 {
        let strategy_string = ffi::CString::new(args[4].clone())?;
        strategy.graph_part_ovl(strategy_string)?;
    }

    let mut graph = Graph::from_file(&args[2], -1)?;
    let vertnbr = graph.data().vertnbr() as usize;
    let baseval = graph.data().baseval as usize;
    let mut parttax = vec![0; vertnbr];
    let mut loadtab: Vec<scotch::Num> = vec![0; partnbr];
    let mut flagtab: Vec<usize> = vec![!0; partnbr];

    graph.part_ovl(partnbr as scotch::Num, &mut strategy, &mut parttax)?;

    let graph_data = graph.data();

    for vertnum in 0..vertnbr {
        let veloval = graph_data.velotab.get(vertnum).unwrap_or(&1);
        let partval = parttax[vertnum];
        if partval >= 0 {
            // vertex belongs to one part only
            loadtab[partval as usize] += veloval;
        } else {
            for edgenum in graph_data.verttab[vertnum]..graph_data.vendtab[vertnum] {
                let vertend = graph_data.edgetab[edgenum as usize] as usize;
                let partend = parttax[vertend - baseval]; // vertend is based.
                if partend < 0 {
                    // Neighbor has no identifiable part.
                    continue;
                }
                if flagtab[partend as usize] == vertnum {
                    // Neighbor part is already accounted for.
                    continue;
                }
                loadtab[partend as usize] += veloval;
                flagtab[partend as usize] = vertnum;
            }
        }
    }

    let mut loadsum = 0;
    let mut loadmax = 0;
    let mut loadmin = scotch::Num::MAX;

    for (partnum, &partload) in loadtab.iter().enumerate() {
        loadsum += partload;
        if partload > loadmax {
            loadmax = partload;
        }
        if partload < loadmin {
            loadmin = partload;
        }
        println!("M\tCompload[{:02}]\t{}", partnum, partload);
    }

    let loadavg = loadsum as f64 / partnbr as f64;
    println!("M\tCompLoadAvg\t{}", loadavg);
    println!("M\tCompLoadMax/Avg\t{}", loadmax as f64 / loadavg);

    let mut file = fs::File::create(&args[3])?;
    writeln!(file, "{}", vertnbr)?;
    for vertnum in 0..vertnbr {
        let label = graph_data
            .vlbltab
            .get(vertnum)
            .cloned()
            .unwrap_or(vertnum as scotch::Num);
        writeln!(file, "{}\t{}", label, parttax[vertnum])?;
    }

    Ok(())
}
