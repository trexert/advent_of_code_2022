use num::{Bounded, Num};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub fn solve<Id: Clone + Eq + Hash, L: Copy + Bounded + Num + Ord>(
    all_edges: &HashMap<Id, Vec<(Id, L)>>,
    routes: &Vec<(Id, Id)>,
) -> Vec<Option<L>>
where
    <L as Num>::FromStrRadixErr: Debug,
{
    let mut solver: Solver<Id, L> = Solver::from_raw_map(all_edges);
    solver.solve();
    routes
        .into_iter()
        .map(|(start, end)| solver.matrix[solver.id_to_node[start]][solver.id_to_node[end]])
        .collect()
}

pub fn solve_uniform_edges<Id: Clone + Eq + Hash, L: Copy + Bounded + Num + Ord>(
    all_edges: &HashMap<Id, Vec<Id>>,
    routes: &Vec<(Id, Id)>,
) -> Vec<Option<L>>
where
    <L as Num>::FromStrRadixErr: Debug,
{
    solve(
        &all_edges
            .iter()
            .map(|(id, edges)| {
                (
                    id.clone(),
                    edges
                        .iter()
                        .map(|edge| (edge.clone(), L::from_str_radix("1", 10).unwrap()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect(),
        routes,
    )
}

struct Solver<Id: Clone + Eq + Hash, L: Copy + Bounded + Num + Ord>
where
    <L as Num>::FromStrRadixErr: Debug,
{
    node_count: usize,
    matrix: Vec<Vec<Option<L>>>,
    id_to_node: HashMap<Id, usize>,
}

impl<Id: Clone + Eq + Hash, L: Copy + Bounded + Num + Ord> Solver<Id, L>
where
    <L as Num>::FromStrRadixErr: Debug,
{
    fn from_raw_map(all_edges: &HashMap<Id, Vec<(Id, L)>>) -> Self {
        let node_count = all_edges.len();
        let mut matrix = vec![vec![None; node_count]; node_count];
        let id_to_node: HashMap<Id, usize> = all_edges
            .into_iter()
            .enumerate()
            .map(|(i, (id, _))| (id.clone(), i))
            .collect();
        for (node, edges) in all_edges {
            for (edge_end, length) in edges {
                matrix[id_to_node[node]][id_to_node[edge_end]] = Some(*length);
            }
        }
        Solver {
            node_count,
            matrix,
            id_to_node,
        }
    }

    fn solve(&mut self) {
        for intermediate in 0..self.node_count {
            for from in 0..self.node_count {
                for to in 0..self.node_count {
                    self.matrix[from][to] = match (
                        self.matrix[from][to],
                        self.matrix[from][intermediate],
                        self.matrix[intermediate][to],
                    ) {
                        (Some(direct), Some(via1), Some(via2)) => Some(direct.min(via1 + via2)),
                        (None, Some(via1), Some(via2)) => Some(via1 + via2),
                        (Some(direct), _, _) => Some(direct),
                        (_, _, _) => None,
                    }
                }
            }
        }
    }
}
