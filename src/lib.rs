//! SigmaOS Screen Recorder
//! Screen recording with multiple formats and state machine
//!
//! Principles:
//! - COMPONENT ISOLATION: Standalone screen recording
//! - CQS: Command-Query Separation
//! - FAIL-FAST: State validation

pub mod recorder;

// Re-export key interfaces
pub use recorder::ScreenRecorder;
pub use recorder::RecordConfig;
pub use recorder::RecorderState;
