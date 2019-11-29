// Copyright © 2019 Intel Corporation. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

//! rust-vmm device model.

extern crate vm_memory;

use std::io;
use vm_memory::GuestAddress;

pub mod resources;

/// IO Addresses.
#[derive(Debug, Copy, Clone)]
pub enum IoAddress {
    /// Port I/O address.
    Pio(u16),

    /// Memory mapped I/O address.
    Mmio(GuestAddress),
}

/// Device IO trait.
/// A device supporting memory based I/O should implement this trait, then
/// register itself against the different IO type ranges it handles.
/// The VMM will then dispatch IO (PIO or MMIO) VM exits by calling into the
/// registered devices read or write method from this trait.
pub trait DeviceIo: Send {
    /// Read from the guest physical address `addr` to `data`.
    fn read(&mut self, addr: IoAddress, data: &mut [u8]);

    /// Write `data` to the guest physical address `addr`.
    fn write(&mut self, addr: IoAddress, data: &[u8]);
}

/// Trait that needs to be implemented by all rust-vmm devices which
/// require an interrupt.
///
/// This trait can be extended for non-legacy interrupts as well.
/// In case of MSI-X, we can have the table configuration hold a vector
/// of <dyn Interrupt>.
pub trait Interrupt {
    fn trigger(&self) -> io::Error;
}