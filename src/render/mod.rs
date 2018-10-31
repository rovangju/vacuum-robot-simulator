use opengl_graphics::GlGraphics;

use graphics::color;
use graphics::ellipse::Ellipse;
use graphics::line::Line;
use graphics::math::Matrix2d;
use graphics::rectangle::Rectangle;
use graphics::{DrawState, Transformed};

use controller;
use geometry;
use pointcloud;
use simulation::robot;

pub struct RenderConfig {
    pub scale: f64,
}

impl RenderConfig {
    pub fn pixel_coords(&self, vec: geometry::Vector) -> (f64, f64) {
        (self.scale * (vec.x as f64), -self.scale * (vec.y as f64))
    }
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub trait Draw {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics);
}

impl Draw for geometry::Line {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let line = Line::new(WHITE, 1.0);

        let (x1, y1) = config.pixel_coords(self.start);
        let (x2, y2) = config.pixel_coords(self.end);
        let coords: [f64; 4] = [x1, y1, x2, y2];

        line.draw(coords, &DrawState::default(), transform, gl);
    }
}

impl Draw for controller::Controller {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let transform_gridmap = transform.trans(-450.0, 0.0);
        self.gridmap.draw(config, transform_gridmap, gl);
    }
}

impl Draw for controller::gridmap::GridMap {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        use controller::gridmap::CellState::*;
        let size = controller::gridmap::SIZE;
        let cell_size = 0.1 * config.scale;

        // Draw background
        let rect_bg = Rectangle::new(color::hex("333333"));
        let width = cell_size * (size as f64);
        rect_bg.draw(
            [0.0, -width, width, width],
            &DrawState::default(),
            transform,
            gl,
        );

        // Draw cells
        let mut draw_cell = |rect: Rectangle, r, c| {
            let x = (c as f64) * cell_size;
            let y = -(r as f64) * cell_size;

            rect.draw(
                [x, y, cell_size, cell_size],
                &DrawState::default(),
                transform,
                gl,
            );
        };

        let rect_occupied = Rectangle::new(WHITE);
        let rect_freespace = Rectangle::new(color::hex("525f49"));
        for r in 0..size {
            for c in 0..size {
                match self.cell_state(r, c) {
                    Some(&Occupied(_)) => draw_cell(rect_occupied, r, c),
                    Some(&Freespace) => draw_cell(rect_freespace, r, c),
                    _ => {}
                }
            }
        }
    }
}

impl Draw for robot::Robot {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let robot_color = color::hex("ffd42a");
        let robot_circ = Ellipse {
            color: robot_color,
            border: None,
            resolution: 64,
        };

        let robot_radius = config.scale;
        let pos = self.pose.position;
        let (px, py) = config.pixel_coords(pos);

        robot_circ.draw(
            [0.0, 0.0, robot_radius, robot_radius],
            &Default::default(),
            transform.trans(px - robot_radius / 2.0, py - robot_radius / 2.0),
            gl,
        );

        // Draw heading angle
        let line = Line::new(robot_color, 1.0);
        let (hx, hy) = config.pixel_coords(
            pos + geometry::Vector::from_angle(self.pose.heading) * config.scale * 0.05,
        );
        line.draw([px, py, hx, hy], &DrawState::default(), transform, gl);
    }
}

impl Draw for pointcloud::PointCloud {
    fn draw(&self, config: &RenderConfig, transform: Matrix2d, gl: &mut GlGraphics) {
        let point = Ellipse {
            color: color::hex("1a1a1a"),
            border: None,
            resolution: 32,
        };
        let point_radius = 0.25 * config.scale;
        for &p in self.iter() {
            let (px, py) = config.pixel_coords(p.pos);

            point.draw(
                [0.0, 0.0, point_radius, point_radius],
                &Default::default(),
                transform.trans(px - point_radius / 2.0, py - point_radius / 2.0),
                gl,
            )
        }
    }
}
