
use std::io::{self,BufRead};
//~ use std::process;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
	prnt: Option<usize>,
	left: Option<usize>,
	rite: Option<usize>,
	line: Option<String>,
}

impl Node {
	fn new () -> Node {
		Node{
			prnt: None,
			left: None,
			rite: None,
			line: None
		}
	}
}

fn main() {
	
	let mut nodes: Vec<Node> = Vec::new();
	
	// first node
	nodes.push(Node::new());
	
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    nodes[0].line = Some(line_iter.next().unwrap().unwrap());
		
	let mut head = 0;

    // populate nodes
    for line_res in line_iter {
		
		loop {
			let line = line_res.as_ref().unwrap();
			let ndln = nodes.len(); // store next index before borrowing nref
			
			// determine direction
			let (next, nref) = match line.cmp(nodes[head].line.as_ref().unwrap()) {
				Ordering::Greater => { 
					(nodes[head].rite, &mut nodes[head].rite)
				},
				_ => {
					(nodes[head].left, &mut nodes[head].left)
				}
			};
			
			// write out if unoccupied
			if next.is_none() {
				// set the index, to relieve borrow checker of nref
				*nref = Some(ndln);
				nodes.push(Node::new());
				nodes[ndln].prnt = Some(head);
				nodes[ndln].line = Some(line.to_string());
				
				head = 0;
				break;
			} else {
				head = next.unwrap();
			}
		}	
	}
	
	while nodes[head].left.is_some() {
		head = nodes[head].left.unwrap();
	}
	
	
	while nodes[0].left.is_some() || nodes[0].line.is_some() || nodes[0].rite.is_some() {
		// traverse left
		if nodes[head].left.is_some() {
			head = nodes[head].left.unwrap();
			continue;
		}
		
		// print self
		if nodes[head].line.is_some() {
			println!("{}", nodes[head].line.as_ref().unwrap());
			nodes[head].line = None;
			continue;
		// traverse right
		} else if nodes[head].rite.is_some() {
			head = nodes[head].rite.unwrap();
			continue;
		}
		
		if head == 0 {
			break;
		}
		
		let prnt = nodes[head].prnt.unwrap();
		if nodes[prnt].left.is_some() && nodes[prnt].left.unwrap() == head {
			nodes[prnt].left = None;
		} else if nodes[prnt].rite.is_some(){
			nodes[prnt].rite = None;
		}
		head = prnt;
	}
	
}
