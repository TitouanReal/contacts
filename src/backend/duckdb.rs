// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use super::ContactBackend;
use crate::utils::Contact;
use duckdb::Connection;

pub struct Backend {}

impl Backend {
    fn init() {
        let db = Connection::open("data/contacts.duckdb").unwrap();

        db.execute("CREATE SEQUENCE IF NOT EXISTS seq_contactid START 4;", [])
            .unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS contacts (
                id UINTEGER PRIMARY KEY DEFAULT NEXTVAL('seq_contactid'),
                name STRING,
                emails STRING[]
            );",
            [],
        )
        .unwrap();

        db.execute(
            "INSERT OR IGNORE INTO contacts (id, name, emails) VALUES
                (1, 'John Doe', LIST['john.doe@example.com', 'john.doe@example2.com']),
                (2, 'Jane Doe', LIST['jane.doe@example.com']),
                (3, 'Alice Smith', LIST['jane.doe@example.com']);",
            [],
        )
        .unwrap();
    }
}

impl ContactBackend for Backend {
    async fn get_contacts() -> Result<Vec<(u64, Contact)>, ()> {
        Self::init();

        let db = Connection::open("data/contacts.duckdb").unwrap();

        let mut stmt = db.prepare("SELECT id, name, mails FROM contacts").unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    Contact {
                        name: row.get(1)?,
                        // TODO: Support mails
                        mails: vec![],
                        // TODO: Support phones
                        phones: vec![],
                    },
                ))
            })
            .unwrap()
            .map(|person| person.unwrap());

        Ok(person_iter.collect())
    }

    async fn _add_contact(_info: Contact) -> Result<u64, ()> {
        unimplemented!()
    }

    async fn _update_contact(_id: u64, _info: Contact) -> Result<(), ()> {
        unimplemented!()
    }

    async fn _remove_contact(_id: u64) -> Result<(), ()> {
        unimplemented!()
    }
}
