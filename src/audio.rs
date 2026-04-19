/// Audio playback of Morse code as 700 Hz sine-wave tones.
///
/// Timing (standard ~20 WPM):
///   dot              =  60 ms
///   dash             = 180 ms  (3 × dot)
///   intra-char gap   =  60 ms  (between dots/dashes in one letter)
///   inter-letter gap = 180 ms  (between letters)
///   inter-word gap   = 420 ms  (7 × dot; triggered by " / ")
///
/// A short 10 ms linear ramp-up and ramp-down is applied to every tone
/// to eliminate audible clicks at symbol boundaries.
use rodio::{OutputStream, Sink, Source};
use std::f32::consts::TAU;
use std::time::Duration;

// ── Timing constants ──────────────────────────────────────────────────────────

const SAMPLE_RATE:   u32 = 44_100;
const FREQUENCY:     f32 = 700.0;   // Hz
const AMPLITUDE:     f32 = 0.45;    // 0.0 – 1.0

const DOT_MS:        u64 = 60;
const DASH_MS:       u64 = 180;
const SYMBOL_GAP_MS: u64 = 60;
const LETTER_GAP_MS: u64 = 180;
const WORD_GAP_MS:   u64 = 420;
const RAMP_MS:       u64 = 10;

// ── Public API ────────────────────────────────────────────────────────────────

/// Play `morse` as audio tones, blocking until playback is complete.
///
/// Returns `Ok(())` on success, or an error string if audio is unavailable.
pub fn play(morse: &str) -> Result<(), String> {
    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| format!("Audio error: {e}"))?;

    let sink = Sink::try_new(&stream_handle).map_err(|e| format!("Audio sink error: {e}"))?;

    let words: Vec<&str> = morse.split(" / ").collect();

    for (wi, word) in words.iter().enumerate() {
        if wi > 0 {
            sink.append(silence(WORD_GAP_MS));
        }
        let letters: Vec<&str> = word.trim().split(' ').filter(|s| !s.is_empty()).collect();
        for (li, code) in letters.iter().enumerate() {
            if li > 0 {
                sink.append(silence(LETTER_GAP_MS));
            }
            for (si, sym) in code.chars().enumerate() {
                if si > 0 {
                    sink.append(silence(SYMBOL_GAP_MS));
                }
                match sym {
                    '.' => sink.append(tone(DOT_MS)),
                    '-' => sink.append(tone(DASH_MS)),
                    _   => {}
                }
            }
        }
    }

    sink.sleep_until_end();
    Ok(())
}

// ── Sample generators ─────────────────────────────────────────────────────────

/// A sine-wave tone of the given duration with ramp envelope.
fn tone(duration_ms: u64) -> impl Source<Item = f32> {
    SineWave::new(duration_ms)
}

/// Silent (zero-amplitude) source of the given duration.
fn silence(duration_ms: u64) -> impl Source<Item = f32> {
    rodio::source::Zero::new(1, SAMPLE_RATE)
        .take_duration(Duration::from_millis(duration_ms))
}

// ── SineWave source ───────────────────────────────────────────────────────────

struct SineWave {
    sample_idx:  u64,
    total_samples: u64,
    ramp_samples:  u64,
}

impl SineWave {
    fn new(duration_ms: u64) -> Self {
        let total_samples = SAMPLE_RATE as u64 * duration_ms / 1_000;
        let ramp_samples  = SAMPLE_RATE as u64 * RAMP_MS / 1_000;
        Self { sample_idx: 0, total_samples, ramp_samples }
    }

    /// Linear ramp envelope: fade in/out over `ramp_samples` to prevent clicks.
    fn envelope(&self) -> f32 {
        let i = self.sample_idx;
        let n = self.total_samples;
        let r = self.ramp_samples;
        if i < r {
            i as f32 / r as f32
        } else if i > n.saturating_sub(r) {
            (n - i) as f32 / r as f32
        } else {
            1.0
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.sample_idx >= self.total_samples {
            return None;
        }
        let t = self.sample_idx as f32 / SAMPLE_RATE as f32;
        let sample = AMPLITUDE * self.envelope() * (TAU * FREQUENCY * t).sin();
        self.sample_idx += 1;
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self)          -> u16 { 1 }
    fn sample_rate(&self)       -> u32 { SAMPLE_RATE }
    fn total_duration(&self)    -> Option<Duration> {
        Some(Duration::from_millis(
            self.total_samples * 1_000 / SAMPLE_RATE as u64,
        ))
    }
}
