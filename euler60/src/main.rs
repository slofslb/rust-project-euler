use petgraph::algo;
use petgraph::Graph;
use primes::PrimeSet;

fn main() {
    // 参考这篇文章，学会了petgraph的使用
    // https://thobbs.cz/rust-play/petgraph_review.html
    let mut graph: Graph<&str, &str, petgraph::Directed> = Graph::new();
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");
    graph.add_edge(a, b, "");
    graph.add_edge(a, c, "");
    graph.add_edge(b, d, "");
    graph.add_edge(b, e, "");

    //let mut dag = tree.clone();
    let f = graph.add_node("f");
    graph.add_edge(d, f, "");
    graph.add_edge(e, f, "");

    //let mut with_scc = dag.clone();
    graph.add_edge(f, b, "");

    println!(
        "Does 'with_scc' have strongly connected components? {}",
        algo::is_cyclic_directed(&graph)
    );
    println!(
        "Does 'dag' have strongly connected components? {}",
        algo::is_cyclic_directed(&graph)
    );

    // let  _condensed_graph = algo::condensation(with_scc, true);
    // let condensed_graph = _condensed_graph.map(
    //     |_, w| format!("{:?}", w),
    //     |_, w| w.clone(),
    // );
    // println!("{:#?}", condensed_graph);

    // let _with_scc_and_edge_weights = with_scc.clone();
    // let with_scc_and_edge_weights = _with_scc_and_edge_weights.map(
    //     |_, w| w.clone(),
    //     |i, _| i.index(),
    // );
    // println!("{:#?}", with_scc_and_edge_weights);

    let sccs = algo::tarjan_scc(&graph);
    for scc in sccs {
        println!("----");
        if scc.len() == 4 {
            for node_id in scc {
                println!("{:?} {}", node_id, graph.node_weight(node_id).unwrap());
            }
        }
    }

    // g.extend_with_edges(&[
    //     (0, 1), (0, 2), (0, 3)
    // ]);
    // let tt = algo::kosaraju_scc(g);
    // println!("{:?}", tt);

    let max = 9000;
    let primes: Vec<Vec<u64>> = vec![vec![]];
    let p1 = prime_pairs(max, &primes);
    let p2 = prime_pairs(max, &p1);
    let p3 = prime_pairs(max, &p2);
    let p4 = prime_pairs(max, &p3);
    let p5 = prime_pairs(max, &p4);
    if !p5.is_empty() {
        println!("{:?}", p5[0]);
        println!("{:?}", p5[0].iter().sum::<u64>());
    }
}
// error: 129976621
// right: 26033 = [13, 5197, 5701, 6733, 8389]

fn prime_pairs(max: u64, list: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut pairs = vec![];

    for primes_list in list.iter() {
        let mut pset1 = PrimeSet::new();
        for j in pset1.iter() {
            if j > max {
                break;
            }
            if primes_list.is_empty() {
                pairs.push(vec![j]);
                continue;
            }
            if j <= *primes_list.last().unwrap() {
                continue;
            }
            if all_primes(primes_list, j) {
                let mut v = primes_list.to_vec();
                v.push(j);
                pairs.push(v);
            }
        }
    }
    pairs
}

fn all_primes(primes_list: &[u64], b: u64) -> bool {
    for &a in primes_list.iter() {
        if !is_concat_primes(a, b) {
            return false;
        }
    }
    true
}

fn is_concat_primes(a: u64, b: u64) -> bool {
    let c = format!("{}{}", a, b);
    let c = c.parse::<u64>().unwrap();
    if !primes::is_prime(c) {
        return false;
    }
    let d = format!("{}{}", b, a);
    let d = d.parse::<u64>().unwrap();
    primes::is_prime(d)
}
