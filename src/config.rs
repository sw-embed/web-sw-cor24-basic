//! VM configuration — pre-assembled pvm.s binary and label addresses.
//!
//! The COR24 assembler runs at build time (in build.rs), not in WASM.
//! This module provides the pre-assembled binary and key label addresses.

pub const PVM_BINARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pvm.bin"));

include!(concat!(env!("OUT_DIR"), "/pvm_labels.rs"));

pub fn label_addr(name: &str) -> u32 {
    PVM_LABELS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, a)| *a)
        .unwrap_or(0)
}

pub const BASIC_P24: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/basic.p24"));
