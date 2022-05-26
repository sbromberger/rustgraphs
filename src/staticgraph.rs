use crate::{Graph, GraphError};
use graphmatrix::{GraphMatrix, GraphMatrixIterator};
use num;
use std::fmt;

pub struct StaticGraph<T>
where
    T: num::PrimInt,
{
    ne: usize,
    fadj: GraphMatrix<T>,
}

impl<T: num::PrimInt> StaticGraph<T> {
    pub fn from_edgelist(el: Vec<(T, T)>) -> Result<StaticGraph<T>, GraphError> {
        let fadj = GraphMatrix::from_edgelist(el).or(Err(GraphError::GraphCreateError(
            String::from("Invalid edges"),
        )))?;
        Ok(StaticGraph {
            ne: fadj.ne(),
            fadj,
        })
    }
    fn neighbors(&self, r: T) -> Vec<T> {
        match self.fadj.row(r) {
            Ok(rr) => rr.to_vec(),
            Err(_) => Vec::new(),
        }
    }
}

impl<T: num::PrimInt> Graph<T> for StaticGraph<T> {
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

impl<T: num::PrimInt> fmt::Display for StaticGraph<T> {
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
