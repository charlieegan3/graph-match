extern crate graph_match;

use std::collections::HashMap;
use graph_match::graph;

#[test]
fn traversal_simple() {
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), None);
    let node1 = simple_graph.add_node("node1".to_string(), None);
    let node2 = simple_graph.add_node("node2".to_string(), None);
    simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
    simple_graph.add_edge(node1, node2, "edge1".to_string(), None);

    assert_eq!(graph_match::expand_subgraph(&simple_graph, 0, &vec![]), vec![0,1,2]);
}

#[test]
fn traveral_incomplete() {
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), None);
    let node1 = simple_graph.add_node("node1".to_string(), None);
    let node2 = simple_graph.add_node("node2".to_string(), None);
    simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
    simple_graph.add_edge(node2, node1, "edge1".to_string(), None);

    assert_eq!(graph_match::expand_subgraph(&simple_graph, 0, &vec![]), vec![0,1]);
}

#[test]
fn traveral_restricted() {
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), None);
    let node1 = simple_graph.add_node("node1".to_string(), None);
    let node2 = simple_graph.add_node("node2".to_string(), None);
    let node3 = simple_graph.add_node("node3".to_string(), None);
    simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
    simple_graph.add_edge(node1, node2, "banned".to_string(), None);
    simple_graph.add_edge(node1, node3, "edge2".to_string(), None);

    assert_eq!(graph_match::expand_subgraph(&simple_graph, 0, &vec!["banned".to_string()]), vec![0,1,3]);
}

#[test]
fn match_complete_graph() {
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), None);
    let node1 = simple_graph.add_node("node1".to_string(), None);
    let node2 = simple_graph.add_node("node2".to_string(), None);
    simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
    simple_graph.add_edge(node1, node2, "edge1".to_string(), None);

    let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = query_graph.add_node("node0".to_string(), None);
    let node1 = query_graph.add_node("node1".to_string(), None);
    let node2 = query_graph.add_node("node2".to_string(), None);
    query_graph.add_edge(node0, node1, "edge0".to_string(), None);
    query_graph.add_edge(node1, node2, "edge1".to_string(), None);

    assert_eq!(graph_match::match_graph(query_graph, 0, simple_graph), true);
}

#[test]
fn match_subgraph() {
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), None);
    let node1 = simple_graph.add_node("node1".to_string(), None);
    let node2 = simple_graph.add_node("node2".to_string(), None);
    let node3 = simple_graph.add_node("node3".to_string(), None);
    simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
    simple_graph.add_edge(node1, node2, "edge1".to_string(), None);
    simple_graph.add_edge(node2, node3, "edge1".to_string(), None);

    let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = query_graph.add_node("node0".to_string(), None);
    let node1 = query_graph.add_node("node1".to_string(), None);
    let node2 = query_graph.add_node("node2".to_string(), None);
    query_graph.add_edge(node0, node1, "edge0".to_string(), None);
    query_graph.add_edge(node1, node2, "edge1".to_string(), None);

    assert_eq!(graph_match::match_graph(query_graph, 0, simple_graph), true);
}

#[test]
fn match_failure() {
    let mut attributes = HashMap::new();
    attributes.insert("key".to_string(), "value".to_string());
    let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = simple_graph.add_node("node0".to_string(), Some(attributes.clone()));
    let node1 = simple_graph.add_node("node1".to_string(), Some(attributes.clone()));
    simple_graph.add_edge(node0, node1, "edge0".to_string(), Some(attributes.clone()));

    attributes.insert("key2".to_string(), "value2".to_string());
    let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
    let node0 = query_graph.add_node("node0".to_string(), Some(attributes.clone()));
    let node1 = query_graph.add_node("node1".to_string(), Some(attributes.clone()));
    query_graph.add_edge(node0, node1, "edge0".to_string(), Some(attributes.clone()));

    assert_eq!(graph_match::match_graph(query_graph, 0, simple_graph), false);
}
