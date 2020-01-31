#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk::prelude::*;

use hdk_proc_macros::zome;

pub mod course;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry


#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn course_definition() -> ValidatingEntryType {
        course::entry_definition()
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        course::create(title, timestamp)
    }

    // Homework: add the get_course and delete_course endpoints
    #[zome_fn("hc_public")]
    fn get_course(course_address: Address) -> ZomeApiResult<Option<Entry>> {
        course::get_course(course_address)
    }

    #[zome_fn("hc_public")]
    fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
        course::delete_course(course_address)
    }

}
