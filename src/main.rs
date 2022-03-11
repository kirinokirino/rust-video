#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::multiple_inherent_impl,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads
)]
#![allow(clippy::cast_precision_loss, clippy::missing_panics_doc)]

mod color;
mod common;
mod framebuffer;

use std::io::stdout;
use std::time::{Duration, Instant};

use color::Color;
use framebuffer::{write, FrameBuffer};

fn main() {
    let mut frame = FrameBuffer::new(1440, 900);
    let mut out = stdout();
    let overall = Instant::now();
    let mut period: Period = Period::new(State::First, overall.elapsed());
    loop {
        let time = overall.elapsed().as_secs_f32();
        period = period.check(overall.elapsed());
        match period.state {
            State::First => {
                //frame.fill(Color::new(0.5, 0.5, 0.5, 1.0));
                let x = time
                    .sin()
                    .mul_add((frame.width as f32) / 2.0, (frame.width as f32) / 2.0)
                    as u16;
                let y = time
                    .cos()
                    .mul_add((frame.height as f32) / 2.0, (frame.height as f32) / 2.0)
                    as u16;
                for offset in 0..20 {
                    frame.pixel(
                        x + offset - 10,
                        y + offset - 10,
                        Color::new(1.0, 1.0, 1.0, 1.0),
                    );
                }
            }
            State::Second => {}
        }
        write(&frame, &mut out);
    }
}

struct Period {
    state: State,
    start_time: Duration,
    duration: Duration,
}

impl Period {
    pub fn new(state: State, start_time: Duration) -> Self {
        match state {
            State::First => Self {
                state,
                start_time,
                duration: Duration::new(2, 0),
            },
            State::Second => Self {
                state,
                start_time,
                duration: Duration::new(1, 0),
            },
        }
    }

    pub fn check(self, time: Duration) -> Self {
        if time >= self.start_time + self.duration {
            self.finish(time)
        } else {
            self
        }
    }

    pub fn finish(self, time: Duration) -> Self {
        match self.state {
            State::First => Self::new(State::Second, time),
            State::Second => Self::new(State::First, time),
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    First,
    Second,
}

impl State {
    pub const fn next(&self) -> Self {
        match *self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}
