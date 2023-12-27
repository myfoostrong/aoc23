use std::{collections::{HashMap, HashSet}, borrow::Borrow, cmp::min};
use rand::random;
use petgraph::{graph::{Graph,  EdgeIndex, NodeIndex, EdgeReference, Node}, data::Build, Undirected, algo::condensation, visit::NodeRef};
use petgraph::dot::{Dot, Config};
use petgraph::visit::EdgeRef;


pub fn solve1(file_path: &str) -> usize {
  let mut total = 0;
  if let Ok(lines) = aoc23::read_lines(file_path) {
    let grid_length = aoc23::get_file_line_size(file_path);
    let mut map:  HashMap<String, NodeIndex> = HashMap::new();
    // let mut comps_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut g: Graph<String, u32, Undirected> = Graph::new_undirected();
    
    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        let component: Vec<_> = row.split(' ').map(|x| x.to_string()).collect();
        // update_map(&map, component)
        let name = &component[0][..3].to_string();
        let node = getsert(&mut g, &mut map, name);
        for conn in component[1..].iter() {
          let conn_node = getsert(&mut g, &mut map, conn);
          g.add_edge(node, conn_node, 1);
        }
        // comps.push(component[1..].to_vec());
        
        // comps_map.insert(name.clone(), component[1..].to_vec());
        // g.extend_with_edges(component[1..].iter().map(|c| (name, c)))
      }
    }
    
    let (x, g1) = fast_min_cut(&g);   
    let mut t = 1;
    // println!("{}")
    for w in g1.node_weights() {
      t *= w.split(',').count();
    }
    total = t;
    // let c = condensation(g, true);
    println!("{:?}", Dot::with_config(&g1, &[Config::EdgeNoLabel]));
  }
  total 
}

fn fast_min_cut(g: &Graph<String, u32, Undirected>) -> (usize, Graph<String, u32, Undirected>) {
  let V = g.node_count();
  let E = g.edge_count();
  let mut min_edges = E;
  let mut g_out: Graph<String, u32, Undirected> = Graph::new_undirected();
  let limit = (V as f32 * f32::log10(V as f32) / (V - 1) as f32) as usize;
  println!("Limit: {}", limit);
  for i in 0..limit {
    let (j, g1) = _fast_min_cut(g);
    if min_edges > j {
      min_edges = j;
      g_out = g1.clone();
    }
    // min_edges = min(min_edges, _fast_min_cut(g));
  }
  let mut vertices = V;
  // let mut new_g = contract(&g);
  // println!("{:?}", Dot::with_config(&new_g, &[Config::EdgeIndexLabel]));
  // new_g.edge_count()
  (min_edges, g_out)
}

fn _fast_min_cut(g: &Graph<String, u32, Undirected>) -> (usize, Graph<String, u32, Undirected>) {
  if g.node_count() <= 6 {
    let (i, g1) = karger(g);
    return (i, g1)
  } else {
    let t = f32::floor((1 + g.node_count()) as f32 / f32::sqrt(2.0)) as usize;
    let g1 = contract(g, t);
    let g2 = contract(g, t);
    let (i, g3) = _fast_min_cut(&g1);
    let (j, g4) = _fast_min_cut(&g2);
    if i > j {
      return (j, g4.clone())
    } else {
      return (i, g3.clone())
    }
    // return min(_fast_min_cut(&g1), _fast_min_cut(&g2))
  }
}

fn karger(g: &Graph<String, u32, Undirected>) -> (usize, Graph<String, u32, Undirected>) {
  let V = g.node_count();
  let E = g.edge_count();
  let mut min_edges = E;
  let mut g1: Graph<String, u32, Undirected> = g.clone();
  let mut g_out: Graph<String, u32, Undirected> = Graph::new_undirected();
  let limit = (V * (V-1) * (f32::log10(V as f32) / 2.0) as usize);
  println!("Limit: {}", limit);
  for i in 0..limit {
    g1 = contract(&g1, 2);
    if min_edges > g1.edge_count() {
      min_edges = g1.edge_count();
      g_out = g1.clone();
    }
    // min_edges = min(min_edges, g1.edge_count());
  }
  (min_edges, g_out)
}

fn contract(graph: &Graph<String, u32, Undirected>, t: usize) -> Graph<String, u32, Undirected> {
  let mut g = graph.clone();
  while g.node_count() > t {
    let r: usize = random::<usize>() % g.edge_count();
    let edge = g.edge_indices().nth(r).expect("Shit");
    merge_verticies(&mut g, edge);
  }
  g
}

fn merge_verticies(g: &mut Graph<String, u32, Undirected>, edge: EdgeIndex) {
  let (x, y) = g.edge_endpoints(edge).expect("Shit");
  let mut edges = Vec::new();
  for e in g.edges(x) {
    let mut n = NodeIndex::new(1);
    if x == e.source() {
      n = e.target();
    } else {
      n = e.source();
    }
    if n != y {
      edges.push(n)
    }
  }
  let q: Vec<EdgeIndex> = edges.iter().map(|e| g.add_edge(*e, y, 1)).collect();
  let v = x.weight();
  

  let mut v = g.node_weight_mut(x).expect("Shit").clone();
  let mut w = g.node_weight_mut(y).expect("Shit");
  // println!("{:?}", v);
  w.push_str(&format!(",{}",&v));
  g.remove_node(x);
}

fn getsert(g: &mut Graph<String, u32, Undirected>, map: &mut HashMap<String, NodeIndex>, name: &String) -> NodeIndex {
  if !map.contains_key(&name.clone()) {
    let nidx = g.add_node(name.clone());
    map.insert(name.clone(), nidx);
    nidx
  } else {
    map[name]
  }
}



// fn update_map(map: &HashMap<String, HashSet<String>>, component: Vec<String>) {
//   let mut name: String = String::new();
//   for (i, conn) in component.iter().enumerate() {
//     if i == 0 {
//       name = conn[0..2].to_string();
//       if !map.contains_key(&name) {
//         map.insert(name.clone(), HashSet::new());
//       }
//     }
//     let mut comp = &map[&name.clone().borrow()];
//     comp.insert(conn.to_string());
//     if !map.contains_key(conn) {
//       map.insert(conn.clone(), HashSet::new());
//     }
//     map.get_mut(conn).expect("Shit").insert(name);
//   }
// }

pub fn solve2(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u32::try_from(grid_length).unwrap();
    
    for (j, line) in lines.enumerate() {
      if let Ok(row) = line {
        // println!("{}",row);
        
      }
    }
    
  }
  total
}