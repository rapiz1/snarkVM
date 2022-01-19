// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    ahp::{indexer::Circuit, prover::ProverConstraintSystem, verifier::VerifierFirstMessage},
    marlin::MarlinMode,
    Vec,
};
use snarkvm_algorithms::fft::EvaluationDomain;
use snarkvm_fields::PrimeField;
use snarkvm_polycommit::{LabeledPolynomial, Polynomial};

/// State for the AHP prover.
pub struct ProverState<'a, F: PrimeField, MM: MarlinMode> {
    pub(super) padded_public_variables: Vec<F>,
    pub(super) private_variables: Vec<F>,
    /// query bound b
    pub(super) zk_bound: usize,
    /// Az
    pub(super) z_a: Option<Vec<F>>,
    /// Bz
    pub(super) z_b: Option<Vec<F>>,

    pub(super) w_poly: Option<LabeledPolynomial<F>>,
    pub(super) mz_polys: Option<(LabeledPolynomial<F>, LabeledPolynomial<F>)>,
    pub(super) mz_poly_randomizer: Option<(F, F)>,

    pub(super) index: &'a Circuit<F, MM>,

    /// the random values sent by the verifier in the first round
    pub(super) verifier_first_message: Option<VerifierFirstMessage<F>>,

    /// the blinding polynomial for the first round
    pub(super) mask_poly: Option<LabeledPolynomial<F>>,

    /// the blinding polynomial for the first round
    pub(super) t_poly: Option<Polynomial<F>>,

    /// domain X, sized for the public input
    pub(super) domain_x: EvaluationDomain<F>,

    /// domain H, sized for constraints
    pub(super) domain_h: EvaluationDomain<F>,

    /// domain K, sized for matrix nonzero elements
    pub(super) domain_k: EvaluationDomain<F>,
}

impl<'a, F: PrimeField, MM: MarlinMode> ProverState<'a, F, MM> {
    pub fn initialize(
        padded_public_input: Vec<F>,
        private_variables: Vec<F>,
        zk_bound: usize,
        index: &'a Circuit<F, MM>,
        domain_x: EvaluationDomain<F>,
        domain_h: EvaluationDomain<F>,
        domain_k: EvaluationDomain<F>,
    ) -> Self {
        Self {
            padded_public_variables: padded_public_input,
            private_variables,
            zk_bound,
            index,
            domain_x,
            domain_h,
            domain_k,
            mask_poly: None,
            t_poly: None,
            verifier_first_message: None,
            w_poly: None,
            mz_poly_randomizer: None,
            mz_polys: None,
            z_a: None,
            z_b: None,
        }
    }

    /// Get the public input.
    pub fn public_input(&self) -> Vec<F> {
        ProverConstraintSystem::unformat_public_input(&self.padded_public_variables)
    }

    /// Get the padded public input.
    pub fn padded_public_input(&self) -> &[F] {
        &self.padded_public_variables
    }
}
