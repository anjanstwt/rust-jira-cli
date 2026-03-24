use crate::db::{DBState, Database, Epic, JSONFileDatabase, Status, Story};
use anyhow::{Result, anyhow};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_file()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        // read the json file and get the current state of data
        let mut parsed = self.database.read_file()?;

        // change the data (last_item_id, epic)
        let item_id = parsed.last_item_id + 1;
        parsed.last_item_id = item_id;
        parsed.epics.insert(item_id, epic);

        self.database.write_file(&parsed)?;

        Ok(item_id)
    }

    pub fn update_epic_status(&self, id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_file()?;

        parsed
            .epics
            .get_mut(&id)
            .ok_or_else(|| anyhow!("Cannot find epic with this id"))?
            .status = status;

        self.database.write_file(&parsed)?;
        Ok(())
    }

    pub fn delete_epic(&self, id: u32) -> Result<()> {
        let mut parsed = self.database.read_file()?;

        // delete the stories
        let stories = parsed
            .epics
            .get(&id)
            .ok_or_else(|| anyhow!("Cannot find epic with this id"))?
            .stories
            .clone();

        stories.iter().for_each(|i| {
            parsed.stories.remove(&i);
        });

        parsed.epics.remove(&id);
        self.database.write_file(&parsed)?;

        Ok(())
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_file()?;

        let item_id = parsed.last_item_id + 1;
        parsed.last_item_id = item_id;
        parsed.stories.insert(item_id, story);
        parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Cannot find epic with this id"))?
            .stories
            .push(item_id);

        self.database.write_file(&parsed)?;
        Ok(item_id)
    }

    pub fn update_story(&self, id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_file()?;

        parsed
            .stories
            .get_mut(&id)
            .ok_or_else(|| anyhow!("Cannot find story with this id"))?
            .status = status;

        self.database.write_file(&parsed)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut parsed = self.database.read_file()?;

        let epic = parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Cannot find epic with this id"))?;

        let story_index = epic
            .stories
            .iter()
            .position(|id| id == &story_id)
            .ok_or_else(|| anyhow!("Cannot find story with this id"))?;

        epic.stories.remove(story_index);
        parsed.stories.remove(&story_id);

        self.database.write_file(&parsed)?;
        Ok(())
    }
}
