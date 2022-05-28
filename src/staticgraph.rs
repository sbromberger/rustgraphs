use crate::{Graph, GraphError};
use graphmatrix::{GraphMatrix, GraphMatrixIterator};
use num;
use std::{fmt, fs};
use std::io::{BufRead, BufReader};

pub struct StaticGraph<T>
where
    T: num::PrimInt + std::fmt::Display,
{
    ne: usize,
    fadj: GraphMatrix<T>,
}

fn parse_line<T: std::str::FromStr>(line: &str) -> Option<(T, T)> {
        let (s, d) = line.split_once(",")?;
        let s: T = s.parse().ok()?;
        let d: T = d.parse().ok()?;
        Some((s, d))
}

impl<T: num::PrimInt + std::fmt::Display + std::str::FromStr> StaticGraph<T> {
    pub fn from_edgelist(el: Vec<(T, T)>) -> Result<StaticGraph<T>, GraphError> {
        let fadj = GraphMatrix::from_edgelist(el)
            .or_else(|_| Err(GraphError::CreateError))?;
        Ok(StaticGraph {
            ne: fadj.ne(),
            fadj,
        })
    }
    fn neighbors(&self, v: T) -> Vec<T> {
        match self.fadj.row(v) {
            Ok(row) => row.to_vec(),
            Err(_) => Vec::new(),
        }
    }
    pub fn edges(&self) -> StaticGraphEdgeIter<T> {
        StaticGraphEdgeIter::new(self)
    }

    pub fn from_edgefile(f: &str) -> Result<StaticGraph<T>, GraphError> {
        let file = fs::File::open(f)?;
        let reader = BufReader::new(file);
        let mut edges: Vec<(T, T)> = Vec::new();
        for line in reader.lines() {
            let edge = parse_line(&line?).ok_or(GraphError::ParseError)?;
            edges.push(edge);
        }
        Self::from_edgelist(edges)

    }
}

impl<T: num::PrimInt + std::fmt::Display + std::str::FromStr> Graph<T> for StaticGraph<T> {
    fn nv(&self) -> usize {
        self.fadj.dims().0
    }

    fn ne(&self) -> usize {
        self.ne
    }

    fn out_neighbors(&self, r: T) -> Vec<T> {
        self.neighbors(r)
    }

    fn in_neighbors(&self, r: T) -> Vec<T> {
        self.neighbors(r)
    }
    fn has_edge(&self, e: &(T, T)) -> bool {
        self.fadj.has_index(e.0, e.1).unwrap_or(false)
    }
    fn has_vertex(&self, v: &T) -> bool {
        *v < T::from(self.nv()).unwrap_or(T::zero())
    }
}

#[derive(Debug)]
pub struct StaticGraphEdgeIter<'a, V> {
    gmiter: GraphMatrixIterator<'a, V>,
}

impl<'a, V: num::PrimInt + std::fmt::Display> StaticGraphEdgeIter<'a, V> {
    pub fn new(g: &'a StaticGraph<V>) -> Self {
        let i = GraphMatrixIterator::new(&g.fadj);
        StaticGraphEdgeIter { gmiter: i }
    }
}

impl<'a, V: num::PrimInt + std::fmt::Display> Iterator for StaticGraphEdgeIter<'a, V> {
    type Item = (V, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.gmiter.next()
    }
}

impl<T: num::PrimInt + std::fmt::Display + std::str::FromStr> fmt::Display for StaticGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) StaticGraph<{}>",
            self.nv(),
            self.ne(),
            std::any::type_name::<T>()
        )
    }
}
