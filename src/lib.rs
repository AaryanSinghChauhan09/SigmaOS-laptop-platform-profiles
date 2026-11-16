// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-laptop-platform-profiles: laptop platform profiles component model.
//!
//! Real implementation: profiless, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profiles {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaptopPlatformProfilesError {
    EmptyProfilesName,
    DuplicateProfiles(String),
    EmptyEventSubject,
}

impl fmt::Display for LaptopPlatformProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProfilesName => write!(f, "profiles name must not be empty"),
            Self::DuplicateProfiles(id) => write!(f, "duplicate profiles {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LaptopPlatformProfilesError {}

pub struct LaptopPlatformProfiles {
    pub name: String,
    profiless: BTreeMap<String, Profiles>,
    events: BTreeMap<String, Event>,
}

impl LaptopPlatformProfiles {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            profiless: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_profiles(&mut self, item: Profiles) -> Result<(), LaptopPlatformProfilesError> {
        if item.name.trim().is_empty() {
            return Err(LaptopPlatformProfilesError::EmptyProfilesName);
        }
        let id = item.id.clone();
        if self.profiless.contains_key(&id) {
            return Err(LaptopPlatformProfilesError::DuplicateProfiles(id.clone()));
        }
        self.profiless.insert(id, item);
        Ok(())
    }

    pub fn profiles_count(&self) -> usize {
        self.profiless.len()
    }

    pub fn get_profiles(&self, id: &str) -> Option<&Profiles> {
        self.profiless.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LaptopPlatformProfilesError> {
        if event.subject.trim().is_empty() {
            return Err(LaptopPlatformProfilesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profiles(id: &str, name: &str) -> Profiles {
        Profiles {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LaptopPlatformProfiles::new("LaptopPlatformProfiles");
        assert_eq!(m.name, "LaptopPlatformProfiles");
        assert_eq!(m.profiles_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_profiles() {
        let mut m = LaptopPlatformProfiles::new("LaptopPlatformProfiles");
        m.add_profiles(sample_profiles("a1", "Alpha")).unwrap();
        assert_eq!(m.profiles_count(), 1);
        assert!(m.get_profiles("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LaptopPlatformProfiles::new("LaptopPlatformProfiles");
        assert!(m.add_profiles(sample_profiles("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LaptopPlatformProfiles::new("LaptopPlatformProfiles");
        m.add_profiles(sample_profiles("x", "One")).unwrap();
        assert!(m.add_profiles(sample_profiles("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LaptopPlatformProfiles::new("LaptopPlatformProfiles");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
