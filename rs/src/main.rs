use nanami_algo_ds::algo::kruskal::Kruskal;
use nanami_algo_ds::ds::graph::WeightedEdge;
use nanami_algo_ds::ds::graph::WeightedGraph;
use proconio::input;
use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Args {
    #[clap(subcommand)]
    subcommands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Algo {
        #[clap(subcommand)]
        algoop: AlgoOp,
    },
}

#[derive(Debug, Subcommand)]
enum AlgoOp {
    Kruskal,
}

fn main() {
    let args = Args::parse();
    match args.subcommands {
        SubCommands::Algo { algoop } => {
            match algoop {
                AlgoOp::Kruskal => {
                    run_kruskal();
                },
            }
        },
    }
}

fn run_kruskal() {
    input! {
        n: i32,
        edge_n: i32,
        raw_edges: [(i32, i32, i32); edge_n],
    }

    let mut edges = Vec::<WeightedEdge>::new();
    for (a, b, cost) in raw_edges {
        edges.push(WeightedEdge::new(a, b, cost));
    } 

    let graph = WeightedGraph::new(n, edges.clone());

    let mut kruskal = Kruskal::new(graph);
    let result = kruskal.derive_mst().get_cost();

    println!("min cost => {}", result);

    let critical_edges = kruskal.derive_critical_edges();
    println!("critical edges => {:?}", critical_edges);

    let pseudo_critical_edges = kruskal.derive_pseudo_critical_edges(&critical_edges);
    println!("pseudo critical edges => {:?}", pseudo_critical_edges);

}
