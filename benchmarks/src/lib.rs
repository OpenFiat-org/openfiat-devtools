//! Placeholder functions benchmarked in `benches/`, standing in for real
//! protocol operations (serialization, signature verification, etc.) until
//! `openfiat-core`'s implementations land.

/// Stand-in for a canonical serialization round-trip.
pub fn roundtrip_stub(input: &[u8]) -> Vec<u8> {
    input.to_vec()
}
