// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::production::Production;
use crate::grammar::symbol::Symbol;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct State {
    pub(crate) name:         String,
    pub(crate) production:   Production,
    pub(crate) dot_index:    usize,
    pub(crate) start_column: usize,
    pub(crate) end_column:   usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut symbols: Vec<String> =
            self.production.symbols.iter().map(Symbol::to_string).collect();
        symbols.insert(self.dot_index, "•".to_string());

        write!(
            f,
            "{} := {} [{}-{}]",
            self.name,
            symbols.join(" "),
            self.start_column,
            self.end_column,
        )
    }
}

impl State {
    pub(crate) fn completed(&self) -> bool {
        self.dot_index >= self.production.symbols.len()
    }

    pub(crate) fn next_symbol(&self) -> Option<Symbol> {
        if self.completed() {
            None
        } else {
            Some(self.production.symbols[self.dot_index].clone())
        }
    }

    pub(crate) fn hash_me(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
