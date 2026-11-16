// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-toqa: toqa component model.
//!
//! Real implementation: toqa_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToqaItem {
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
pub enum ToqaError {
    EmptyToqaItemName,
    DuplicateToqaItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ToqaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyToqaItemName => write!(f, "toqa_item name must not be empty"),
            Self::DuplicateToqaItem(id) => write!(f, "duplicate toqa_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ToqaError {}

pub struct Toqa {
    pub name: String,
    toqa_items: BTreeMap<String, ToqaItem>,
    events: BTreeMap<String, Event>,
}

impl Toqa {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            toqa_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_toqa_item(&mut self, item: ToqaItem) -> Result<(), ToqaError> {
        if item.name.trim().is_empty() {
            return Err(ToqaError::EmptyToqaItemName);
        }
        let id = item.id.clone();
        if self.toqa_items.contains_key(&id) {
            return Err(ToqaError::DuplicateToqaItem(id.clone()));
        }
        self.toqa_items.insert(id, item);
        Ok(())
    }

    pub fn toqa_item_count(&self) -> usize {
        self.toqa_items.len()
    }

    pub fn get_toqa_item(&self, id: &str) -> Option<&ToqaItem> {
        self.toqa_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ToqaError> {
        if event.subject.trim().is_empty() {
            return Err(ToqaError::EmptyEventSubject);
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

    fn sample_toqa_item(id: &str, name: &str) -> ToqaItem {
        ToqaItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Toqa::new("Toqa");
        assert_eq!(m.name, "Toqa");
        assert_eq!(m.toqa_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_toqa_item() {
        let mut m = Toqa::new("Toqa");
        m.add_toqa_item(sample_toqa_item("a1", "Alpha")).unwrap();
        assert_eq!(m.toqa_item_count(), 1);
        assert!(m.get_toqa_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Toqa::new("Toqa");
        assert!(m.add_toqa_item(sample_toqa_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Toqa::new("Toqa");
        m.add_toqa_item(sample_toqa_item("x", "One")).unwrap();
        assert!(m.add_toqa_item(sample_toqa_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Toqa::new("Toqa");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
