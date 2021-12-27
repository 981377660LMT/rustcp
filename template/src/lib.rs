//io
pub mod fast_input;
pub mod fast_input_test;

//math
pub mod algebraic_structure;
pub mod math;
pub mod arithmetic;

//rand
pub mod rand;
pub mod shuffle;

//number
pub mod num_integer;
pub mod num_real;
pub mod num_number;
pub mod num_gcd;
pub mod dynamic_modint;
pub mod static_modint;
pub mod binary;
pub mod affine;
pub mod permutation;

// ds
pub mod dsu_compress_path;
pub mod prefix_sum;
pub mod sparse_table;
pub mod binary_lifting_compress_on_tree;
pub mod range_minimum_query;
pub mod fenwick_tree;
pub mod segtree;
pub mod segment;
pub mod range_affine_range_sum;
pub mod segtree_beat_ext;
pub mod persistent_segtree;
pub mod lazy_persistent_segtree;
pub mod range_kth_smallest_persistent_segtree;
pub mod treap;
pub mod trie;
pub mod lichao_segtree;
pub mod range_tree;

//graph
pub mod root_tree;
pub mod graph;
pub mod link_cut_tree;
pub mod shortest_path;
pub mod cycle_detect;
pub mod strongly_connected_component;
pub mod topo_sort;

//algo
pub mod binary_search;
pub mod template_macro;

//util
pub mod id;
pub mod collection;
pub mod function;

//flow
pub mod max_flow_dinic;
pub mod max_flow;
pub mod cost_flow;
pub mod cost_flow_augment;
pub mod cost_flow_augment_dijkstra;

//match
pub mod bipartite_match_dinic;
pub mod bipartite_match_kuhn_random;
pub mod general_graph_match;
pub mod bipartite_minimum_weight_match;
pub mod bipartite_maximum_weight_match_km;