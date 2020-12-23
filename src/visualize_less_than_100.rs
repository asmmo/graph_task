use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut, };
use crate::ThreadSafeGenError;
use crate::less_than_1000_distances_table::ResultPair;
use crate::process_csv::Node;

pub fn plot_images(all_nodes: &[Node], filtered_nodes_with_edges: &[ResultPair]) -> Result<(), ThreadSafeGenError>{

    // scaling
    let biggest_t = 47.0;
    let mut biggest_x = 0.0;
    let mut biggest_y = 0.0;
    for res_pair in filtered_nodes_with_edges{
        let p1 = &all_nodes[res_pair.pair_node_1.parse::<usize>()?];
        let p2 = &all_nodes[res_pair.pair_node_2.parse::<usize>()?];
        let (x1, y1, x2, y2) =
            (
                p1.x as f64, p1.y as f64,
                p2.x as f64, p2.y as f64,
            );


        if x1 > biggest_x{
            biggest_x = x1;
        }
        if x2 > biggest_x{
            biggest_x = x2;
        }
        if y1 > biggest_y{
            biggest_y = y1;
        }
        if y2 > biggest_y{
            biggest_y = y2;
        }
    }

    let mut image_xy = RgbImage::new(1280, 1024);
    let x_factor = 1280f32 / biggest_x as f32 ;
    let y_factor = 1024f32 / biggest_y as f32 ;

    for res_pair in filtered_nodes_with_edges{
        let p1 = &all_nodes[res_pair.pair_node_1.parse::<usize>()?];
        let p2 = &all_nodes[res_pair.pair_node_2.parse::<usize>()?];

        plot_node(&mut image_xy, p1.x as f32  * x_factor, p1.y as f32  * y_factor);
        plot_node(&mut image_xy, p2.x as f32  * x_factor, p2.y as f32  * y_factor);

        plot_edge(
            &mut image_xy,
            p1.x as f32  * x_factor,
            p1.y as f32  * y_factor,
            p2.x as f32  * x_factor,
            p2.y as f32  * y_factor,
        );
    }
    image_xy.save("xy.png")?;


    let mut image_tx = RgbImage::new(1280, 1024);
    let t_factor = 1280f32 / biggest_t as f32 ;
    let x_factor = 1024f32 / biggest_x as f32 ;

    for res_pair in filtered_nodes_with_edges{
        let p1 = &all_nodes[res_pair.pair_node_1.parse::<usize>()?];
        let p2 = &all_nodes[res_pair.pair_node_2.parse::<usize>()?];

        plot_node(&mut image_tx, p1.t  as f32 * t_factor, p1.x as f32  * x_factor);
        plot_node(&mut image_tx, p2.t  as f32 * t_factor, p2.x as f32  * x_factor);

        plot_edge(
            &mut image_tx,
            p1.t as f32 * t_factor,
            p1.x as f32  * x_factor,
            p2.t as f32 * t_factor,
            p2.x as f32 * x_factor,
        );
    }
    image_tx.save("tx.png")?;
    Ok(())
}

fn plot_node(image: &mut RgbImage, x: f32, y: f32){
    draw_filled_circle_mut(
        image,
        (x as i32, y as i32),
        2,
        Rgb([0u8, 0u8, 255u8]),
    );
}

fn plot_edge(image: &mut RgbImage, x1: f32, y1: f32, x2: f32, y2: f32){
    draw_line_segment_mut(
        image,
        (x1, y1),
        (x2, y2),
        Rgb([255u8, 255u8, 255u8])
    );

}