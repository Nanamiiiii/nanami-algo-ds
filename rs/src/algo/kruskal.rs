#![allow(dead_code)]
#![allow(unused_imports)]

use crate::ds::graph::{ WeightedEdge, WeightedGraph };
use crate::ds::union_find::UnionFind;

#[derive(Clone, Debug)]
pub struct Kruskal {
    graph: WeightedGraph, 
    cost: i32,
}

impl Kruskal {
    pub fn new(graph: WeightedGraph) -> Self {
        Self {
            graph,
            cost: -1,
        }
    }

    pub fn derive_mst(&mut self) -> &mut Self {
        let graph = &mut self.graph.clone();
        graph.sort_by_weight();
        let mut cost = 0;
        let mut uf = UnionFind::new(graph.order());
        for i in 0..graph.size() {
            if !uf.same(graph.edges()[i].vertex_a, graph.edges()[i].vertex_b) {
                cost += graph.edges()[i].cost;
                uf.unite(graph.edges()[i].vertex_a, graph.edges()[i].vertex_b);
            }
        }
        self.cost = cost;
        self
    }

    pub fn get_cost(&self) -> i32 {
        self.cost
    }

    pub fn derive_critical_edges(&mut self) -> Vec<i32> {
        if self.cost < 0 {
            panic!("You must derive MST cost before deriving critical edges.")
        }

        let mut critical_edges = Vec::<i32>::new();

        for i in 0..self.graph.size() {
            let ign_graph = &mut self.graph.clone();
            ign_graph.delete_edge(i);
            ign_graph.sort_by_weight();
            let mut cost = 0;
            let mut uf = UnionFind::new(ign_graph.order());
            for j in 0..ign_graph.size() {
                if !uf.same(ign_graph.edges()[j].vertex_a, ign_graph.edges()[j].vertex_b) {
                    cost += ign_graph.edges()[j].cost;
                    uf.unite(ign_graph.edges()[j].vertex_a, ign_graph.edges()[j].vertex_b);
                }
            }
            if cost > self.cost || uf.size_max() < uf.n {
                critical_edges.push(i as i32);
            }
        }

        critical_edges
    }

    pub fn derive_pseudo_critical_edges(&mut self, critical: &Vec<i32>) -> Vec<i32> {
        if self.cost < 0 {
            panic!("You must derive MST cost before deriving critical edges.")
        }

        let mut pseudo_critical_edges = Vec::<i32>::new();

        for i in 0..self.graph.size() {
            if critical.contains(&(i as i32)) {
                continue;
            }

            let graph = &mut self.graph.clone();

            let mut uf = UnionFind::new(graph.order());
            let mut cost = 0;
            uf.unite(graph.edges()[i].vertex_a, graph.edges()[i].vertex_b);
            cost += graph.edges()[i].cost;

            graph.delete_edge(i);
            graph.sort_by_weight();

            for j in 0..graph.size() {
                if !uf.same(graph.edges()[j].vertex_a, graph.edges()[j].vertex_b) {
                    cost += graph.edges()[j].cost;
                    uf.unite(graph.edges()[j].vertex_a, graph.edges()[j].vertex_b);
                }
            }

            if cost == self.cost {
                pseudo_critical_edges.push(i as i32);
            }
        }

        pseudo_critical_edges
    }
}
