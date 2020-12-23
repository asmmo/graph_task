use std::io::Read;
use std::path::Path;
use crate::ThreadSafeGenError;

#[derive(Debug)]
pub struct Node{
    pub node_id: String,
    pub t: u8,
    pub x: f64,
    pub y: f64,
    pub ball_id: String,
}

pub fn parse_csv<T: AsRef<Path>>(location: T) -> Result<Vec<Node>, ThreadSafeGenError>{
    let mut csv_file = std::fs::File::open(location)?;
    let mut csv_file_str = String::new();
    csv_file.read_to_string(&mut csv_file_str)?;
    let iter = csv_file_str.split_whitespace().skip(1);
    let mut nodes_vec = Vec::with_capacity(iter.clone().count());
    let mut temp_node;
    for line in iter{
        let fields = line.split_terminator(',').collect::<Vec<&str>>();
        temp_node = Node{
            node_id: fields[0].to_string(),
            t: fields[1].to_string().parse()?,
            x: fields[2].to_string().parse()?,
            y: fields[3].to_string().parse()?,
            ball_id: fields[4].to_string(),
        };
        nodes_vec.push(temp_node);
    }
    Ok(nodes_vec)
}