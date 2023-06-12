// Graphics Library
extern crate piston_window;
use piston_window::*;

// Seed
use crate::lib::seed::*;

#[derive(Clone)]
pub struct Simulation<const WIDTH: usize, const HEIGHT: usize> {
    width: f64,
    height: f64,

    seeds: Vec<Seed>
}

impl<const WIDTH: usize, const HEIGHT: usize> Simulation<WIDTH, HEIGHT> {
    pub fn new(seed_qty: usize) -> Self {
        let mut seeds = Vec::new();

        for _ in 0..seed_qty {
            seeds.push(Seed::new(WIDTH as f64, HEIGHT as f64));
        }

        Simulation {
            width: WIDTH as f64,
            height: HEIGHT as f64,

            seeds
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
        let mut window: PistonWindow = WindowSettings::new("RustyVoronoi", [self.width as u32, self.height as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

        while let Some(e) = events.next(&mut window) {
            // Keep Mouse Cursor Position
            e.mouse_cursor(|pos| {
                self.seeds[0].x = pos[0];
                self.seeds[0].y = pos[1];
            });

            // Handle Mouse Click
            if let Some(button) = e.press_args() {

            }

            // Draw graphics
            window.draw_2d(&e, |c, g, device| {
                // Clear canvas
                clear([0.0, 0.0, 0.0, 1.0], g);

                // Update Voronoi
                for x in 0..WIDTH as u32 {
                    for y in 0..HEIGHT as u32 {
                        let (mut min_dist, mut min_idx) = (f64::MAX, 0);

                        for (idx, point) in self.seeds.iter().enumerate() {
                            let dist = ((x as f64 - point.x) * (x as f64 - point.x) +
                                (y as f64 - point.y) * (y as f64 - point.y)).sqrt();

                            if dist < min_dist {
                                min_dist = dist;
                                min_idx = idx;
                            }
                        }

                        let color = [(min_idx * 50 % 255) as f32 / 255.0,
                            (min_idx * 80 % 255) as f32 / 255.0,
                            (min_idx * 100 % 255) as f32 / 255.0,
                            1.0];

                        rectangle(color,
                                  [x as f64, y as f64, 1.0, 1.0],
                                  c.transform,
                                  g);
                    }
                }
            });
        }
    }
}
