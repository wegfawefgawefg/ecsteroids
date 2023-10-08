use glam::{UVec2, Vec2};
use rand::Rng;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode, Vector2};

pub type RenderCommandBuffer = Vec<DrawCommand>;

#[derive(Clone)]
pub enum DrawCommand {
    ClearScreen,
    ColoredSquare { pos: Vec2, color: Color },
    Ship { pos: Vec2, dir: Vec2 },
    Asteroid { pos: Vec2, size: u32, dir: Vec2 },
}

// defualt entity size
const SIZE: i32 = 1;
const SEGMENTS: usize = 12;
static RADIUS_VARIATIONS: [f32; SEGMENTS] = [
    0.8, 0.75, 0.9, 0.85, 0.7, 0.88, 0.95, 0.78, 0.92, 0.76, 0.87, 0.8,
];

pub fn execute_render_command_buffer(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    render_command_buffer: &mut RenderCommandBuffer,
) {
    let mut rng = rand::thread_rng();
    for command in render_command_buffer.iter() {
        match command {
            DrawCommand::ClearScreen => {
                d.clear_background(Color::WHITE);
            }
            DrawCommand::ColoredSquare { pos, color } => {
                d.draw_rectangle(pos.x as i32, pos.y as i32, SIZE, SIZE, *color);
            }
            DrawCommand::Ship { pos, dir } => {
                let ship_color = Color::new(0, 100, 255, 255);
                let center = Vec2::new(pos.x, pos.y);
                let dir = dir.normalize();
                let dir = dir * 10.0;
                d.draw_circle_lines(center.x as i32, center.y as i32, 3.0, ship_color);
                d.draw_line(
                    center.x as i32,
                    center.y as i32,
                    (center.x + dir.x) as i32,
                    (center.y + dir.y) as i32,
                    ship_color,
                );
            }
            DrawCommand::Asteroid { pos, size, dir } => {
                let mut points: Vec<Vec2> = Vec::new();
                let base_angle = 2.0 * std::f32::consts::PI / SEGMENTS as f32;

                // Convert the direction vector into an angle
                let rot_angle = dir.y.atan2(dir.x);

                // Generate points for the asteroid using the static radius variations
                for i in 0..SEGMENTS {
                    let angle = base_angle * i as f32;
                    let r = *size as f32 * RADIUS_VARIATIONS[i];

                    let point = Vec2 {
                        x: r * angle.cos(),
                        y: r * angle.sin(),
                    };

                    // Rotate the point around the asteroid's center using glam's Mat2
                    let rotation_matrix = glam::Mat2::from_angle(rot_angle);
                    let rotated_point = rotation_matrix * point + *pos;

                    points.push(rotated_point);
                }

                d.draw_circle(
                    pos.x as i32,
                    pos.y as i32,
                    *size as f32 * 0.8,
                    Color::new(255, 255, 255, 30),
                );

                // Draw each line segment for the asteroid
                for i in 0..SEGMENTS {
                    let start_point = points[i];
                    let end_point = points[(i + 1) % SEGMENTS];
                    d.draw_line_v(
                        Vector2::new(start_point.x, start_point.y),
                        Vector2::new(end_point.x, end_point.y),
                        Color::WHITE,
                    );
                }

                // also draw a debug circle at the center of the asteroid
            }
        }
    }
}
