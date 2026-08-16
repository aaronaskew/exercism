/// Solve using Heirholzer's algorithm
fn solve(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    // Empty check
    if input.is_empty() {
        return Some(vec![]);
    }

    let max_val = input.iter().map(|&(a, b)| a.max(b)).max().unwrap_or(0);
    let n = max_val as usize + 1;

    // Adjacency list (target_vertex, edge_id)
    let mut adj_list: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    let mut degree = vec![0usize; n];

    for (edge_id, &(a, b)) in input.iter().enumerate() {
        let a = a as usize;
        let b = b as usize;
        adj_list[a].push((b, edge_id));
        adj_list[b].push((a, edge_id));
        degree[a] += 1;
        degree[b] += 1;
    }

    // Check for all even degrees of nodes, required for Eulerian circuit
    if degree.iter().any(|deg| !deg.is_multiple_of(2)) {
        return None;
    }

    // Find first non-zero degree vertex
    let mut start_v = 0;
    while degree[start_v] == 0 && start_v < n {
        start_v += 1;
    }

    // Hierholzer's algorithm
    let mut stack = vec![start_v];
    let mut answer = vec![];

    // Track which dominoes have been used to handle undirected multigraphs safely
    let mut edge_used = vec![false; input.len()];

    // Optimization to avoid rescanning used edges (makes traversal O(E))
    let mut edge_idx = vec![0; n];

    while let Some(&curr) = stack.last() {
        let mut found_edge = false;

        // look for edge
        while edge_idx[curr] < adj_list[curr].len() {
            let (next, edge_id) = adj_list[curr][edge_idx[curr]];
            edge_idx[curr] += 1;

            if !edge_used[edge_id] {
                edge_used[edge_id] = true;
                stack.push(next);
                found_edge = true;
                break;
            }
        }

        if !found_edge {
            answer.push(stack.pop().expect("stack should not be empty"));
        }
    }

    // check graph connectivity
    if answer.len() != input.len() + 1 {
        return None;
    }

    let mut chain = vec![];

    for i in 0..answer.len() - 1 {
        chain.push((answer[i] as u8, answer[i + 1] as u8));
    }

    Some(chain)
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    solve(input)
}
