// build.rs
//
// Copyright (c) 2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

const MKL_INSTALLER_URL: &str = "https://registrationcenter-download.intel.com/akdlm/IRC_NAS/47c7d946-fca1-441a-b0df-b094e3f045ea/intel-onemkl-2025.2.0.629.sh";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(target_os = "macos") {
        panic!("MacOS is not supported")
    } else if cfg!(target_os = "windows") {
        panic!("Windows is not supported yet")
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = out_dir.join("mkl/latest/lib");
    if !lib_dir.exists() {
        let res = ureq::get(MKL_INSTALLER_URL)
            .call()
            .expect("Failed to fetch MKL installer");
        assert_eq!(
            res.status(),
            200,
            "Download failed with status {}",
            res.status()
        );

        let installer_name = MKL_INSTALLER_URL.split('/').last().unwrap();
        io::copy(
            &mut io::BufReader::new(res.into_body().into_reader()),
            &mut fs::File::create(out_dir.join(installer_name)).unwrap(),
        )
        .unwrap();

        let installation_args = vec![
            installer_name,
            "-a",
            "-s",
            "--eula",
            "accept",
            "--install-dir",
            out_dir.to_str().unwrap(),
        ];

        let mut cmd = Command::new("sh");
        cmd.current_dir(&out_dir);
        cmd.args(&installation_args);
        cmd.env("HOME", &out_dir);
        let status = cmd.status().expect("Failed to run MKL installer");
        assert!(
            status.success(),
            "Failed to install MKL with status: {:?}",
            status.code().unwrap()
        );
    }

    println!("cargo:rustc-link-search={}", lib_dir.display());
    if cfg!(feature = "lp64") {
        println!("cargo:rustc-link-lib=static=mkl_intel_lp64");
    } else {
        println!("cargo:rustc-link-lib=static=mkl_intel_ilp64");
    }
    if cfg!(feature = "openmp") {
        println!("cargo:rustc-link-lib=static=mkl_intel_thread");
    } else {
        println!("cargo:rustc-link-lib=static=mkl_sequential");
    }
    println!("cargo:rustc-link-lib=static=mkl_core");
    println!("cargo:rustc-link-lib=mkl_rt");
    println!("cargo::metadata=ROOT={}", out_dir.join("mkl/latest").display());
}
