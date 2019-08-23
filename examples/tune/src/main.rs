#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate monotron_app;

use monotron_app::{Channel, Host, Note, Waveform};

const FRAMES_PER_BAR: usize = 128;
const MAX_VOLUME: u8 = 255;

#[cfg(not(target_os = "none"))]
pub fn main() {
    Host::init();
    play_tune();
    Host::deinit();
    std::process::exit(0);
}

#[no_mangle]
#[cfg(target_os = "none")]
pub extern "C" fn monotron_main() -> i32 {
    play_tune();
    0
}

#[derive(Debug)]
enum Length {
    Whole,
    Half,
    Quarter,
    DottedQuarter,
    Eighth,
    Sixteenth,
}

struct Track<'a> {
    channel: Channel,
    waveform: Waveform,
    volume: u8,
    play_idx: usize,
    play_next_at: usize,
    notes: &'a [(Option<Note>, Length)],
}

fn play_tune() {
    Host::puts(b"Press Ctrl+C to exit.");

    let mut track0 = Track {
        channel: Channel::Channel0,
        play_idx: 0,
        play_next_at: 0,
        waveform: Waveform::Square,
        volume: MAX_VOLUME / 2,
        notes: &[
            // Bar 1
            (Some(Note::E5), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::E5), Length::Sixteenth),
            (Some(Note::D5), Length::Sixteenth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            // Bar 2
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::E5), Length::Quarter),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            // Bar 3
            (Some(Note::B4), Length::DottedQuarter),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Quarter),
            (Some(Note::E5), Length::Quarter),
            // Bar 4
            (Some(Note::C5), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Half),
            // Bar 5
            (Some(Note::D5), Length::Quarter),
            (Some(Note::F5), Length::Eighth),
            (Some(Note::A5), Length::Quarter),
            (Some(Note::G5), Length::Eighth),
            (Some(Note::F5), Length::Eighth),
            // Bar 6
            (Some(Note::E5), Length::DottedQuarter),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::E5), Length::Quarter),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            // Bar 7
            (Some(Note::B4), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Quarter),
            (Some(Note::E5), Length::Quarter),
            // Bar 8
            (Some(Note::C5), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut track1 = Track {
        channel: Channel::Channel1,
        play_idx: 0,
        waveform: Waveform::Square,
        volume: MAX_VOLUME / 2,
        play_next_at: 0,
        notes: &[
            (Some(Note::B4), Length::Quarter),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            (None, Length::Sixteenth),
            (None, Length::Sixteenth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            // Bar 2
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            // Bar 3
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Quarter),
            (Some(Note::C4), Length::Quarter),
            // Bar 4
            (Some(Note::A4), Length::Quarter),
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Half),
            // Bar 5
            (Some(Note::F4), Length::Quarter),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::C5), Length::Sixteenth),
            (Some(Note::C5), Length::Sixteenth),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            // Bar 6
            (Some(Note::G4), Length::DottedQuarter),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            (Some(Note::A4), Length::Sixteenth),
            (Some(Note::G4), Length::Sixteenth),
            (Some(Note::F4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            // Bar 7
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            // Bar 8
            (Some(Note::A4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut track2 = Track {
        channel: Channel::Channel2,
        play_idx: 0,
        waveform: Waveform::Sawtooth,
        volume: MAX_VOLUME,
        play_next_at: 0,
        notes: &[
            // Bar 1
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            // Bar 2
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            // Bar 3
            (Some(Note::GsAb2), Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            (Some(Note::GsAb2), Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            // Bar 4
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            // Bar 5
            (Some(Note::D3), Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::F2), Length::Eighth),
            // Bar 6
            (Some(Note::C2), Length::Eighth),
            (Some(Note::C3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::C3), Length::Eighth),
            (Some(Note::C2), Length::Eighth),
            (Some(Note::G2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::G2), Length::Eighth),
            // Bar 7
            (Some(Note::B2), Length::Eighth),
            (Some(Note::B3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::B3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            // Bar 8
            (Some(Note::A2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::A2), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut frame_count = 0;
    loop {
        let mut again = true;
        while again {
            again = false;
            for track in &mut [&mut track0, &mut track1, &mut track2] {
                let (note, length) = &track.notes[track.play_idx];
                if frame_count == track.play_next_at {
                    if let Some(pitch) = note {
                        Host::play(*pitch, track.channel, track.waveform, track.volume);
                    } else {
                        Host::play(Note::C0, track.channel, track.waveform, 0);
                    }
                    track.play_next_at += match length {
                        Length::Whole => FRAMES_PER_BAR,
                        Length::Half => FRAMES_PER_BAR / 2,
                        Length::Quarter => FRAMES_PER_BAR / 4,
                        Length::DottedQuarter => 3 * (FRAMES_PER_BAR / 8),
                        Length::Eighth => FRAMES_PER_BAR / 8,
                        Length::Sixteenth => FRAMES_PER_BAR / 16,
                    };
                    track.play_idx += 1;
                    if track.play_idx >= track.notes.len() {
                        track.play_idx = 0;
                    }
                    again = true;
                }
            }
        }
        frame_count += 1;
        Host::wfvbi();
        if Host::kbhit() {
            let c = Host::readc();
            if c == 3 {
                return;
            }
        }
    }
}
