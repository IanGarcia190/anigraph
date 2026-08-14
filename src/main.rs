use macroquad::math::Vec2;
use macroquad::prelude::*;

fn window() -> Conf {
    Conf {
        window_title: "A Graph".to_string(),
        window_width: 800,
        window_height: 600,
        platform: miniquad::conf::Platform {
            swap_interval: Some(2), // 0: unlimited, 1: 60fps, 2: 30fps
            ..Default::default()
        },
        ..Default::default()
    }
}

//-------------Main Loop--------------------------------------------------------
#[macroquad::main(window)]
async fn main() {
    let x_data = make_range(0.0, 16.0, 0.1);
    let y_data = target_function(&x_data);
    let x_max = f32max(&x_data);
    let y_max = f32max(&y_data);
    // let (x_dim, y_dim) = dimensions;
    // for dim in x_dim {
    //     println!("X: {}, Y: {}", dim.x, dim.y);
    // }
    // println!();
    // for dim in y_dim {
    //     println!("X: {}, Y: {}", dim.x, dim.y);
    // }
    // std::process::exit(0);

    loop {
        clear_background(BLACK);
        let x_values = axis_values(x_max);
        let y_values = axis_values(y_max);
        let dimensions = axis_dimensions(&x_values, &y_values, x_max, y_max);
        draw_axis(&(x_values, y_values), &dimensions);

        let point_coord = point_coord(&x_data, &y_data, x_max, y_max);
        draw_points(&point_coord);
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await;
    }
}

fn point_coord(x_data: &[f32], y_data: &[f32], x_max: f32, y_max: f32) -> Vec<Vec2> {
    let mut coordinates = Vec::new();

    let origin: Vec2 = Vec2::new(0.1 * screen_width(), 0.9 * screen_height() + 2.0);

    let bodywidth = 0.8 * screen_width();
    let bodyheight = 0.8 * screen_height();

    //normalize values
    //stretch by frame width and height
    //add origin buffer

    // x_data and y_data are essentially guarenteed to have the same length.
    for i in 0..x_data.len() {
        let mut x = x_data[i];
        x /= x_max;
        x *= bodywidth;
        x += origin.x;

        let mut y = y_data[i];
        y /= y_max;
        y *= bodyheight;
        y = origin.y - y;

        let new_vec2 = Vec2::new(x, y);
        coordinates.push(new_vec2);
    }

    coordinates
}

fn draw_points(coordinates: &Vec<Vec2>) {
    for coord in coordinates {
        draw_circle(coord.x, coord.y, 5.0, YELLOW);
    }
}

fn target_function(range: &[f32]) -> Vec<f32> {
    let result: Vec<f32> = range.iter().map(|x| x.powf(0.5)).collect();
    result
}

fn axis_values(max: f32) -> Vec<f32> {
    let mut values = make_range(0.0, max, max / 5.0);
    for value in values.iter_mut() {
        *value = value.floor();
    }

    values
}

fn axis_dimensions(
    x_values: &Vec<f32>,
    y_values: &Vec<f32>,
    x_max: f32,
    y_max: f32,
) -> (Vec<Vec2>, Vec<Vec2>) {
    //sus out actual dimensions that these positions give on the body of the graph;
    //find max, divide by max, multiply every element by body width.

    let bodywidth = 0.8 * screen_width();
    let bodyheight = 0.8 * screen_height();
    let x_buffer = 0.075 * screen_width();
    let y_buffer = 0.075 * screen_height();

    let mut x_dims: Vec<Vec2> = Vec::new();
    let mut y_dims: Vec<Vec2> = Vec::new();
    for value in x_values {
        let mut x = *value;
        x /= x_max;
        x *= bodywidth;
        x += x_buffer;
        let new_vec2 = Vec2::new(x, screen_height() - y_buffer);
        x_dims.push(new_vec2);
    }
    for value in y_values {
        let mut y = *value;
        y /= y_max;
        y *= bodyheight;
        y += y_buffer;
        let new_vec2 = Vec2::new(x_buffer, screen_height() - y);
        y_dims.push(new_vec2);
    }

    (x_dims, y_dims)
}

fn draw_axis(values: &(Vec<f32>, Vec<f32>), dimensions: &(Vec<Vec2>, Vec<Vec2>)) {
    //I am going to do arithmetic in terms of percentile.
    //So I can simply multiply by window size.

    let (x_val, y_val) = values;
    let (x_dim, y_dim) = dimensions;
    if x_val.len() != y_val.len() {
        println!("The arrays containing values for both axis were not of the same length.");
        std::process::exit(1);
    }
    for i in 0..x_val.len() {
        let x_position = x_dim[i];
        let y_position = y_dim[i];

        //drawing x-value
        text_draw(
            &format!("{:.2}", x_val[i]),
            x_position.x,
            x_position.y,
            None,
            30,
            0.0,
            YELLOW,
        );

        //drawing y-value
        text_draw(
            &format!("{:.2}", y_val[i]),
            y_position.x,
            y_position.y,
            None,
            30,
            0.0,
            YELLOW,
        );
    }

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
        screen_width() * 0.025,
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
