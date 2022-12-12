use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub fn solve<T: Clone + Eq + Hash>(
    all_edges: &HashMap<T, Vec<(T, u64)>>,
    start: T,
    end: T,
) -> Option<(Vec<T>, u64)> {
    let mut solver = Solver::from_raw_map(all_edges);
    solver.solve(start, end)
}

pub fn solve_uniform_edges<T: Clone + Eq + Hash>(
    all_edges: &HashMap<T, Vec<T>>,
    start: T,
    end: T,
) -> Option<(Vec<T>, u64)> {
    solve(
        &all_edges
            .iter()
            .map(|(id, edges)| {
                (
                    id.clone(),
                    edges
                        .iter()
                        .map(|edge| (edge.clone(), 1))
                        .collect::<Vec<_>>(),
                )
            })
            .collect(),
        start,
        end,
    )
}

struct DijkstraNode<T: Clone + Hash> {
    identifier: T,
    edges: Vec<(T, u64)>,
    explored: RefCell<bool>,
}

impl<T: Clone + Eq + Hash> DijkstraNode<T> {
    pub fn new(identifier: T, edges: Vec<(T, u64)>) -> Self {
        DijkstraNode {
            identifier,
            edges,
            explored: RefCell::new(false),
        }
    }
}

impl<T: Clone + Eq + Hash> Hash for DijkstraNode<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

struct DijkstraRoute<'a, T: Clone + Eq + Hash> {
    length: u64,
    route: Vec<T>,
    current_node: &'a DijkstraNode<T>,
}

impl<'a, T: Clone + Eq + Hash> DijkstraRoute<'a, T> {
    fn new(starting_node: &'a DijkstraNode<T>) -> Self {
        DijkstraRoute {
            length: 0,
            route: vec![starting_node.identifier.clone()],
            current_node: starting_node,
        }
    }

    fn with_extra_edge(&self, edge_length: u64, edge_end: &'a DijkstraNode<T>) -> Self {
        let mut new_route = self.route.clone();
        new_route.push(edge_end.identifier.clone());
        DijkstraRoute {
            length: self.length + edge_length,
            route: new_route,
            current_node: edge_end,
        }
    }
}

impl<T: Clone + Eq + Hash> PartialEq for DijkstraRoute<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

impl<T: Clone + Eq + Hash> Eq for DijkstraRoute<'_, T> {}

impl<T: Clone + Eq + Hash> PartialOrd for DijkstraRoute<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.length.partial_cmp(&other.length)
    }
}

impl<T: Clone + Eq + Hash> Ord for DijkstraRoute<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length)
    }
}

pub struct Solver<'a, T: Clone + Eq + Hash> {
    nodes: HashMap<T, DijkstraNode<T>>,
    routes: BinaryHeap<Reverse<DijkstraRoute<'a, T>>>,
}

impl<'a, T: Clone + Eq + Hash + 'a> Solver<'a, T> {
    fn solve(&'a mut self, start: T, end: T) -> Option<(Vec<T>, u64)> {
        let start_node = &self.nodes[&start];
        self.routes.push(Reverse(DijkstraRoute::new(start_node)));

        while let Some(Reverse(current_route)) = self.routes.pop() {
            let current_node = current_route.current_node;
            if current_node.identifier == end {
                return Some((current_route.route, current_route.length));
            }

            if !current_node.explored.borrow().clone() {
                for (edge_end, edge_length) in &current_route.current_node.edges {
                    self.routes.push(
                        // Reverse for min heap.
                        Reverse(
                            current_route.with_extra_edge(*edge_length, &self.nodes[&edge_end]),
                        ),
                    );
                }
            }
            *current_node.explored.borrow_mut() = true;
        }

        // Did not find a route
        None
    }

    fn from_raw_map(all_edges: &HashMap<T, Vec<(T, u64)>>) -> Self {
        Solver {
            nodes: all_edges
                .iter()
                .map(|(id, edges)| (id.clone(), DijkstraNode::new(id.clone(), edges.clone())))
                .collect::<HashMap<T, DijkstraNode<T>>>(),
            routes: BinaryHeap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn simple_dijkstra() {
        let dijkstra_state = [
            (1, vec![(2, 1), (3, 4)]),
            (2, vec![(1, 2), (3, 2)]),
            (3, vec![]),
        ]
        .into_iter()
        .collect();
        let dijkstra_sln = solve(&dijkstra_state, 1, 3);
        assert_eq!(Some((vec![1, 2, 3], 3)), dijkstra_sln);
    }

    #[test]
    fn simple_dijkstra_uniform_edges() {
        let dijkstra_state = [(1, vec![2, 3]), (2, vec![1, 3]), (3, vec![])]
            .into_iter()
            .collect();
        let dijkstra_sln = solve_uniform_edges(&dijkstra_state, 1, 3);
        assert_eq!(Some((vec![1, 3], 1)), dijkstra_sln);
    }
}
