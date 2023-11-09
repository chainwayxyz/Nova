//! This module defines a collection of traits that define the behavior of a `zkSNARK` for `RelaxedR1CS`
use crate::{
  errors::NovaError,
  r1cs::{R1CSShape, RelaxedR1CSInstance, RelaxedR1CSWitness},
  traits::Group,
  CommitmentKey,
};
use rand_core::RngCore;

use serde::{Deserialize, Serialize};

/// A trait that defines the behavior of a `zkSNARK`
pub trait RelaxedR1CSSNARKTrait<G: Group>:
  Send + Sync + Serialize + for<'de> Deserialize<'de>
{
  /// A type that represents the prover's key
  type ProverKey: Send + Sync + Serialize + for<'de> Deserialize<'de>;

  /// A type that represents the verifier's key
  type VerifierKey: Send + Sync + Serialize + for<'de> Deserialize<'de> + DigestHelperTrait<G>;

  /// Produces the keys for the prover and the verifier
  fn setup(
    ck: &CommitmentKey<G>,
    S: &R1CSShape<G>,
    rng: impl RngCore,
  ) -> Result<(Self::ProverKey, Self::VerifierKey, G::PreprocessedGroupElement), NovaError>;

  /// Produces a new SNARK for a relaxed R1CS
  fn prove(
    ck: &CommitmentKey<G>,
    pk: &Self::ProverKey,
    S: &R1CSShape<G>,
    U: &RelaxedR1CSInstance<G>,
    W: &RelaxedR1CSWitness<G>,
  ) -> Result<Self, NovaError>;

  /// Verifies a SNARK for a relaxed R1CS
  fn verify(&self, vk: &Self::VerifierKey, U: &RelaxedR1CSInstance<G>) -> Result<(), NovaError>;
}

/// A helper trait that defines the behavior of a verifier key of `zkSNARK`
pub trait DigestHelperTrait<G: Group> {
  /// Returns the digest of the verifier's key
  fn digest(&self) -> G::Scalar;
}
