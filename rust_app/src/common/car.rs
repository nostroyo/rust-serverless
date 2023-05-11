use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub pk: Uuid,
    pub name: String,
    pub agility: u16,
    pub power: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    pub pk: Uuid,
    pub driver: Option<Driver>,
}

impl Car {
    pub fn new(driver: Option<Driver>) -> Self {
        Car {
            pk: Uuid::new_v4(),
            driver,
        }
    }
}

impl Driver {
    pub fn new(name: &str) -> Self {
        Driver {
            pk: Uuid::new_v4(),
            name: name.to_string(),
            agility: 0,
            power: 0,
        }
    }
}
