// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

mod dbus;
mod duckdb;

use crate::utils::Contact;

pub type Backend = dbus::Backend;

pub trait ContactBackend {
    /// Return all the contacts.
    async fn get_contacts() -> Result<Vec<(u64, Contact)>, ()>;

    /// Add a contact to the backend.
    ///
    /// If successful, returns the id of the created contact.
    async fn _add_contact(info: Contact) -> Result<u64, ()>;

    /// Update the contact associated with `id` with the provided `info`.
    async fn _update_contact(id: u64, info: Contact) -> Result<(), ()>;

    /// Remove the contact associated with `id`.
    async fn _remove_contact(id: u64) -> Result<(), ()>;
}
