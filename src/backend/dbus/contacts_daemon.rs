// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use zbus::proxy;
#[proxy(
    interface = "com.github.TitouanReal.ContactsDaemon",
    default_service = "com.github.TitouanReal.ContactsDaemon",
    default_path = "/com/github/TitouanReal/ContactsDaemon"
)]
pub trait ContactsDaemon {
    /// GetContacts method
    fn get_contacts(&self) -> zbus::Result<Vec<(u64, String)>>;
    /// AddContact method
    fn add_contact(&self, vcard: String) -> zbus::Result<()>;
    /// RemoveContact method
    fn remove_contact(&self, id: u64) -> zbus::Result<()>;
}
