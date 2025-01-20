// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

mod contacts_daemon;

use crate::utils::{Contact, Mail, Phone};
use contacts_daemon::ContactsDaemonProxy;
use std::io::BufReader;
use zbus::Connection;

use super::ContactBackend;

pub struct Backend {}

impl ContactBackend for Backend {
    async fn get_contacts() -> Result<Vec<(u64, Contact)>, ()> {
        let daemon = get_contacts_daemon_proxy().await?;

        let mut contacts = Vec::new();

        let vcards = match daemon.get_contacts().await {
            Ok(vcards) => vcards,
            Err(e) => {
                tracing::error!("{e}");
                return Err(());
            }
        };

        for (id, vcard) in vcards {
            let buf = BufReader::new(vcard.as_bytes());

            let mut reader = ical::VcardParser::new(buf);

            // Read only the first contact of the vCard string passed.
            match reader.next() {
                None => {
                    continue;
                }
                Some(Err(_)) => {
                    continue;
                }
                Some(Ok(data)) => {
                    let version = data
                        .properties
                        .iter()
                        .find(|property| property.name == "VERSION")
                        .unwrap()
                        .value
                        .as_ref()
                        .unwrap()
                        .to_owned();

                    if version != "4.0" {
                        continue;
                    }

                    let name = data
                        .properties
                        .iter()
                        .find(|property| property.name == "FN")
                        .unwrap()
                        .value
                        .as_ref()
                        .unwrap()
                        .to_owned();

                    let mails = data
                        .properties
                        .iter()
                        .filter(|property| property.name == "EMAIL")
                        .map(|property| Mail {
                            address: property.value.as_ref().unwrap().to_owned(),
                        })
                        .collect();

                    let phones = data
                        .properties
                        .iter()
                        .filter(|property| property.name == "TEL")
                        .map(|property| Phone {
                            number: property.value.as_ref().unwrap().to_owned(),
                        })
                        .collect();

                    contacts.push((
                        id,
                        Contact {
                            name,
                            mails,
                            phones,
                        },
                    ));
                }
            }
        }

        Ok(contacts)
    }

    async fn _add_contact(_info: Contact) -> Result<u64, ()> {
        unimplemented!()
    }

    async fn _update_contact(_id: u64, _info: Contact) -> Result<(), ()> {
        unimplemented!()
    }

    async fn _remove_contact(id: u64) -> Result<(), ()> {
        let daemon = get_contacts_daemon_proxy().await.unwrap();

        daemon.remove_contact(id).await.unwrap();

        Ok(())
    }
}

async fn get_contacts_daemon_proxy<'a>() -> Result<ContactsDaemonProxy<'a>, ()> {
    let connection = match Connection::session().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("zbus connection failed. {e}");
            return Err(());
        }
    };

    match ContactsDaemonProxy::new(&connection).await {
        Ok(d) => Ok(d),
        Err(e) => {
            tracing::error!("Contacts daemon proxy can't be created. Is it installed? {e}");
            Err(())
        }
    }
}
