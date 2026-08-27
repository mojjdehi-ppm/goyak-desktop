use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

/// Extra silence kept after the dubbed audio should have finished, to cover the
/// render endpoint's own buffering before the loopback tap goes quiet again.
const TAIL_MS: u64 = 250;

/// Guards against the app dubbing its own output.
///
/// Whole-device loopback taps the default render endpoint, which is also where
/// the dubbed audio is played. Without a gate the translated speech is captured
/// straight back, sent to Gemini, translated again, and played again - the model
/// never gets past that first utterance because it is permanently busy with its
/// own echo.
///
/// Per-process loopback cannot pick up Goyak's own output, and neither can
/// whole-device loopback when the dub is played to a *different* device, so in
/// both of those cases the gate is off and capture runs full duplex.
pub struct FeedbackGate {
    start: Instant,
    /// Milliseconds since `start` at which the queued dubbed audio runs out.
    speaking_until_ms: AtomicU64,
    enabled: AtomicBool,
}

impl FeedbackGate {
    pub fn new(enabled: bool) -> Self {
        Self {
            start: Instant::now(),
            speaking_until_ms: AtomicU64::new(0),
            enabled: AtomicBool::new(enabled),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// The playback thread turns the gate off once it knows the dub is going to
    /// a device other than the one being captured.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
        if !enabled {
            self.speaking_until_ms.store(0, Ordering::Relaxed);
        }
    }

    fn now_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    /// Called by the playback thread whenever the amount of queued dubbed audio
    /// changes. `queued_ms` is how much is still waiting to reach the speakers;
    /// pass 0 when the queue is emptied or output is muted.
    pub fn set_queued(&self, queued_ms: u64) {
        if !self.is_enabled() {
            return;
        }
        let until = if queued_ms == 0 {
            0
        } else {
            self.now_ms().saturating_add(queued_ms)
        };
        self.speaking_until_ms.store(until, Ordering::Relaxed);
    }

    /// True while our own dubbed audio is still going out of the speakers, so
    /// the capture thread must not forward what it hears to the model.
    pub fn should_suppress_capture(&self) -> bool {
        if !self.is_enabled() {
            return false;
        }
        let until = self.speaking_until_ms.load(Ordering::Relaxed);
        until != 0 && self.now_ms() < until.saturating_add(TAIL_MS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_gate_does_not_suppress() {
        assert!(!FeedbackGate::new(true).should_suppress_capture());
    }

    #[test]
    fn suppresses_while_dubbed_audio_is_queued() {
        let gate = FeedbackGate::new(true);
        gate.set_queued(5_000);
        assert!(gate.should_suppress_capture());
    }

    #[test]
    fn stops_suppressing_once_the_queue_is_emptied() {
        let gate = FeedbackGate::new(true);
        gate.set_queued(5_000);
        gate.set_queued(0);
        assert!(!gate.should_suppress_capture());
    }

    /// Process loopback cannot hear our own output, so it must never be gated.
    #[test]
    fn a_disabled_gate_never_suppresses() {
        let gate = FeedbackGate::new(false);
        gate.set_queued(5_000);
        assert!(!gate.should_suppress_capture());
    }

    #[test]
    fn turning_the_gate_off_releases_capture_immediately() {
        let gate = FeedbackGate::new(true);
        gate.set_queued(5_000);
        gate.set_enabled(false);
        assert!(!gate.should_suppress_capture());
    }
}
