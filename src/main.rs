mod geometry;
use geometry::*;

mod linear_function;
use linear_function::LinearFunction;

use minifb::{Key, Window, WindowOptions};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const WINDOW_WIDTH: usize = WIDTH as usize;
const WINDOW_HEIGHT: usize = HEIGHT as usize;
const LINE_COLOR: u32 = 255 << 16;
const CAMERA_SPEED: i32 = 350;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
        "Software Renderer",
        WINDOW_WIDTH, WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(
        Some(std::time::Duration::from_secs_f64(1.0 / 60.0))
    );

    let plate = read_obj("plate.obj");
    println!("{}", plate.points.len());

    let triangle = Triangle {
        p1: Point::new(20, 300),
        p2: Point::new(210, 200),
        p3: Point::new(210, 300),
        color: LINE_COLOR,
        draw_method: DrawMethod::WireFrame,
    };

    let triangle2 = Triangle {
        p1: Point::new(600, 200),
        p2: Point::new(600, 500),
        p3: Point::new(700, 200),
        color: LINE_COLOR,
        draw_method: DrawMethod::Fill,
    };

    let triangles = vec![triangle, triangle2];

    let mut camera = Point { x: 0, y: 0 };
    let mut last = std::time::Instant::now();

    while window.is_open() {
        let delta = last.elapsed().as_nanos() as f32 / 1_000_000_000.0;
        last = std::time::Instant::now();

        let camera_move_speed = (CAMERA_SPEED as f32 * delta) as i32;

        if window.is_key_down(Key::W) {
            camera.y += camera_move_speed;
        }
        if window.is_key_down(Key::A) {
            camera.x -= camera_move_speed;
        }
        if window.is_key_down(Key::S) {
            camera.y -= camera_move_speed;
        }
        if window.is_key_down(Key::D) {
            camera.x += camera_move_speed;
        }

        for i in buffer.iter_mut() {
            *i = 0;
        }

        for triangle in &triangles {
            draw_triangle(&triangle, &camera, &mut buffer);
        }

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn draw_triangle(triangle: &Triangle, camera: &Point<i32>, buffer: &mut Vec<u32>) {
    match triangle.draw_method {
        DrawMethod::WireFrame => {
            draw_line(&triangle.p1, &triangle.p2, camera, buffer, triangle.color);
            draw_line(&triangle.p2, &triangle.p3, camera, buffer, triangle.color);
            draw_line(&triangle.p3, &triangle.p1, camera, buffer, triangle.color);
        },
        DrawMethod::Fill => {
            let mut points: Vec<Point<i32>> = 
                vec![&triangle.p1, &triangle.p2, &triangle.p3]
                    .iter()
                    .map(|p| world_to_screen(p, camera))
                    .collect();

            // Sort points by y-value to prep for top-down filling.
            points.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());

            // Any triangle (that does not already have a flat base) can be cut into two flat based triangles by slicing the shape at the horizontal line
            // y = b.y, where b has the "middle" value of y compared to the other triangle's points (aka. a.y < b.y < c.y or c.y < b.y < a.y).
            let longside = LinearFunction::from_points(&points[0], &points[2]);

            if points[0].y != points[1].y {
                fill_flat_base_triangle(
                    &longside,
                    &LinearFunction::from_points(&points[0], &points[1]),
                    points[0].y, points[1].y,
                    buffer, triangle.color,
                );
            }

            if points[1].y != points[2].y {
                fill_flat_base_triangle(
                    &longside,
                    &LinearFunction::from_points(&points[1], &points[2]),
                    points[1].y, points[2].y,
                    buffer, triangle.color,
                );
            }
        }
    }
}

fn fill_flat_base_triangle(line1: &LinearFunction, line2: &LinearFunction, y1: i32, y2: i32, buffer: &mut Vec<u32>, color: u32) {
    for y in order_range(y1, y2) {
        let x1 = line1.solve_x(y);
        let x2 = line2.solve_x(y);

        for x in order_range(x1, x2) {
            // TODO: Shouldn't need to construct new Point struct here, maybe make a faster version of draw_pixel with primitives?
            draw_pixel(&Point::new(x, y), buffer, color);
        }
    }
}

fn draw_line(p1: &Point<i32>, p2: &Point<i32>, camera: &Point<i32>, buffer: &mut Vec<u32>, color: u32) {
    // Handle drawing a vertical line since we can't calculate slope.
    if p1.x == p2.x {
        for y in order_range(p1.y, p2.y) {
            draw_pixel(
                &world_to_screen(&Point::<i32>::new(p1.x, y), camera),
                buffer, color
            );
        }
        return;
    }

    // Handle drawing a horizontal line, as we can skip certain computational steps.
    if p1.y == p2.y {
        for x in order_range(p1.x, p2.x) {
            draw_pixel(
                &world_to_screen(&Point::<i32>::new(x, p1.y), camera),
                buffer, color
            )
        }
        return;
    }

    let fx = LinearFunction::from_points(&p1, &p2);

    for x in order_range(p1.x, p2.x) {
        let y = fx.solve_y(x);
        let next_y = fx.solve_y(x + 1);
            
        // Fill vertical pixels to connect this part of the line to the next x value.
        for mid_y in order_range(y, next_y) {
            draw_pixel(
                &world_to_screen(&Point::<i32>::new(x, mid_y), camera),
                buffer, color
            );
        }
    }
}

fn order_range(value1: i32, value2: i32) -> std::ops::Range<i32> {
    std::ops::Range {
        start: std::cmp::min(value1, value2),
        end: std::cmp::max(value1, value2),
    }
}

fn draw_pixel(pixel: &Point<i32>, buffer: &mut Vec<u32>, color: u32) {
    let row = HEIGHT - pixel.y;
    if pixel.x >= 0 && pixel.x < (WIDTH - 1) && row >= 0 && row < (HEIGHT - 1) {
        buffer[(row * WIDTH + pixel.x) as usize] = color;
    }
}

fn world_to_screen(point: &Point<i32>, camera: &Point<i32>) -> Point<i32> {
    Point {
        x: point.x - camera.x,
        y: point.y - camera.y,
    }
}
