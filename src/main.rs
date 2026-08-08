use macroquad::math::Vec2;
use macroquad::prelude::*;

fn window() -> Conf {
    Conf {
        window_title: "A Graph".to_string(),
        window_width: 800,
        window_height: 600,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1), // 0: unlimited, 1: 60fps, 2: 30fps
            ..Default::default()
        },
        ..Default::default()
    }
}

//-------------Main Loop--------------------------------------------------------
#[macroquad::main(window)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_axis();
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await;
    }
}

fn draw_axis() {
    //I am going to do arithmetic in terms of percentile.
    //So I can simply multiply by window size.

    draw_rectangle(
        0.1 * screen_width(),
        0.1 * screen_height(),
        0.01 * screen_width(),
        0.8 * screen_height(),
        YELLOW,
    );

    draw_rectangle(
        0.1 * screen_width(),
        0.9 * screen_height(),
        0.8 * screen_width(),
        0.01 * screen_height(),
        YELLOW,
    );

    let mut text = "X-Axis";
    let color = YELLOW;
    let mut center_offset = get_text_center(text, None, 30, 1.0, 0.0);
    let rotation = -90.0f32.to_radians();

    draw_text(
        text,
        (screen_width() / 2.0) - center_offset.x,
        screen_height() * 0.95,
        30.0,
        color,
    );

    text = "Y-Axis";
    center_offset = get_text_center(text, None, 30, 1.0, rotation);

    draw_text_ex(
        text,
        screen_width() * 0.05,
        (screen_height() / 2.0) - center_offset.y,
        TextParams {
            font_size: 30,
            rotation,
            color,
            ..Default::default()
        },
    );
}
