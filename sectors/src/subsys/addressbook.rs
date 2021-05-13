use capnp::serialize_packed;
use std::io::{BufRead, Write};
use capnp::message::{Builder, HeapAllocator, TypedReader};

pub mod addressbook {
    use super::*;
    use crate::addressbook_capnp::{address_book, person};

    pub fn write_address_book<W>(write: W) -> ::capnp::Result<()> where W:Write {
        let mut message = ::capnp::message::Builder::new_default();
        {
            let address_book = message.init_root::<address_book::Builder>();

            let mut people = address_book.init_people(2);

            {
                let mut alice = people.reborrow().get(0);
                alice.set_id(123);
                alice.set_name("Alice");
                alice.set_email("alice@example.com");
                {
                    let mut alice_phones = alice.reborrow().init_phones(1);
                    alice_phones.reborrow().get(0).set_number("555-1212");
                    alice_phones.reborrow().get(0).set_type(person::phone_number::Type::Mobile);
                }
                alice.get_employment().set_school("MIT");
            }

            {
                let mut bob = people.get(1);
                bob.set_id(456);
                bob.set_name("Bob");
                bob.set_email("bob@example.com");
                {
                    let mut bob_phones = bob.reborrow().init_phones(2);
                    bob_phones.reborrow().get(0).set_number("555-4567");
                    bob_phones.reborrow().get(0).set_type(person::phone_number::Type::Home);
                    bob_phones.reborrow().get(1).set_number("555-7654");
                    bob_phones.reborrow().get(1).set_type(person::phone_number::Type::Work);
                }
                bob.get_employment().set_unemployed(());
            }
        }

        serialize_packed::write_message(write, &message)
    }

    pub fn print_address_book<R>(read: R) -> ::capnp::Result<()> where R: BufRead{
        // let stdin = ::std::io::stdin();
        let message_reader = serialize_packed::read_message(read,
                                                            ::capnp::message::ReaderOptions::new())?;
        let address_book = message_reader.get_root::<address_book::Reader>()?;

        for person in address_book.get_people()?.iter() {
            println!("{}: {}", person.get_name()?, person.get_email()?);
            for phone in person.get_phones()?.iter() {
                let type_name = match phone.get_type() {
                    Ok(person::phone_number::Type::Mobile) => "mobile",
                    Ok(person::phone_number::Type::Home) => "home",
                    Ok(person::phone_number::Type::Work) => "work",
                    Err(::capnp::NotInSchema(_)) => "UNKNOWN",
                };
                println!("  {} phone: {}", type_name, phone.get_number()?);
            }
            match person.get_employment().which() {
                Ok(person::employment::Unemployed(())) => {
                    println!("  unemployed");
                }
                Ok(person::employment::Employer(employer)) => {
                    println!("  employer: {}", employer?);
                }
                Ok(person::employment::School(school)) => {
                    println!("  student at: {}", school?);
                }
                Ok(person::employment::SelfEmployed(())) => {
                    println!("  self-employed");
                }
                Err(::capnp::NotInSchema(_)) => { }
            }
        }
        Ok(())
    }

    pub fn build_address_book() -> TypedReader<Builder<HeapAllocator>, address_book::Owned> {
        let mut message = Builder::new_default();
        {
            let address_book = message.init_root::<address_book::Builder>();

            let mut people = address_book.init_people(2);

            {
                let mut alice = people.reborrow().get(0);
                alice.set_id(123);
                alice.set_name("Alice");
                alice.set_email("alice@example.com");
                {
                    let mut alice_phones = alice.reborrow().init_phones(1);
                    alice_phones.reborrow().get(0).set_number("555-1212");
                    alice_phones.reborrow().get(0).set_type(person::phone_number::Type::Mobile);
                }
                alice.get_employment().set_school("MIT");
            }

            {
                let mut bob = people.get(1);
                bob.set_id(456);
                bob.set_name("Bob");
                bob.set_email("bob@example.com");
                {
                    let mut bob_phones = bob.reborrow().init_phones(2);
                    bob_phones.reborrow().get(0).set_number("555-4567");
                    bob_phones.reborrow().get(0).set_type(person::phone_number::Type::Home);
                    bob_phones.reborrow().get(1).set_number("555-7654");
                    bob_phones.reborrow().get(1).set_type(person::phone_number::Type::Work);
                }
                bob.get_employment().set_unemployed(());
            }
        }

        // There are two ways to get a TypedReader from our `message`:
        //
        // Option 1: Go through the full process manually
        //  message.into_reader().into_typed()
        //
        // Option 2: Use the "Into" trait defined on the builder
        //   message.into()
        //
        // Option 3: Use the "From" trait defined on the builder
        TypedReader::from(message)
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn addr_works() -> anyhow::Result<()> {
        addressbook::write_address_book(&mut ::std::io::stdout())?;
        // addressbook::print_address_book()?;

        Ok(())
    }

    #[test]
    fn addr_sent_works() -> anyhow::Result<()> {
        use std::sync::mpsc;
        use std::thread;
        use crate::addressbook_capnp::{address_book, person};

        let book = addressbook::build_address_book();

        let (tx_book, rx_book) = mpsc::channel::<TypedReader<Builder<HeapAllocator>, address_book::Owned>>();
        let (tx_id, rx_id) = mpsc::channel::<u32>();

        thread::spawn(move || {
            let addressbook_reader = rx_book.recv().unwrap();
            let addressbook = addressbook_reader.get().unwrap();
            let first_person = addressbook.get_people().unwrap().get(0);
            let first_id = first_person.get_id();
            tx_id.send(first_id)
        });

        tx_book.send(book).unwrap();
        let first_id = rx_id.recv().unwrap();
        assert_eq!(first_id, 123);

        Ok(())
    }
}



