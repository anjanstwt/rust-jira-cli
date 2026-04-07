use crate::{
    db::{Epic, Status, Story},
    utils::IO,
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");

    let name = IO::input(Some("Enter the name:"), "name is not provided");
    let description = IO::input(
        Some("Enter the description:"),
        "description is not provided",
    );

    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!("----------------------------");

    let name = IO::input(Some("Enter the name:"), "name is not provided");
    let description = IO::input(
        Some("Enter the description:"),
        "description is not provided",
    );

    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");

    let input = IO::input(
        Some("Deleting this epic will also delete it's corresponding stories [Y/N]:"),
        "Invalid input",
    );

    if input.trim().eq("Y") {
        return true;
    }

    false
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");

    let input = IO::input(
        Some("Are you sure you want to delete this story [Y/N]:"),
        "Invalid input",
    );

    if input.trim().eq("Y") {
        return true;
    }

    false
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");

    let input = IO::input(
        Some("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): "),
        "invalid input",
    );

    let status = input.parse::<u8>();

    if let Ok(status) = status {
        match status {
            1 => return Some(Status::Open),
            2 => return Some(Status::InProgress),
            3 => return Some(Status::Resolved),
            4 => return Some(Status::Closed),
            _ => return None,
        }
    }
    None
}
