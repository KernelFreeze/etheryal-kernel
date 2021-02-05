// MIT License
//
// Copyright (c) 2021 The etheryal Project Developers
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

use core::ptr::NonNull;

use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};

#[derive(Clone)]
pub struct EtheryalAcpiHandler {
    offset: usize,
}

impl EtheryalAcpiHandler {
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }
}

impl AcpiHandler for EtheryalAcpiHandler {
    /// Given a physical address and a size, map a region of physical memory
    /// that contains `T` (note: the passed size may be larger than
    /// `size_of::<T>()`). The address is not necessarily page-aligned, so the
    /// implementation may need to map more than `size` bytes. The virtual
    /// address the region is mapped to does not matter, as long as it is
    /// accessible to `acpi`.
    unsafe fn map_physical_region<T>(
        &self, physical_address: usize, size: usize,
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new((physical_address + self.offset) as *mut T)
                .expect("Failed to map virtual address for ACPI"),
            region_length: size,
            mapped_length: size,
            handler: self.clone(),
        }
    }

    /// Unmap the given physical mapping. This is called when a
    /// `PhysicalMapping` is dropped.
    fn unmap_physical_region<T>(&self, _region: &PhysicalMapping<Self, T>) {}
}

pub unsafe fn create_acpi_tables(
    memory_offset: usize, rsdp_address: usize,
) -> AcpiTables<EtheryalAcpiHandler> {
    let handler = EtheryalAcpiHandler::new(memory_offset);
    let tables = AcpiTables::from_rsdp(handler, rsdp_address)
        .expect("Failed to parse required ACPI tables from bootloader provided RSDP.");

    tables
}
