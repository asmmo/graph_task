use graph_task::process_csv::*;
use graph_task::less_than_1000_distances_table::*;
use graph_task::visualize_less_than_100::plot_images;

fn main() {
    let nodes = parse_csv("flying_balls.csv").unwrap();
    let less_than_1000_table = compute_distances_less_than(&nodes, 1000.0);
    store_distances_table(&less_than_1000_table, "les_1000_distances_table.csv").unwrap();
    let less_than_100_table = compute_distances_less_than(&nodes, 100.0);
    plot_images(&nodes, &less_than_100_table).unwrap();
}
