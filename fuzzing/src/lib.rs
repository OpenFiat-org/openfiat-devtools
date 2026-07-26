//! Helper functions exercised by fuzz targets under `fuzz/fuzz_targets/`.
//! Real parsers/serializers will replace these stand-ins as they land in
//! `openfiat-core`.

/// Stand-in for a future wire-format parser.
pub fn parse_stub(input: &[u8]) -> Result<usize, &'static str> {
    if input.is_empty() {
        return Err("empty input");
    }
    Ok(input.len())
}
