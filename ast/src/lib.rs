// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The abstract syntax tree (ast) for a Leo program.
//!
//! This module contains the [`Ast`] type, a wrapper around the [`Program`] type.
//! The [`Ast`] type is intended to be parsed and modified by different passes
//! of the Leo compiler. The Leo compiler can generate a set of R1CS constraints from any [`Ast`].
#[macro_use]
extern crate thiserror;

pub mod annotation;
pub use self::annotation::*;

pub mod circuits;
pub use self::circuits::*;

pub mod common;
pub use self::common::*;

pub mod errors;
pub use self::errors::*;

pub mod expression;
pub use self::expression::*;

pub mod functions;
pub use self::functions::*;

pub mod groups;
pub use self::groups::*;

pub mod imports;
pub use self::imports::*;

pub mod input;
pub use self::input::*;

pub mod program;
pub use self::program::*;

pub mod statements;
pub use self::statements::*;

pub mod types;
pub use self::types::*;

mod node;
pub use node::*;

use leo_grammar::Grammar;

/// The abstract syntax tree (AST) for a Leo program.
///
/// The [`Ast`] type represents a Leo program as a series of recursive data types.
/// These data types form a tree that begins from a [`Program`] type root.
///
/// A new [`Ast`] can be created from a [`Grammar`] generated by the pest parser in the `grammar` module.
#[derive(Debug, Eq, PartialEq)]
pub struct Ast {
    ast: Program,
}

impl Ast {
    /// Creates a new AST from a given program name and grammar tree.
    pub fn new<'ast>(program_name: &str, grammar: &Grammar<'ast>) -> Result<Self, AstError> {
        Ok(Self {
            ast: Program::from(program_name, grammar.as_repr())?,
        })
    }

    /// Returns a reference to the inner program AST representation.
    pub fn into_repr(self) -> Program {
        self.ast
    }

    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.ast)
    }

    /// Deserializes the JSON string into a ast.
    pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
        let ast: Program = serde_json::from_str(json)?;
        Ok(Self { ast })
    }
}
