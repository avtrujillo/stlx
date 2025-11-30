use std::marker::PhantomData;

use dfdx::{shapes::Shape, tensor::{NoneTape, Storage, Tape, Tensor}};
use num_traits::Float;

/// Marker trait for tensors with differentiable element types
// TODO: figure out whether we ever actually use this
trait Differentiable {}

impl<S: Shape, E: Float, D: Storage<E>, T> Differentiable for Tensor<S, E, D, T> {}

pub struct Abs<SI, SO, EI, EO, DI, DO, TI, TO> where
    SI: Shape, SO: Shape,
    EI: Float, EO: Float,
    DI: Storage<EI>, DO: Storage<EO>,
    TI: Tape<EI, DI>, TO: Tape<EO, DO>
{
    input: PhantomData<Tensor<SI, EI, DI, TI>>,
    output: PhantomData<Tensor<SO, EO, DO, TO>>
}