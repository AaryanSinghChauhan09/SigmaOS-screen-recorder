#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecordingState { #[default] Idle, Recording, Paused, Stopped }

#[derive(Debug, Default)]
pub struct ScreenRecorderEngine {
    state: RecordingState,
}

impl ScreenRecorderEngine {
    pub fn new() -> Self { Self { state: RecordingState::Idle } }
    pub fn start(&mut self) { self.state = RecordingState::Recording; }
    pub fn stop(&mut self) { self.state = RecordingState::Stopped; }
    pub fn get_state(&self) -> RecordingState { self.state }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_screen_recorder_state_flow() {
        let mut rec = ScreenRecorderEngine::new();
        assert_eq!(rec.get_state(), RecordingState::Idle);
        rec.start();
        assert_eq!(rec.get_state(), RecordingState::Recording);
        rec.stop();
        assert_eq!(rec.get_state(), RecordingState::Stopped);
    }
}
