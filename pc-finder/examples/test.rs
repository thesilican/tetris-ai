// use common::*;
// use pc_finder::*;
// use std::collections::HashMap;

fn main() {
    // let board = PcBoard::from_rows([0b0000001111, 0b0000010000, 0b0000100000, 0b0000011000]);
    // let board_ser = PcBoardSer::from(board);
    // let board_de = PcBoard::from(board_ser);
    // println!("{:#}", board);
    // println!("{}", board.is_valid());
    // println!("{:?}", board_ser);
    // println!("{:#}", board_de);

    // let graph = PcGraph::generate();
    // println!("Graph size: {}", graph.graph.len());
    // graph.save("data/test-1.bin").unwrap();
    // let graph_loaded = PcGraph::load("data/test-1.bin").unwrap();
    // println!("{}", graph == graph_loaded);

    // println!("{}", MOVES_4F.perms.len() < (1 << 16));

    // let arr: Vec<Vec<i32>> = vec![
    //     vec![0, 1, 2],
    //     vec![3, 4, 5, 6],
    //     vec![7, 8],
    //     vec![],
    //     vec![9, 10],
    // ];
    // let mut hash_map = HashMap::<&[i32], usize>::new();
    // for (i, val) in arr.iter().enumerate() {
    //     hash_map.insert(val, i);
    // }
    // println!("{:?}", hash_map);
}
