#[cfg(test)]
mod priorityqueue_test {
    use rstest::rstest;

    use nanami_algo_ds::ds::priority_queue::PriorityQueue;

    // <i32, i32>
    static ELEMS_CASE_0: [(i32, i32); 3] = [(0, 10), (5, 1), (99, 22)];
    static POP_ORDER_CASE_0: [i32; 3] = [99, 0, 5]; 

    #[rstest]
    #[case(&ELEMS_CASE_0, &POP_ORDER_CASE_0)]
    fn simple_push_pop_i32_i32(#[case] elems: &[(i32, i32)], #[case] pop_order: &[i32]) {
        let mut priority_q = PriorityQueue::<i32, i32>::new();
        for (i, p) in elems {
            priority_q.push(*i, *p);
        }
        for i in pop_order {
            assert_eq!(*i, *priority_q.pop().unwrap().item());
        }
        assert!(priority_q.pop().is_none())
    }
}