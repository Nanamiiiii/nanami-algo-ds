#![allow(dead_code)]

use std::cmp::{Ordering, Reverse};
use std::collections::{ BinaryHeap, TryReserveError };
use std::collections::binary_heap::{ Iter, PeekMut, Drain };

pub struct PriorityQueue<T, P: Ord + Copy> {
    q: BinaryHeap<ElemWithPriority<T, P>>,
}

impl<T, P: Ord + Copy> PriorityQueue<T, P> {
    pub fn new() -> Self {
        Self {
            q: BinaryHeap::<ElemWithPriority<T, P>>::new(),
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            q: BinaryHeap::<ElemWithPriority<T, P>>::with_capacity(size),
        }
    }

    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, ElemWithPriority<T, P>>> {
        self.q.peek_mut()
    }

    pub fn pop(&mut self) -> Option<ElemWithPriority<T, P>> {
        self.q.pop()
    } 

    pub fn push(&mut self, item: T, priority: P) {
        self.q.push(ElemWithPriority(item, priority));
    }

    pub fn into_sorted_vec(self) -> Vec<ElemWithPriority<T, P>> {
        self.q.into_sorted_vec()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.q.append(&mut other.q);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&ElemWithPriority<T, P>) -> bool,
    {
        self.q.retain(f);
    }

    pub fn iter(&self) -> Iter<'_, ElemWithPriority<T, P>> {
        self.q.iter()
    }

    pub fn peek(&self) -> Option<&ElemWithPriority<T, P>> {
        self.q.peek()
    }

    pub fn capacity(&self) -> usize {
        self.q.capacity()
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.q.reserve_exact(additional);
    }

    pub fn reserve(&mut self, additional: usize) {
        self.q.reserve(additional);
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.q.try_reserve_exact(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.q.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.q.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.q.shrink_to(min_capacity);
    }

    pub fn into_vec(self) -> Vec<ElemWithPriority<T, P>> {
        self.q.into_vec()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn drain(&mut self) -> Drain<'_, ElemWithPriority<T, P>> {
        self.q.drain()
    }

    pub fn clear(&mut self) {
        self.q.clear();
    }
}

pub struct ElemWithPriority<T, P: Ord + Copy>(pub T, pub P);

impl<T, P: Ord + Copy> ElemWithPriority<T, P> {
    fn new(obj: T, priority: P) -> ElemWithPriority<T, P> {
        ElemWithPriority(obj, priority)
    }

    fn rev(obj: T, priority: P) -> Reverse<ElemWithPriority<T, P>> {
        Reverse(ElemWithPriority::new(obj, priority))
    }
}

impl<T, P: Ord + Copy> Eq for ElemWithPriority<T, P> {}

impl<T, P: Ord + Copy> PartialEq for ElemWithPriority<T, P> {
    fn eq(&self, other: &ElemWithPriority<T, P>) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T, P: Ord + Copy> PartialOrd for ElemWithPriority<T, P> {
    fn partial_cmp(&self, other: &ElemWithPriority<T, P>) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl<T, P: Ord + Copy> Ord for ElemWithPriority<T, P> {
    fn cmp(&self, other: &ElemWithPriority<T, P>) -> Ordering {
        self.1.cmp(&other.1)
    }
}
