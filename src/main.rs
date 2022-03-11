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

use color::Color;
use framebuffer::FrameBuffer;

fn main() {
    let mut frame = FrameBuffer::new(600, 480);
    let mut out = stdout();
    let mut time: f32 = 0.0;
    loop {
        {
            frame.fill(Color::new(0.5, 0.5, 0.5, 1.0));
            let x = (time / 50.0)
                .sin()
                .mul_add((frame.width as f32) / 2.0, (frame.width as f32) / 2.0)
                as u16;
            let y = (time / 50.0)
                .cos()
                .mul_add((frame.height as f32) / 2.0, (frame.height as f32) / 2.0)
                as u16;
            for offset in 0..20 {
                frame.pixel(
                    x + offset - 10,
                    y + offset - 10,
                    Color::new(0.0, 0.0, 0.0, 1.0),
                );
            }
        }
        frame.write(&mut out);
        time += 1.0;
    }
}
