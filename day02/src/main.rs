mod noise_fn;

use self::noise_fn::NoiseFunctions;
use macroquad::{color::hsl_to_rgb, prelude::*};

fn draw_sized_square(x: f32, y: f32, size: f32, color: Color) {
    draw_rectangle(x - (size / 2.0), y - (size / 2.0), size, size, color);
}

#[macroquad::main("Something that Ages")]
async fn main() {
    println!("Hello!");

    let mut noise = NoiseFunctions::default();
    let mut age = 0.0;
    let mut space_down = false;

    loop {
        // Exit on Esc
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // Change noise type
        if is_key_down(KeyCode::Space) {
            if !space_down {
                noise.select(noise.selected().next());
            }

            space_down = true;
        } else {
            space_down = false;
        }

        // Reset age
        if is_key_down(KeyCode::R) {
            age = 0.0;
        }

        clear_background(WHITE);

        // Draw noise type label
        draw_text(
            &format!("{:?}", noise.selected())[..],
            20.0,
            40.0,
            30.0,
            GRAY,
        );

        // Draw squares
        for x in 0..20 {
            for y in 0..20 {
                // Noise value
                let nv = noise.get_3d(x as f64 / 20.0, y as f64 / 20.0, age).abs() as f32;
                // let nv = 1.0;

                // Color generated from noise value
                let color = hsl_to_rgb(nv + (age / 2.0) as f32, 0.5, 0.5);

                draw_sized_square(
                    (x * 22) as f32 + (screen_width() / 2.0 - 220.0) + ((nv * 10.0).sin()),
                    (y * 22) as f32 + (screen_height() / 2.0 - 220.0) + ((nv * 10.0).cos()),
                    nv * 100.0,
                    color,
                );
            }
        }

        age += 0.005;

        next_frame().await;
    }

    println!("Goodbye.");
}
