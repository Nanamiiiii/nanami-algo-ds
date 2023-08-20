#![allow(dead_code)]

use std::default::Default;

#[derive(Clone, Default, Debug)]
pub struct UnionFind {
    pub n: i32,
    pub parents: Vec<i32>,
    pub sizes: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: i32) -> Self {
        let mut parents: Vec<i32> = Vec::new();
        for i in 0..n {
            parents.push(i);
        }

        Self {
            n,
            parents,
            sizes: vec![1; n as usize],
        }
    }

    pub fn find(&self, x: i32) -> i32 {
        if x == self.parents[x as usize] {
            x
        } else {
            self.find(self.parents[x as usize])
        }
    }

    pub fn unite(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return
        }
        if self.size(root_x) >= self.size(root_y) {
            self.parents[root_y as usize] = root_x;
            self.sizes[root_x as usize] += self.sizes[root_y as usize];
        } else {
            self.parents[root_x as usize] = root_y;
            self.sizes[root_y as usize] += self.sizes[root_x as usize];
        }
    }

    pub fn same(&self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size_max(&self) -> i32 {
        let mut max = 0;
        for i in &self.sizes {
            if *i > max {
                max = *i
            } 
        }
        max
    }

    pub fn size(&self, x: i32) -> i32 {
        self.sizes[self.find(x) as usize]
    }
}
