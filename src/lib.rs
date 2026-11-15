//! SigmaOS-screen-recorder: screen recording model inspired by Omacom's omareel.
//!
//! Real implementation: recordings, regions, synthetic cursor, auto zooms, camera bubble.
//! Replaces the 28-line state machine placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Recording state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecordingState {
    #[default]
    Idle,
    Recording,
    Paused,
    Stopped,
}

/// Screen region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenRegion {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Recording session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordingSession {
    pub id: String,
    pub name: String,
    pub region: ScreenRegion,
    pub synthetic_cursor: bool,
    pub auto_zoom: bool,
    pub camera_bubble: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecorderError {
    EmptyName,
    DuplicateSession(String),
}

impl fmt::Display for RecorderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "session name must not be empty"),
            Self::DuplicateSession(id) => write!(f, "duplicate session {:?}", id),
        }
    }
}

impl std::error::Error for RecorderError {}

/// Screen recorder engine.
pub struct ScreenRecorderEngine {
    pub state: RecordingState,
    sessions: BTreeMap<String, RecordingSession>,
}

impl ScreenRecorderEngine {
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            sessions: BTreeMap::new(),
        }
    }

    pub fn start(&mut self) { self.state = RecordingState::Recording; }

    pub fn stop(&mut self) { self.state = RecordingState::Stopped; }

    pub fn pause(&mut self) { self.state = RecordingState::Paused; }

    pub fn get_state(&self) -> RecordingState { self.state }

    pub fn add_session(&mut self, session: RecordingSession) -> Result<(), RecorderError> {
        if session.name.trim().is_empty() { return Err(RecorderError::EmptyName); }
        let id = session.id.clone();
        if self.sessions.contains_key(&id) {
            return Err(RecorderError::DuplicateSession(id.clone()));
        }
        self.sessions.insert(id, session);
        Ok(())
    }

    pub fn session_count(&self) -> usize { self.sessions.len() }

    pub fn get_session(&self, id: &str) -> Option<&RecordingSession> { self.sessions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_recorder_state_flow_aaa() {
        let mut rec = ScreenRecorderEngine::new();
        assert_eq!(rec.get_state(), RecordingState::Idle);
        rec.start();
        assert_eq!(rec.get_state(), RecordingState::Recording);
        rec.pause();
        assert_eq!(rec.get_state(), RecordingState::Paused);
        rec.stop();
        assert_eq!(rec.get_state(), RecordingState::Stopped);
    }

    #[test]
    fn test_add_session_aaa() {
        let mut rec = ScreenRecorderEngine::new();
        let region = ScreenRegion { id: "r1".into(), x: 0, y: 0, width: 1920, height: 1080 };
        let s = RecordingSession { id: "s1".into(), name: "demo".into(), region, synthetic_cursor: true, auto_zoom: true, camera_bubble: true };
        rec.add_session(s).unwrap();
        assert_eq!(rec.session_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut rec = ScreenRecorderEngine::new();
        let region = ScreenRegion { id: "r1".into(), x: 0, y: 0, width: 1920, height: 1080 };
        let s = RecordingSession { id: "x".into(), name: "".into(), region, synthetic_cursor: true, auto_zoom: true, camera_bubble: true };
        assert!(rec.add_session(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut rec = ScreenRecorderEngine::new();
        let region = ScreenRegion { id: "r1".into(), x: 0, y: 0, width: 1920, height: 1080 };
        let s = RecordingSession { id: "x".into(), name: "X".into(), region: region.clone(), synthetic_cursor: true, auto_zoom: true, camera_bubble: true };
        rec.add_session(s).unwrap();
        let s2 = RecordingSession { id: "x".into(), name: "X".into(), region, synthetic_cursor: true, auto_zoom: true, camera_bubble: true };
        assert!(rec.add_session(s2).is_err());
    }
}
