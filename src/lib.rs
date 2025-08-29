// lib.rs
//
// Copyright (c) 2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

pub const MKLROOT: &str = concat!(env!("OUT_DIR"), "/mkl/latest");

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_mklroot() {
        assert!(Path::new(MKLROOT).join("include/mkl.h").exists());
        assert!(Path::new(MKLROOT).join("lib/libmkl_core.a").exists());
    }
}
