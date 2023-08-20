#![allow(dead_code)]

#[derive(Eq, Copy, Clone, Debug)]
pub struct WeightedEdge {
    pub vertex_a: i32,
    pub vertex_b: i32,
    pub cost: i32,
}

impl WeightedEdge {
    pub fn new(vertex_a: i32, vertex_b: i32, cost: i32) -> Self {
        Self {
            vertex_a,
            vertex_b,
            cost,
        }
    }
}

impl PartialEq for WeightedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

#[derive(Clone, Debug)]
pub struct WeightedGraph {
    vertex_n: i32,
    edge_set: Vec<WeightedEdge>,    
}

impl WeightedGraph {
    pub fn new(vertex_n: i32, edge_set: Vec<WeightedEdge>) -> Self {
        Self {
            vertex_n,
            edge_set,
        }
    }

    pub fn edges(&self) -> &Vec<WeightedEdge> {
        &self.edge_set 
    }

    pub fn sort_by_weight(&mut self) {
        self.edge_set.sort()
    }

    pub fn order(&self) -> i32 {
        self.vertex_n
    }

    pub fn size(&self) -> usize {
        self.edge_set.len()
    }

    pub fn delete_edge(&mut self, pos: usize) {
        self.edge_set.remove(pos);
    }
}