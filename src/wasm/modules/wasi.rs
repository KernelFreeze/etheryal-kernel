// MIT License
//
// Copyright (c) 2021 Miguel Peláez
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use wasmi::{
    Error, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef, ModuleImportResolver,
    Signature, TableDescriptor, TableRef,
};

use crate::prelude::*;

pub struct WasiImportResolver;

impl WasiImportResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl ModuleImportResolver for WasiImportResolver {
    /// Resolve a function.
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        Err(Error::Instantiation(format!("Export {} not found", field_name)))
    }

    /// Resolve a global variable.
    fn resolve_global(&self, field_name: &str, _global_type: &GlobalDescriptor) -> Result<GlobalRef, Error> {
        Err(Error::Instantiation(format!("Export {} not found", field_name)))
    }

    /// Resolve a memory.
    fn resolve_memory(&self, field_name: &str, _memory_type: &MemoryDescriptor) -> Result<MemoryRef, Error> {
        Err(Error::Instantiation(format!("Export {} not found", field_name)))
    }

    /// Resolve a table.
    fn resolve_table(&self, field_name: &str, _table_type: &TableDescriptor) -> Result<TableRef, Error> {
        Err(Error::Instantiation(format!("Export {} not found", field_name)))
    }
}
