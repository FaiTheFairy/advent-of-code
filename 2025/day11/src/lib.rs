use std::collections::{HashMap, HashSet};

pub type Graph = HashMap<String, Vec<String>>;

pub fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (lhs, rhs) = line.split_once(": ").unwrap_or_else(|| {
            panic!("Invalid line (expected `in: out1 out2`): {line}");
        });

        let mut outputs = vec![];
        for output in rhs.split_whitespace() {
            outputs.push(output.to_string());
        }
        graph.insert(lhs.to_string(), outputs);
    }
    graph
}

pub fn count_paths(graph: &Graph, start: &str, goal: &str) -> u64 {
    let mut cached_path_counts = HashMap::new();
    let mut nodes_on_current_stack = HashSet::new();

    count_paths_from_node(
        start,
        goal,
        graph,
        &mut cached_path_counts,
        &mut nodes_on_current_stack,
    )
}

/// Helper: returns the number of complete paths from `current` to `goal`.
///
/// - If `current == goal`, that is exactly one completed path
/// - Otherwise, sum the counts of all outgoing neighbors
/// - Use momization (`cached_path_counts`) to avoid recomputing.
/// - Use `nodes_on_current_stack` to detect cycles during DFS
fn count_paths_from_node(
    current: &str,
    goal: &str,
    graph: &Graph,
    cached_path_counts: &mut HashMap<String, u64>,
    nodes_on_current_stack: &mut HashSet<String>,
) -> u64 {
    if current == goal {
        return 1;
    }

    // memoization: if we've computed it, reuse it
    if let Some(&already_computed) = cached_path_counts.get(current) {
        return already_computed;
    }

    // cycle detection: only this checks cycles
    if nodes_on_current_stack.contains(current) {
        panic!("Cycle detected involving `{current}`");
    }
    nodes_on_current_stack.insert(current.to_string());

    let mut total_paths_from_current: u64 = 0;

    if let Some(outgoing_neighbors) = graph.get(current) {
        for next_node in outgoing_neighbors {
            let paths_via_next = count_paths_from_node(
                next_node,
                goal,
                graph,
                cached_path_counts,
                nodes_on_current_stack,
            );
            total_paths_from_current = total_paths_from_current.saturating_add(paths_via_next);
        }
    }

    nodes_on_current_stack.remove(current);
    cached_path_counts.insert(current.to_string(), total_paths_from_current);

    total_paths_from_current
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    fn expected_example_graph() -> Graph {
        let mut g: Graph = HashMap::new();
        g.insert(
            "aaa".to_string(),
            vec!["you".to_string(), "hhh".to_string()],
        );
        g.insert(
            "you".to_string(),
            vec!["bbb".to_string(), "ccc".to_string()],
        );
        g.insert(
            "bbb".to_string(),
            vec!["ddd".to_string(), "eee".to_string()],
        );
        g.insert(
            "ccc".to_string(),
            vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()],
        );
        g.insert("ddd".to_string(), vec!["ggg".to_string()]);
        g.insert("eee".to_string(), vec!["out".to_string()]);
        g.insert("fff".to_string(), vec!["out".to_string()]);
        g.insert("ggg".to_string(), vec!["out".to_string()]);
        g.insert(
            "hhh".to_string(),
            vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()],
        );
        g.insert("iii".to_string(), vec!["out".to_string()]);
        g
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE);
        let expected = expected_example_graph();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_paths_example() {
        let graph = parse_input(EXAMPLE);
        let result = count_paths(&graph, "you", "out");
        let expected: u64 = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_paths_from_node_counts_subproblem() {
        let graph = parse_input(EXAMPLE);

        let mut cached_path_counts: HashMap<String, u64> = HashMap::new();
        let mut nodes_on_current_stack: HashSet<String> = HashSet::new();

        // From "bbb" to "out" there are 2 paths:
        // bbb -> eee -> out
        // bbb -> ddd -> ggg -> out
        let result = count_paths_from_node(
            "bbb",
            "out",
            &graph,
            &mut cached_path_counts,
            &mut nodes_on_current_stack,
        );

        let expected: u64 = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_paths_from_node_memoizes_result() {
        let graph = parse_input(EXAMPLE);

        let mut cached_path_counts: HashMap<String, u64> = HashMap::new();
        let mut nodes_on_current_stack: HashSet<String> = HashSet::new();

        let result = count_paths_from_node(
            "bbb",
            "out",
            &graph,
            &mut cached_path_counts,
            &mut nodes_on_current_stack,
        );

        assert_eq!(result, 2);
        // After calling, "bbb" should be memoized.
        assert_eq!(cached_path_counts.get("bbb").copied(), Some(2));
        // The recursion stack should be empty again after completion.
        assert!(nodes_on_current_stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Cycle detected")]
    fn test_count_paths_detects_cycle() {
        // a -> b -> a (cycle), and also a -> out
        let mut graph: Graph = HashMap::new();
        graph.insert("a".to_string(), vec!["b".to_string(), "out".to_string()]);
        graph.insert("b".to_string(), vec!["a".to_string()]);
        graph.insert("out".to_string(), vec![]);

        // Starting at a, exploring a->b->a triggers the cycle panic.
        let _ = count_paths(&graph, "a", "out");
    }

    #[test]
    fn test_count_paths_start_is_goal() {
        let graph = parse_input(EXAMPLE);
        let result = count_paths(&graph, "out", "out");
        let expected: u64 = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_paths_unreachable_goal() {
        // start exists but cannot reach out
        let mut graph: Graph = HashMap::new();
        graph.insert("you".to_string(), vec!["a".to_string()]);
        graph.insert("a".to_string(), vec![]); // dead end
        graph.insert("out".to_string(), vec![]);

        let result = count_paths(&graph, "you", "out");
        let expected: u64 = 0;
        assert_eq!(result, expected);
    }
}
