//! # Alkahest-Lexer for WebAssembly 3.0
//! 
//! This crate provides a lexer implementation for WebAssembly 3.0 (Wasm 3.0).
//! It is designed to tokenize Wasm 3.0 source code, breaking it down into
//! manageable pieces for further processing by a parser or compiler.
#![cfg_attr(not(test), no_std)]

pub mod token;
pub use crate::token::{Token, TokenKind};