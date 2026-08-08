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
    let x_range = make_range(0.0, 8.0, 1.0);
    let data_points = linear_function(&x_range);
    let (x_dim, y_dim) = get_dimensions(&data_points);

    for x in x_dim {
        println!("{}", x);
    }
    println!();
    for y in y_dim {
        println!("{}", y);
    }

    loop {
        clear_background(BLACK);
        draw_axis(&data_points);
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await;
    }
}

fn linear_function(range: &[f32]) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = Vec::new();
    for x in range {
        let y = *x;
        points.push(Vec2::new(*x, y));
    }

    points
}

fn get_dimensions(points: &[Vec2]) -> (Vec<f32>, Vec<f32>) {
    let mut x_range = Vec::new();
    let mut y_range = Vec::new();

    for point in points {
        x_range.push(point.x);
        y_range.push(point.y);
    }

    //sus out actual dimensions that these positions give on the body of the graph;
    //find max, divide by max, multiply every element by body width.

    let bodywidth = 0.8 * screen_width();
    let bodyheight = 0.8 * screen_height();
    let x_max = f32max(&x_range);
    let y_max = f32max(&y_range);

    for x in x_range.iter_mut() {
        *x /= x_max;
        *x *= bodywidth;
    }

    for y in y_range.iter_mut() {
        *y /= y_max;
        *y *= bodyheight;
    }

    (x_range, y_range)
}

fn draw_axis(points: &Vec<Vec2>) {
    //I am going to do arithmetic in terms of percentile.
    //So I can simply multiply by window size.

    //y-axis part of frame
    draw_rectangle(
        0.1 * screen_width(),
        0.1 * screen_height(),
        2.0,
        0.8 * screen_height(),
        YELLOW,
    );

    //x-axis part of frame
    draw_rectangle(
        0.1 * screen_width(),
        0.9 * screen_height(),
        0.8 * screen_width(),
        2.0,
        YELLOW,
    );

    text_draw(
        "X-Axis",
        screen_width() / 2.0,
        screen_height() * 0.95,
        None,
        30,
        0.0,
        YELLOW,
    );

    text_draw(
        "Y-Axis",
        screen_width() * 0.05,
        screen_height() / 2.0,
        None,
        30,
        -90.0f32.to_radians(),
        YELLOW,
    );
}

//The x and y target are the target postions for the center of the text box.
fn text_draw(
    text: &str,
    x_target: f32,
    y_target: f32,
    font: Option<&Font>,
    font_size: u16,
    rotation: f32,
    color: Color,
) {
    let center_offset = get_text_center(text, font, font_size, 1.0, rotation);

    draw_text_ex(
        text,
        x_target - center_offset.x,
        y_target - center_offset.y,
        TextParams {
            font_size,
            rotation,
            color,
            ..Default::default()
        },
    );
}

fn make_range(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let mut range = Vec::new();
    let mut current = start;
    while current < stop {
        range.push(current);
        current += step;
    }
    range
}

fn f32max(range: &[f32]) -> f32 {
    let mut max = 0.0;
    for num in range {
        if *num > max {
            max = *num;
        }
    }
    max
}
