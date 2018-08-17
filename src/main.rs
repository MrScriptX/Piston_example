extern crate piston_window;
extern crate piston;

use piston_window::*;

struct ColoredRect
{
    pub color: [f32; 4],//public because piston needs to know color and pos
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl ColoredRect
{
    fn new() -> Self
    {
        ColoredRect
            {
                color: [1.0, 0.5, 0.25, 1.0],
                position: [0.0, 0.0, 100.0, 100.0],
                velocity: [0.3, 0.3]
            }
    }

    fn update_color(color: f32) -> f32
    {
        if color <= 0.0
            {
                1.0//return value 1.0
            }
        else
        {
            color - 0.001
        }
    }

    fn update(&mut self, size: (f64, f64))
    {
        self.color[0] = Self::update_color(self.color[0]);
        self.color[1] = Self::update_color(self.color[1]);
        self.color[2] = Self::update_color(self.color[2]);

        if self.position[0] + self.position[2] >= size.0 || self.position[0] < 0.0//check collision with window
            {
                self.velocity[0] = -self.velocity[0];
            }
        self.position[0] += self.velocity[0];//update pos on X

        if self.position[1] + self.position[3] >= size.1 || self.position[1] < 0.0
            {
                self.velocity[1] = -self.velocity[1];
            }
        self.position[1] += self.velocity[1];
    }

    fn change_velocity(&mut self, factor: f64)
    {
        self.velocity[0] *= factor;
        self.velocity[1] *= factor;
    }
}

fn main()
{
    let mut rect = ColoredRect::new();
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
                .exit_on_esc(true)
                .vsync(true)
                .build().unwrap();

    while let Some(event) = window.next()
        {
            if let Some(Button::Keyboard(key)) = event.press_args()
                {
                    if key == Key::Z
                        {
                            rect.change_velocity(1.1);
                        }
                    else if key == Key::S
                        {
                            rect.change_velocity(0.9);
                        }
                }

            window.draw_2d(&event, |c, g|
                {
                    clear([1.0; 4], g);
                    rectangle(rect.color, rect.position, c.transform, g);
                });

            rect.update((640.0, 480.0));
        }
}
