#[cfg(test)]
mod kruskal_test {
    use rstest::rstest;

    use nanami_algo_ds::algo::kruskal::Kruskal;
    use nanami_algo_ds::ds::graph::WeightedEdge;
    use nanami_algo_ds::ds::graph::WeightedGraph;

    // Case 0
    static ORDER_CASE_0: i32 = 5;
    static EDGES_CASE_0: [[i32; 3]; 7] = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]];
    static COST_CASE_0: i32 = 7;
    static CRITICAL_CASE_0: [i32; 2] = [0, 1];
    static PSEUDO_CRITICAL_CASE_0: [i32; 4] = [2, 3, 4, 5];
    // Case 1
    static ORDER_CASE_1: i32 = 4;
    static EDGES_CASE_1: [[i32; 3]; 4] = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]];
    static COST_CASE_1: i32 = 3;
    static CRITICAL_CASE_1: [i32; 0] = [];
    static PSEUDO_CRITICAL_CASE_1: [i32; 4] = [0, 1, 2, 3];
    // Case 2 
    static ORDER_CASE_2: i32 = 6;
    static EDGES_CASE_2: [[i32; 3]; 7] = [[0,1,1],[1,2,1],[0,2,1],[2,3,4],[3,4,2],[3,5,2],[4,5,2]];
    static COST_CASE_2: i32 = 10;
    static CRITICAL_CASE_2: [i32; 1] = [3];
    static PSEUDO_CRITICAL_CASE_2: [i32; 6] = [0, 1, 2, 4, 5, 6];
    // Case 3 
    static ORDER_CASE_3: i32 = 4;
    static EDGES_CASE_3: [[i32; 3]; 3] = [[0,1,1],[0,2,2],[0,3,3]];
    static COST_CASE_3: i32 = 6;
    static CRITICAL_CASE_3: [i32; 3] = [0,1,2];
    static PSEUDO_CRITICAL_CASE_3: [i32; 0] = [];

    // Test 0: Deriving MST Cost
    #[rstest]
    #[case(ORDER_CASE_0, &EDGES_CASE_0, COST_CASE_0)]
    #[case(ORDER_CASE_1, &EDGES_CASE_1, COST_CASE_1)]
    #[case(ORDER_CASE_2, &EDGES_CASE_2, COST_CASE_2)]
    #[case(ORDER_CASE_3, &EDGES_CASE_3, COST_CASE_3)]
    fn derive_mst_cost(#[case] order: i32, #[case] edges: &[[i32; 3]], #[case] expected_cost: i32) {
        let mut weighted_edges = Vec::<WeightedEdge>::new();
        for arr in edges {
            weighted_edges.push(WeightedEdge::new(arr[0], arr[1], arr[2]));
        }

        let graph = WeightedGraph::new(order, weighted_edges.clone());
        let mut kruskal = Kruskal::new(graph);

        assert_eq!(kruskal.derive_mst().get_cost(), expected_cost);
    }

    // Test 1: Deriving Critical & Pseudo Critical Edges
    #[rstest]
    #[case(ORDER_CASE_0, &EDGES_CASE_0, &CRITICAL_CASE_0, &PSEUDO_CRITICAL_CASE_0)]
    #[case(ORDER_CASE_1, &EDGES_CASE_1, &CRITICAL_CASE_1, &PSEUDO_CRITICAL_CASE_1)]
    #[case(ORDER_CASE_2, &EDGES_CASE_2, &CRITICAL_CASE_2, &PSEUDO_CRITICAL_CASE_2)]
    #[case(ORDER_CASE_3, &EDGES_CASE_3, &CRITICAL_CASE_3, &PSEUDO_CRITICAL_CASE_3)]
    fn derive_critical_and_pseudo_critical_edges(#[case] order: i32, #[case] edges: &[[i32; 3]], #[case] expected_critical: &[i32], #[case] expected_pseudo: &[i32]) {
        let mut weighted_edges = Vec::<WeightedEdge>::new();
        for arr in edges {
            weighted_edges.push(WeightedEdge::new(arr[0], arr[1], arr[2]));
        }

        let graph = WeightedGraph::new(order, weighted_edges.clone());
        let mut kruskal = Kruskal::new(graph);
        kruskal.derive_mst();

        let critical = kruskal.derive_critical_edges();
        assert_eq!(critical, expected_critical.to_vec());

        let pseudo = kruskal.derive_pseudo_critical_edges(&critical);
        assert_eq!(pseudo, expected_pseudo.to_vec());
    }
}
