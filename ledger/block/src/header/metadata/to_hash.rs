// Copyright 2024 Aleo Network Foundation
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

impl<N: Network> Metadata<N> {
    /// Returns the metadata hash.
    pub fn to_hash(&self) -> Result<Field<N>> {
        println!("IN TO_HASH cumulative_weight for block {}: {}", self.height, self.cumulative_weight);
        
        // Construct the metadata bits
        let metadata_bits = self.to_bits_le();
        println!("First 32 bits of metadata: {:?} for block {}", &metadata_bits[..32], self.height);
        
        // Hash and return
        let metadata_hash = N::hash_bhp1024(&metadata_bits)?;
        println!("Metadata hash: {:?} for block {}", metadata_hash, self.height);
        Ok(metadata_hash)
    }
}
