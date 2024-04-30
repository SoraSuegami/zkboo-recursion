use itertools::Itertools;
use thiserror::Error;

use crate::backend::*;
use crate::finite::*;
use crate::*;

#[derive(Error, Debug)]
pub enum NativeError {}

pub trait NativeHasher<F: FiniteRing>: Debug + Clone + Default {
    fn new() -> Self;
    fn hash(&self, inputs: &[F]) -> Result<Vec<F>, NativeError>;
}

#[derive(Debug, Clone, Default)]
pub struct NativeBackend<F: FiniteRing, H: NativeHasher<F>> {
    pub hasher_prefix: Vec<F>,
    pub hasher: H,
}

impl<F: FiniteRing, H: NativeHasher<F>> Backend<F> for NativeBackend<F, H> {
    type V = F;
    type Error = NativeError;

    fn new(hasher_prefix: Vec<F>) -> Result<Self, Self::Error> {
        let hasher = NativeHasher::new();
        Ok(NativeBackend {
            hasher_prefix,
            hasher,
        })
    }

    fn load_value(&mut self, a: &Self::V) -> Result<(), Self::Error> {
        Ok(())
    }

    fn expose_value(&mut self, a: &Self::V) -> Result<(), Self::Error> {
        Ok(())
    }

    fn constant(&mut self, a: &F) -> Result<Self::V, Self::Error> {
        Ok(a.clone())
    }

    fn add(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Self::Error> {
        Ok(a.add(b))
    }

    fn mul(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Self::Error> {
        Ok(a.mul(b))
    }

    fn sub(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Self::Error> {
        Ok(a.sub(b))
    }

    fn neg(&mut self, a: &Self::V) -> Result<Self::V, Self::Error> {
        Ok(a.neg())
    }

    fn eq(&mut self, a: &Self::V, b: &Self::V) -> Result<Self::V, Self::Error> {
        Ok(if a == b { F::one() } else { F::zero() })
    }

    fn hash_input_share(
        &mut self,
        rand_seed: &[Self::V],
        input_idx: u32,
    ) -> Result<Self::V, Self::Error> {
        let outputs = self
            .hasher
            .hash(&[rand_seed.to_vec(), vec![F::from(input_idx)]].concat())?;
        Ok(outputs[0])
    }

    fn hash_mul_pad(
        &mut self,
        rand_seed: &[Self::V],
        input: &[Self::V],
    ) -> Result<Self::V, Self::Error> {
        let outputs = self
            .hasher
            .hash(&[rand_seed.to_vec(), input.to_vec()].concat())?;
        Ok(outputs[0])
    }

    fn hash_commit(
        &mut self,
        rand_seed: &[Self::V],
        input: &[Self::V],
    ) -> Result<Vec<Self::V>, Self::Error> {
        let outputs = self
            .hasher
            .hash(&[rand_seed.to_vec(), input.to_vec()].concat())?;
        Ok(outputs)
    }

    fn hash_each_transcript(&mut self, input: &[Self::V]) -> Result<Vec<Self::V>, Self::Error> {
        let outputs = self.hasher.hash(&input)?;
        Ok(outputs)
    }

    fn hash_challenge(&mut self, input: &[Self::V]) -> Result<Vec<Self::V>, Self::Error> {
        self.hasher.hash(&input)
    }
}