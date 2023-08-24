// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> ToBits for Plaintext<N> {
    /// Returns this plaintext as a list of **little-endian** bits.
    fn write_bits_le(&self, vec: &mut Vec<bool>) {
        match self {
            Self::Literal(literal, bits_le) => {
                // Compute the bits.
                let bits = bits_le.get_or_init(|| {
                    let mut bits_le = vec![false, false]; // Variant bits.
                    literal.variant().write_bits_le(&mut bits_le);
                    literal.size_in_bits().write_bits_le(&mut bits_le);
                    literal.write_bits_le(&mut bits_le);
                    bits_le
                });
                // Extend the vector with the bits.
                vec.extend_from_slice(bits)
            }
            Self::Struct(struct_, bits_le) => {
                // Compute the bits.
                let bits = bits_le.get_or_init(|| {
                    let mut bits_le = vec![false, true]; // Variant bits.

                    // Write the length of the struct.
                    u8::try_from(struct_.len())
                        .or_halt_with::<N>("Plaintext struct length exceeds u8::MAX")
                        .write_bits_le(&mut bits_le);

                    // Write each member of the struct.
                    for (identifier, value) in struct_ {
                        // Write the identifier of the member.
                        identifier.size_in_bits().write_bits_le(&mut bits_le);
                        identifier.write_bits_le(&mut bits_le);

                        // Write the value of the member.
                        let value_bits = value.to_bits_le();
                        u16::try_from(value_bits.len())
                            .or_halt_with::<N>("Plaintext member exceeds u16::MAX bits")
                            .write_bits_le(&mut bits_le);
                        bits_le.extend_from_slice(&value_bits);
                    }
                    bits_le
                });
                // Extend the vector with the bits.
                vec.extend_from_slice(bits)
            }
        }
    }

    /// Returns this plaintext as a list of **big-endian** bits.
    fn write_bits_be(&self, vec: &mut Vec<bool>) {
        match self {
            Self::Literal(literal, bits_be) => {
                // Compute the bits.
                let bits = bits_be.get_or_init(|| {
                    let mut bits_be = vec![false, false]; // Variant bits.
                    literal.variant().write_bits_be(&mut bits_be);
                    literal.size_in_bits().write_bits_be(&mut bits_be);
                    literal.write_bits_be(&mut bits_be);
                    bits_be
                });
                // Extend the vector with the bits.
                vec.extend_from_slice(bits)
            }
            Self::Struct(struct_, bits_be) => {
                // Compute the bits.
                let bits = bits_be.get_or_init(|| {
                    let mut bits_be = vec![false, true]; // Variant bits.

                    // Write the length of the struct.
                    u8::try_from(struct_.len())
                        .or_halt_with::<N>("Plaintext struct length exceeds u8::MAX")
                        .write_bits_be(&mut bits_be);

                    // Write each member of the struct.
                    for (identifier, value) in struct_ {
                        // Write the identifier of the member.
                        identifier.size_in_bits().write_bits_be(&mut bits_be);
                        identifier.write_bits_be(&mut bits_be);

                        // Write the value of the member.
                        let value_bits = value.to_bits_be();
                        u16::try_from(value_bits.len())
                            .or_halt_with::<N>("Plaintext member exceeds u16::MAX bits")
                            .write_bits_be(&mut bits_be);
                        bits_be.extend_from_slice(&value_bits);
                    }

                    bits_be
                });
                // Extend the vector with the bits.
                vec.extend_from_slice(bits)
            }
        }
    }
}
