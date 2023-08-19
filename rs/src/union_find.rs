#![allow(dead_code)]

use std::default::Default;

#[derive(Clone, Default, Debug)]
struct UnionFind {
    parents: Vec<u32>,
    sizes: Vec<u32>,
}

impl UnionFind {
    fn new(n: u32) -> Self {
        let mut parents: Vec<u32> = Vec::new();
        for i in 0..n {
            parents.push(i);
        }

        Self {
            parents,
            sizes: vec![1; n as usize],
        }
    }

    fn find(&self, x: u32) -> u32 {
        if x == self.parents[x as usize] {
            x
        } else {
            self.find(self.parents[x as usize])
        }
    }

    fn unite(&mut self, x: u32, y: u32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return
        }
        if self.size(root_x) >= self.size(root_y) {
            self.parents[y as usize] = x;
            self.sizes[x as usize] += self.size(root_y);
        } else {
            self.parents[x as usize] = y;
            self.sizes[y as usize] += self.size(root_x);
        }
    }

    fn same(&self, x: u32, y: u32) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&self, x: u32) -> u32 {
        self.sizes[self.find(x) as usize]
    }
}

