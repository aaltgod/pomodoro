use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub activity: String,
}

pub struct Storage {
    database_url: String,
}

impl Storage {
    pub fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        
        let _f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(database_url.to_string())?;

        Ok(
            Self {
                database_url: database_url.to_string(),
            })
    }

    pub fn insert(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        
        let f = std::fs::OpenOptions::new()
            .write(true)
            .open(&self.database_url)?;

        let seriliazed = serde_json::to_string(user)?;

        serde_json::to_writer_pretty(f, &seriliazed)?;
        Ok(())
    }

    pub fn get(&self) -> Result<User, Box<dyn std::error::Error>> {
          
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(&self.database_url)?;

        let data: String = match serde_json::from_reader(f) {
            Ok(data) => data,
            Err(e) => return Err(Box::new(e)),
        };

        let deserialized: User = match serde_json::from_str(&data) {
            Ok(deserialized) => deserialized,
            Err(e) => return Err(Box::new(e)),
        };

        Ok( deserialized )
    }

    pub fn update(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        self.insert(user)?;
        Ok(())
    }
}

