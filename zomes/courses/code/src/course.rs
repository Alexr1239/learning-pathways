use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Course {
    title: String,
    teacher_address: Address,
    modules: Vec<Address>, // Implicit link, as relationship with module
    timestamp: u64,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is a course definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data {
                // Replace ".." with "entry, .."
                EntryValidationData::Create { entry,.. } => {
                    // Homework: add a validation rule that the title can only contain 50 chars or less
                    if entry.title.len() <= 50 {
                        Ok(())
                    }
                    else {
                        Err("Test".into())
                    }

                },
                _ => Ok(())
            }
        }
    )
}

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let teacher_address = AGENT_ADDRESS.clone();
    // Homework: finish the create course zome call
    let entry = Course {
        title,
        teacher_address,
        modules: Vec::new(),
        timestamp,
    };
    let entry = Entry::App("course".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get_course(course_address: Address) -> ZomeApiResult<Option<Entry>> {
    // Homework: finish the get course call
    // Hint: use hdk::get_entry
    hdk::get_entry(&course_address)
}

pub fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
    // Homework: finish the delete course call
    // Hint: use hdk::remove_entry
    hdk::remove_entry(&course_address)
}
