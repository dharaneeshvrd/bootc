//! # Bootable container tool
//!
//! This crate builds on top of ostree's container functionality
//! to provide a fully "container native" tool for using
//! bootable container images.

// See https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unused_must_use)]
#![deny(unsafe_code)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![deny(clippy::dbg_macro)]
#![deny(clippy::todo)]

mod blockdev;
mod bootloader;
pub mod cli;
mod containerenv;
pub(crate) mod ignition;
mod install;
mod lsm;
mod podman;
#[cfg(feature = "internal-testing-api")]
mod privtests;
mod reexec;
mod status;
mod task;
mod utils;

#[cfg(feature = "docgen")]
mod docgen;
