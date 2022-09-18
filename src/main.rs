extern crate core;

use std::collections::{HashMap, VecDeque};

fn main() {
    let graph : HashMap<char,&[char]> = HashMap::from([
        ('a', ['b', 'c'].as_slice()),
        ('b',['d'].as_slice()),
        ('c',['e'].as_slice()),
        ('d',['f'].as_slice()),
        ('e',[].as_slice()),
        ('f',[].as_slice())
    ]);
    breadth_first_search(graph,'a');
}
type Graph = HashMap<char,&'static[char]>;
// normal analytical method

fn depth_first_search(graph : Graph,source : char) -> (){
   //abdfce or acebdf as it uses a stack that is lifo
    let mut stack = vec![source];
    // for (el,ch) in graph {
    //
    // }
    while stack.len()>0 {
        let current = stack.pop().unwrap();
        println!("{}",current);
        for ch in graph[&current].iter(){
           stack.push(*ch) ;
        }
    }
}
// by recursion
fn depth_first_search_recurse(graph :Graph,source : char){
    println!("{}",source);
    let me = graph.clone();
    for ch in graph[&source].iter(){

        depth_first_search_recurse(me.clone(),*ch);
    }
}
fn breadth_first_search(graph :Graph,source : char){
    let mut queue = vec![source];
    while queue.len() >0{
        let current = queue.remove(0);
        println!("{}",current);
        for ch in graph[&current]{
        queue.push(*ch);
        }
    }
}