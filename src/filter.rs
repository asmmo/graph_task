use crate::process_csv::Node;

/// This `struct` will be used to store each pair of `Node`s and `x` distance between them, if the
/// `x` distance between them is < 1000.
#[derive(Clone)]
pub struct ResultPair{
    pub pair_node_1: String,
    pub pair_node_2: String,
    pub dx: f64,
}


/// This function accepts a reference to `&[Node]` (which can be a ref to `Vec<Node>`) wanted to be
/// filtered and then returns a new `Vec<ResultPair>` containing the wanted nodes.
pub fn filter_by_distances_less_than(nodes: &[Node], num: f64) -> Vec<ResultPair>{

    let mut result = Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2);
    let mut counter = 1;
    let mut temp_result_node;
    for node in nodes{
        for i in counter..nodes.len(){
            temp_result_node = ResultPair{
                pair_node_1: node.node_id.clone(),
                pair_node_2: nodes[i as usize].node_id.clone(),
                dx: (node.x - nodes[i as usize].x).abs()
            };
            if temp_result_node.dx < num {
                result.push(temp_result_node);
            }
        }
        counter += 1;
    }

    result.shrink_to_fit();
    result
}