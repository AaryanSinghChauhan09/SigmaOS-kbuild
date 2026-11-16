// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kbuild: kbuild component model.
//!
//! Real implementation: kbuild_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KbuildItem {
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
pub enum KbuildError {
    EmptyKbuildItemName,
    DuplicateKbuildItem(String),
    EmptyEventSubject,
}

impl fmt::Display for KbuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKbuildItemName => write!(f, "kbuild_item name must not be empty"),
            Self::DuplicateKbuildItem(id) => write!(f, "duplicate kbuild_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KbuildError {}

pub struct Kbuild {
    pub name: String,
    kbuild_items: BTreeMap<String, KbuildItem>,
    events: BTreeMap<String, Event>,
}

impl Kbuild {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kbuild_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kbuild_item(&mut self, item: KbuildItem) -> Result<(), KbuildError> {
        if item.name.trim().is_empty() {
            return Err(KbuildError::EmptyKbuildItemName);
        }
        let id = item.id.clone();
        if self.kbuild_items.contains_key(&id) {
            return Err(KbuildError::DuplicateKbuildItem(id.clone()));
        }
        self.kbuild_items.insert(id, item);
        Ok(())
    }

    pub fn kbuild_item_count(&self) -> usize {
        self.kbuild_items.len()
    }

    pub fn get_kbuild_item(&self, id: &str) -> Option<&KbuildItem> {
        self.kbuild_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KbuildError> {
        if event.subject.trim().is_empty() {
            return Err(KbuildError::EmptyEventSubject);
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

    fn sample_kbuild_item(id: &str, name: &str) -> KbuildItem {
        KbuildItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Kbuild::new("Kbuild");
        assert_eq!(m.name, "Kbuild");
        assert_eq!(m.kbuild_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kbuild_item() {
        let mut m = Kbuild::new("Kbuild");
        m.add_kbuild_item(sample_kbuild_item("a1", "Alpha")).unwrap();
        assert_eq!(m.kbuild_item_count(), 1);
        assert!(m.get_kbuild_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Kbuild::new("Kbuild");
        assert!(m.add_kbuild_item(sample_kbuild_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Kbuild::new("Kbuild");
        m.add_kbuild_item(sample_kbuild_item("x", "One")).unwrap();
        assert!(m.add_kbuild_item(sample_kbuild_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Kbuild::new("Kbuild");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
