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

use crate::{ConstraintFieldError, Field, FieldParameters, Fp2, Fp2Parameters, PrimeField, ToConstraintField};
use snarkvm_utilities::FromBits;

impl<F: PrimeField> ToConstraintField<F> for F {
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        Ok(vec![*self])
    }
}

// Impl for base field
impl<F: Field> ToConstraintField<F> for [F] {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        Ok(self.to_vec())
    }
}

impl<F: Field> ToConstraintField<F> for Vec<F> {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        Ok(self.to_vec())
    }
}

impl<F: Field> ToConstraintField<F> for () {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        Ok(Vec::new())
    }
}

// Impl for constraint Fp2<F>
impl<P: Fp2Parameters> ToConstraintField<P::Fp> for Fp2<P> {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<P::Fp>, ConstraintFieldError> {
        let mut c0 = self.c0.to_field_elements()?;
        let c1 = self.c1.to_field_elements()?;
        c0.extend_from_slice(&c1);
        Ok(c0)
    }
}

impl<F: PrimeField> ToConstraintField<F> for [bool] {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        Ok(self
            .chunks(<F as PrimeField>::Parameters::CAPACITY as usize)
            .map(|chunk| F::from_repr(F::BigInteger::from_bits_le(chunk)).unwrap())
            .collect::<Vec<F>>())
    }
}

impl<F: PrimeField, const NUM_BITS: usize> ToConstraintField<F> for [bool; NUM_BITS] {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        self.as_ref().to_field_elements()
    }
}

impl<F: PrimeField> ToConstraintField<F> for [u8] {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        // Derive the field size in bytes, floored to be conservative.
        let floored_field_size_in_bytes = (<F as PrimeField>::Parameters::CAPACITY / 8) as usize;

        // Pack the bytes into field elements.
        Ok(self
            .chunks(floored_field_size_in_bytes)
            .map(|chunk| {
                // Before packing, pad the chunk to the next power of two.
                let mut chunk = chunk.to_vec();
                chunk.resize(floored_field_size_in_bytes.next_power_of_two(), 0u8);
                F::read_le(chunk.as_slice())
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}

impl<F: PrimeField, const NUM_BYTES: usize> ToConstraintField<F> for [u8; NUM_BYTES] {
    #[inline]
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        self.as_ref().to_field_elements()
    }
}

impl<F: Field> ToConstraintField<F> for bool {
    fn to_field_elements(&self) -> Result<Vec<F>, ConstraintFieldError> {
        if *self { Ok(vec![F::one()]) } else { Ok(vec![F::zero()]) }
    }
}
