use graph_coloring2::*;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let graph = Graph::from("./data/gc_20_1");
    graph.print_graph();
    println!();
   
    // greedy coloring 
    /*
    let (color_map, num_colors) = graph.greedy_coloring();
    for (vertex, color) in &color_map {
        println!("Vertex {}: Color {}", vertex, color);
    }
    println!("Number of colors with greedy: {}", num_colors);
    */

    // rlf coloring
    /*
    let (color_map, num_colors) = graph.rlf_coloring();

    for (vertex, color) in &color_map {
        println!("Vertex {}: Color {}", vertex, color);
    }
    println!("Number of colors with rlf: {}", num_colors)
    */
    
    // dsatur
    println!("Vertex - Color");
    let (color_map, num_colors) = graph.dsatur_coloring();
    for (vertex, color) in color_map {
        println!("Vertex {}: Color {}", vertex, color);
    }
    println!("Number of colors: {}", num_colors);
    let end_time = SystemTime::now();
    let elapsed_time = end_time.duration_since(start_time).unwrap();
    println!("Elapsed time: {} seconds and {} nanoseconds", elapsed_time.as_secs(), elapsed_time.subsec_nanos());
}
