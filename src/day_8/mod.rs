mod node;
mod graph;
mod parser;

pub fn solve_day_8_part_1() -> usize{
    let input_file = "resources\\input_day_8.txt";
    let file_content = parser::read_file(input_file)
                                .expect("Failed to read test file");
    let cycle_instructions = parser::parse_cycle_instruction(&file_content)
                                        .expect("Failed to parse cycle instructions");
    let nodes = parser::parse_nodes(&file_content)
                                .expect("Failed to parse nodes");
    let mut graph = graph::Graph::new();
    for node in nodes {
        graph.add_node(node);
    }
    let traversal_steps = graph.traverse("AAA", "ZZZ", &cycle_instructions, 10)
                                            .expect("Traversal failed");
    traversal_steps.1
}