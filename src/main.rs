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
mod framebuffer;
mod render;

use std::io::stdout;

use color::{Color, RGB};
use framebuffer::{FrameBuffer, WritePPM, PPM};
use render::Renderable;

fn main() {
    let mut frame = FrameBuffer::new(1920, 1080);
    let mut out = stdout();
    let mut time: f32 = 0.0;
    loop {
        {
            let w = frame.w;
            let h = frame.h;
            let hw = w as f32 / 2.0;
            let hh = h as f32 / 2.0;

            let frame = &mut frame as &mut dyn PPM;
            frame.fill(w as usize, h as usize, Color::from_rgb(&1.0, &1.0, &1.0));
            frame.dot(
                (time / 50.0).sin().mul_add(hw, hw),
                (time / 50.0).cos().mul_add(hh, hh),
                0,
                10.0,
                Some(10.0),
            );
            let x = (time / 100.0).cos().mul_add(hw, hw);
            let y = (time / 100.0).sin().mul_add(hh, hh);
            frame.line(
                x,
                y,
                w as f32 - x,
                h as f32 - y,
                Color::from_rgb(&0.9, &0.15, &0.2),
            );
        }
        frame.ppm_write(&mut out);
        time += 1.0;
    }
}
