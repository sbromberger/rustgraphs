use num;
pub mod simplepathgraph;
pub mod staticgraph;

#[derive(Debug)]
pub enum GraphError {
    GraphCreateError(String),
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
        Some(T::from(index)?)
    }
}

pub trait Graph<V: Vertex> {
    fn nv(&self) -> usize;
    fn ne(&self) -> usize;
    fn out_neighbors(&self, v: V) -> Vec<V>;
    fn in_neighbors(&self, v: V) -> Vec<V>;
}
