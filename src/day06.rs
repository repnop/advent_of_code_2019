use aoc_runner_derive::aoc;
use petgraph::{graph::NodeIndex, Direction, Graph, Undirected};
use std::collections::HashMap;

fn gen_part1(input: &str) -> (Graph<&str, &str>, NodeIndex<u32>) {
    let mut graph = Graph::<&str, &str>::new();
    let mut com = None;
    let mut map = HashMap::new();

    for line in input.lines() {
        let idx = line.chars().position(|c| c == ')').unwrap();
        let (orbiter, orbitee) = (&line[0..idx], &line[idx + 1..]);

        let is_com = orbiter == "COM";

        let orbiter = *map
            .entry(orbiter)
            .or_insert_with(|| graph.add_node(orbiter));
        let orbitee = *map
            .entry(orbitee)
            .or_insert_with(|| graph.add_node(orbitee));

        if is_com {
            com = Some(orbiter);
        }

        graph.add_edge(orbiter, orbitee, "");
    }

    (graph, com.unwrap())
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let (graph, start) = gen_part1(input);
    part1_inner(&graph, start, 0)
}

fn part1_inner(graph: &Graph<&str, &str>, start: NodeIndex<u32>, mut start_val: usize) -> usize {
    let mut count = start_val;
    start_val += 1;

    for neighbor in graph.neighbors_directed(start, Direction::Outgoing) {
        count += part1_inner(graph, neighbor, start_val);
    }

    count
}

fn gen_part2(
    input: &str,
) -> (
    Graph<&str, &str, Undirected>,
    NodeIndex<u32>,
    NodeIndex<u32>,
) {
    let mut graph = Graph::new_undirected();
    let mut you = None;
    let mut san = None;
    let mut map = HashMap::new();

    for line in input.lines() {
        let idx = line.chars().position(|c| c == ')').unwrap();
        let (orbiter, orbitee) = (&line[0..idx], &line[idx + 1..]);

        let is_you = orbitee == "YOU";
        let is_san = orbitee == "SAN";

        let orbiter = *map
            .entry(orbiter)
            .or_insert_with(|| graph.add_node(orbiter));
        let orbitee = *map
            .entry(orbitee)
            .or_insert_with(|| graph.add_node(orbitee));

        if is_you {
            you = Some(orbiter);
        } else if is_san {
            san = Some(orbiter);
        }

        graph.add_edge(orbiter, orbitee, "");
    }

    (graph, you.unwrap(), san.unwrap())
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let (graph, start, end) = gen_part2(input);

    *petgraph::algo::dijkstra(&graph, start, Some(end), |_| 1)
        .get(&end)
        .unwrap()
}

#[test]
fn part1_example() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    assert_eq!(part1(input), 42);
}

#[test]
fn part2_example() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    assert_eq!(part2(input), 4);
}
