use std::collections::{HashMap, HashSet};
use std::fs;

pub struct Graph {
    edges: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn from(addr: &str) -> Self {
        let file = fs::read_to_string(addr).expect("failed to read file");
        let mut graph = Graph::new();
        
        for line in file.lines().skip(1) {
            let nums: Vec<i32> = line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("all numbers must be u32"))
                .collect();
            graph.add_edge(nums[0], nums[1]);
        }
        graph
    }

    pub fn print_graph(&self) {
        for (vertex, neighbors) in &self.edges {
            print!("{} => ", vertex);
            for neighbor in neighbors {
                print!("{} ", neighbor);
            }
            println!();
        }
    }

    pub fn greedy_coloring(&self) -> (HashMap<i32, i32>, i32) {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        let mut available_colors: HashSet<i32> = HashSet::new();

        for vertex in self.edges.keys() {
            color_map.insert(*vertex, -1);
        }

        for vertex in self.edges.keys() {
            available_colors.clear();
            for &neighbor in self.neighbors(*vertex).unwrap() {
                if let Some(&color) = color_map.get(&neighbor) {
                    if color != -1 {
                        available_colors.insert(color);
                    }
                }
            }

            let mut color = 0;
            while available_colors.contains(&color) {
                color += 1;
            }
            color_map.insert(*vertex, color);
        }
        let num_colors = color_map.values().max().unwrap_or(&-1) + 1;
        (color_map, num_colors)
    }

    pub fn rlf_coloring(&self) -> (HashMap<i32, i32>, i32) {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        let mut remaining_vertices: HashSet<i32> = self.edges.keys().cloned().collect();

        // Recursive function to color the vertices
        fn rlf_color(
            graph: &Graph,
            vertex: i32,
            color_map: &mut HashMap<i32, i32>,
            remaining_vertices: &mut HashSet<i32>,
        ) {
            // Find the color to assign to the vertex
            let mut available_colors: HashSet<i32> = HashSet::new();
            for &neighbor in graph.neighbors(vertex).unwrap() {
                if let Some(&color) = color_map.get(&neighbor) {
                    available_colors.insert(color);
                }
            }

            let mut color = 0;
            while available_colors.contains(&color) {
                color += 1;
            }

            // Assign the color to the vertex
            color_map.insert(vertex, color);
            remaining_vertices.remove(&vertex);

            // Recursively color the remaining uncolored neighbors
            for &neighbor in graph.neighbors(vertex).unwrap() {
                if remaining_vertices.contains(&neighbor) {
                    rlf_color(graph, neighbor, color_map, remaining_vertices);
                }
            }
        }

        // Iterate until all vertices are colored
        while !remaining_vertices.is_empty() {
            // Find the vertex with the maximum degree among remaining vertices
            let max_degree_vertex = remaining_vertices
                .iter()
                .max_by_key(|&vertex| self.neighbors(*vertex).unwrap().len())
                .unwrap()
                .clone();

            // Start coloring from the vertex with maximum degree
            rlf_color(self, max_degree_vertex, &mut color_map, &mut remaining_vertices);
        }
        let num_colors = color_map.values().max().unwrap_or(&-1) + 1;
        (color_map, num_colors)
    }

    // Function to perform graph coloring using DSatur algorithm
    pub fn dsatur_coloring(&self) -> (HashMap<i32, i32>, i32) {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        let mut remaining_vertices: HashSet<i32> = self.edges.keys().cloned().collect();

        while !remaining_vertices.is_empty() {
            // Find the vertex with the highest saturation degree
            let mut max_saturation_vertex = 0;
            let mut max_saturation_degree = -1;
            for &vertex in &remaining_vertices {
                let saturation_degree = self.saturation_degree(vertex, &color_map);
                if saturation_degree > max_saturation_degree {
                    max_saturation_degree = saturation_degree;
                    max_saturation_vertex = vertex;
                }
            }

            // Assign the smallest available color to the vertex
            let mut used_colors: HashSet<i32> = HashSet::new();
            if let Some(neighbors) = self.neighbors(max_saturation_vertex) {
                for &neighbor in neighbors {
                    if let Some(&neighbor_color) = color_map.get(&neighbor) {
                        used_colors.insert(neighbor_color);
                    }
                }
            }
            let mut color = 0;
            while used_colors.contains(&color) {
                color += 1;
            }
            color_map.insert(max_saturation_vertex, color);

            // Remove the vertex from the set of remaining vertices
            remaining_vertices.remove(&max_saturation_vertex);
        }
        let num_colors = color_map.values().max().unwrap_or(&-1) + 1;
        (color_map, num_colors)
    }

    fn saturation_degree(&self, vertex: i32, color_map: &HashMap<i32, i32>) -> i32 {
        let mut used_colors: HashSet<i32> = HashSet::new();
        if let Some(neighbors) = self.neighbors(vertex) {
            for &neighbor in neighbors {
                if let Some(&neighbor_color) = color_map.get(&neighbor) {
                    used_colors.insert(neighbor_color);
                }
            }
        }
        used_colors.len() as i32
    }


    fn add_edge(&mut self, from: i32, to: i32) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
        self.edges.entry(to).or_insert(Vec::new()).push(from);
    }

    fn neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.edges.get(&vertex)
    }
}
