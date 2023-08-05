// use std::collections::HashMap;
// use std::env;
// use std::error::Error;
// use std::fs::read_to_string;

// fn main() {
//     let content: Vec<i32> = "1\n2\n-3\n3\n-2\n0\n4"
//         .split("\n")
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
//
//     // let mut vals = content.clone();
//     // let order: HashMap<_, _> = idx.zip(content).collect();
//
//     // defines the initial order of the values in the file
//     // this is required because the initial ordering must be
//     // used while decrypting the file using the "mixing" algorithm
//     let order: HashMap<_, _> = HashMap::from_iter((0..content.len()).zip(&content));
//     dbg!(order);
//
//     /*     init: 1 2 -3 3 -2 0 4
//      * [0] { 1}: 2 1 -3 3 -2 0 4
//      * [1] { 2}: 1 -3 2 3 -2 0 4
//      * [2] {-3}: 1 2 3 -2 -3 0 4
//      * [3] { 3}: 1 2 -2 -3 0 3 4
//      * [4] {-2}: 1 2 -3 0 3 4 -2
//      * [5] { 0}: 1 2 -3 0 3 4 -2
//      * [6] { 4}: 1 2 -3 4 0 3 -2
//      */
// }

fn main() {}
