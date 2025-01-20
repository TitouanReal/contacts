// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use serde::Serialize;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct Mail {
    pub address: String,
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct Phone {
    pub number: String,
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct Contact {
    pub name: String,
    pub mails: Vec<Mail>,
    pub phones: Vec<Phone>,
}
