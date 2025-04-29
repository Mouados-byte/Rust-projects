use std::{fmt::Error, io::{self, empty}};

struct Contact {
    name: String,
    phone: u32,
    email: String
}

struct ContactManager {
    contacts: Vec<Contact>
}

impl ContactManager {
    fn new() -> Self {
        Self { contacts: Vec::new() }
    }

    fn add_contact(&mut self, name: &str, email: &str, phone: u32) {
        let contact = Contact { name: name.to_string(), phone, email: email.to_string() };
        self.contacts.push(contact)
    }

    fn list_contacts(&self){
        if self.contacts.is_empty() {
            println!("No contacts!");
        }
        println!("-----------------------");
        for i in &self.contacts{
            println!("name: {}", i.name);
            println!("email: {}", i.email);
            println!("phone: {}", i.phone.to_string());
        }
    }

    fn find_by_name(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.name.contains(name))
    }

    fn update_phone(& mut self, name: &str, phone: u32) -> Result<&Contact, String> {
        for contact in &mut self.contacts {
            if contact.name == name {
                contact.phone = phone;
                return Ok(contact);
            }
        }
        Err("Daamn , not updated".to_string())
    }


}

fn main() {
    let mut manager = ContactManager::new();
    
    // Add some contacts
    manager.add_contact(
        "Alice", 
        "alice@example.com", 
        213498712
    );
    
    manager.add_contact(
        "Bob", 
        "bob@example.com", 
        213498712
    );
    
    // List all contacts
    println!("All contacts:");
    manager.list_contacts();
    
    // Find a contact
    if let Some(contact) = manager.find_by_name("Alice") {
        println!("\nFound contact:");
        println!("  Name: {}", contact.name);
        println!("  Email: {}", contact.email);
        println!("  Phone: {}", contact.phone);
    }
    
    // Update a contact
    match manager.update_phone("Bob", 41234134) {
        Ok(_) => {
            println!("\nUpdated Bob's phone number");
        },
        Err(e) => {
            println!("{e}")
        }
    }
    
    // List all contacts again
    println!("\nAll contacts after update:");
    manager.list_contacts();
}