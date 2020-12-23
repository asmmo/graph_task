use std::io::{Read, Write};
use std::path::Path;
use crate::ThreadSafeGenError;

/// `Node` receives a line of the `csv` file lines.
pub struct Node{
    pub node_id: String,
    pub t: u8,
    pub x: f64,
    pub y: f64,
    pub ball_id: String,
}


/// This function will parse the `csv` file and return an `Ok(Vec)` of `Node`s if everything is fine
/// and returns `Err` describing the error has happened otherwise.
/// The vector will contains all of the lines in the `csv` file such that every line represents one `Node`.
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




/// This function takes a `&[ResultPair]` and then creates a `csv` file containing the `ResultPair`
/// in the slice
pub fn store_result_pair_as_csv(table: &[super::filter::ResultPair], location: impl AsRef<Path>)
                                -> Result<(), ThreadSafeGenError> {

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
