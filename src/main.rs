use graph_task::process_csv::*;
use graph_task::filter::*;
use graph_task::visualize::plot_images;

fn main() {

    let nodes = parse_csv("flying_balls.csv").unwrap();
    let less_than_1000_table = filter_by_distances_less_than(&nodes, 1000.0);
    store_result_pair_as_csv(&less_than_1000_table, "les_1000_distances_table.csv").unwrap();
    let less_than_100_table = filter_by_distances_less_than(&nodes, 100.0);
    plot_images(&nodes, &less_than_100_table, "of_x_less_than_100___").unwrap();
    let mut portion_of_less_than_100_table= Vec::with_capacity(less_than_100_table.len()/50usize);
    let mut i = 0;
    while i < less_than_100_table.len(){
        portion_of_less_than_100_table.push(less_than_100_table[i].clone());
        i += 50;
    }
    plot_images(&nodes, &portion_of_less_than_100_table, "portion_of_x_less_than_100___").unwrap();

}
