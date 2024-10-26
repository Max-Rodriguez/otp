/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

//! Data model that represents a single parameter of an atomic
//! field, which together form a RPC method signature.

use crate::dcatomic::DCAtomicField;
use crate::dctype::DCTypeDefinition;
use crate::hashgen::*;
use std::rc::Rc;

/// Represents the type specification of a parameter within an atomic field.
#[derive(Debug)]
pub struct DCParameter<'dc> {
    parent: Rc<DCAtomicField<'dc>>,
    base_type: DCTypeDefinition,
    identifier: Option<String>,
    type_alias: String,
    default_value: Vec<u8>,
    has_default_value: bool,
}

impl<'dc> std::fmt::Display for DCParameter<'dc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

impl<'dc> LegacyDCHash for DCParameter<'dc> {
    fn generate_hash(&self, hashgen: &mut DCHashGenerator) {
        self.base_type.generate_hash(hashgen);
    }
}

impl<'dc> DCParameter<'dc> {
    #[inline(always)]
    pub fn get_atomic_field(&self) -> Rc<DCAtomicField<'dc>> {
        Rc::clone(&self.parent) // clone new rc pointer
    }

    #[inline(always)]
    pub fn has_default_value(&self) -> bool {
        self.has_default_value
    }

    #[inline(always)]
    pub fn get_default_value(&self) -> Vec<u8> {
        self.default_value.clone()
    }

    pub fn set_type(&mut self, dtype: DCTypeDefinition) {
        self.base_type = dtype;
    }

    pub fn set_identifier(&mut self, name: &str) {
        self.identifier = Some(name.to_owned());
    }

    pub fn set_default_value(&mut self, v: Vec<u8>) {
        self.default_value = v;
        self.has_default_value = true;
    }
}
