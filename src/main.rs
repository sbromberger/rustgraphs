use rustgraphs::simplepathgraph::SimplePathGraph;
use rustgraphs::staticgraph::StaticGraph;
use rustgraphs::Graph;

fn main() {
    let g = SimplePathGraph::new(5).unwrap();
    println!("{}", g);
    // println!("{:?}", g.vertices());
    let ei = g.edges();
    let e: Vec<(usize, usize)> = ei.collect();
    println!("{:?}", e);
    println!("{:?}", g.out_neighbors(3));
    println!("{:?}", g.in_neighbors(3));
    println!("{:?}", g.in_neighbors(0));

    let g2 = SimplePathGraph::new(0);
    println!("{:?}", g2);

    let el: Vec<(u8, u8)> = vec![(0, 1), (1, 2), (2, 3), (3, 4), (2, 4), (4, 0)];
    let g2 = StaticGraph::from_edgelist(el).unwrap();
    println!("{}", g2);
    let ei2 = g2.edges();
    println!("----------------");
    println!("{:?}", ei2);
    let ee2: Vec<(u8, u8)> = ei2.collect();
    println!("{:?}", ee2)
}
