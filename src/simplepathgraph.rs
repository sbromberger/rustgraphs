use crate::Graph;
use crate::GraphError;
use std::fmt;

#[derive(Debug)]
pub struct SimplePathGraph {
    nv: usize,
}

impl SimplePathGraph {
    pub fn new(n: usize) -> Result<Self, GraphError> {
        if n == 0 {
            return Err(GraphError::GraphCreateError(String::from(
                "Graph must have at least one vertex",
            )));
        }
        Ok(SimplePathGraph { nv: n })
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        let mut n: Vec<usize> = Vec::new();
        if v > 0 {
            n.push(v - 1);
        }
        if v < self.nv - 1 {
            n.push(v + 1);
        }
        n
    }
}
impl Graph<usize> for SimplePathGraph {
    fn nv(&self) -> usize {
        self.nv
    }
    fn ne(&self) -> usize {
        self.nv - 1
    }
    fn in_neighbors(&self, v: usize) -> Vec<usize> {
        self.neighbors(v)
    }

    fn out_neighbors(&self, v: usize) -> Vec<usize> {
        self.neighbors(v)
    }
}

pub struct SimplePathGraphEdgeIterator {
    nv: usize,
    curr_vx: usize,
}

impl SimplePathGraphEdgeIterator {
    pub fn new(g: &SimplePathGraph) -> Self {
        SimplePathGraphEdgeIterator {
            nv: g.nv(),
            curr_vx: 0,
        }
    }
}
impl Iterator for SimplePathGraphEdgeIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_vx == self.nv - 1 {
            return None;
        }
        let e = (self.curr_vx, self.curr_vx + 1);
        self.curr_vx += 1;
        Some(e)
    }
}
impl fmt::Display for SimplePathGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) SimplePathGraph", self.nv(), self.ne())
    }
}
