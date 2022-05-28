use std::io;
use thiserror::Error;

pub mod simplepathgraph;
pub mod staticgraph;


#[derive(Error, Debug)]
pub enum GraphError {
    #[error("could not create graph")]
    CreateError,
    #[error("graph I/O error")]
    IOError(#[from] io::Error),
    #[error("graph parse error")]
    ParseError,
    #[error("graph index error")]
    IndexError,
}

pub trait Vertex {
    fn index(&self) -> Option<usize>;
    fn get(index: usize) -> Option<Self>
    where
        Self: Sized;
}

impl<T: num::PrimInt + Sized> Vertex for T {
    fn index(&self) -> Option<usize> {
        self.to_usize()
    }
    fn get(index: usize) -> Option<T> {
        T::from(index)
    }
}

pub trait Graph<V: Vertex> {
    fn nv(&self) -> usize;
    fn ne(&self) -> usize;
    fn out_neighbors(&self, v: V) -> Vec<V>;
    fn in_neighbors(&self, v: V) -> Vec<V>;
    fn has_edge(&self, e: &(V, V)) -> bool;
    fn has_vertex(&self, v: &V) -> bool;
}
