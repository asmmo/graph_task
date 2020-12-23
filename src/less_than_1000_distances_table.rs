use crate::process_csv::Node;
use std::path::Path;
use std::io::Write;
use crate::ThreadSafeGenError;

#[derive(Debug)]
pub struct ResultPair{
    pub pair_node_1: String,
    pub pair_node_2: String,
    pub dx: f64,
}

pub fn compute_distances_less_than(nodes: &[Node], num: f64) -> Vec<ResultPair>{

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

pub fn store_distances_table(table: &[ResultPair], location: impl AsRef<Path>) -> Result<(), ThreadSafeGenError> {

    let mut csv_file = std::fs::File::create(location)?;
    let mut line ;
    line = "pair_node_1,pair_node_2,d(x)\n".to_string();
    csv_file.write_all(line.as_bytes())?;
    for pair in table{
        line = format!("{},{},{}\n", pair.pair_node_1, pair.pair_node_2, pair.dx);
        csv_file.write_all(line.as_bytes())?;

    }
    csv_file.flush()?;
    Ok(())
}
