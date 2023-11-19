use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
pub struct Location {
    id: String,
    longitude: f64,
    latitude: f64,
    user_id: String,
}

#[derive(Deserialize)]
pub struct LocationSave {
    longitude: f64,
    latitude: f64,
}

#[derive(Clone, Deserialize)]
pub struct LocationPoint {
    longitude: f64,
    latitude: f64,
}

#[derive(Clone, Deserialize)]
pub struct LocationBatchSave {
    locations: Vec<LocationPoint>,
}

#[derive(Clone)]
pub struct LocationModelController {
    locations: Arc<Mutex<Vec<Option<Location>>>>,
}

impl LocationModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            locations: Arc::default(),
        })
    }

    pub async fn list_user_locations(self, user_id: String) -> Result<Vec<Location>> {
        let store = self.locations.lock().unwrap();

        let user_locations: Vec<Location> = store
            .iter()
            .filter_map(|loc| {
                loc.as_ref()
                    .filter(|&current| current.user_id == user_id.clone())
                    .cloned()
            })
            .collect();

        Ok(user_locations)
    }

    pub async fn save_location(self, payload: LocationSave, user_id: String) -> Result<Location> {
        let mut store = self.locations.lock().unwrap();

        let location = Location {
            id: uuid7::uuid7().to_string(),
            longitude: payload.longitude,
            latitude: payload.latitude,
            user_id: user_id,
        };

        store.push(Some(location.clone()));

        Ok(location)
    }

    pub async fn save_batch_locations(
        self,
        payload: LocationBatchSave,
        user_id: String,
    ) -> Result<()> {
        let mut store = self.locations.lock().unwrap();

        for i in 0..payload.locations.len() {
            store.push(Some(Location {
                id: uuid7::uuid7().to_string(),
                longitude: payload.locations[i].longitude,
                latitude: payload.locations[i].latitude,
                user_id: user_id.clone(),
            }))
        }

        Ok(())
    }
}
