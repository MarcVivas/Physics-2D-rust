mod particle_system;
mod particle;
mod world;

use rand;
use crate::particle_system::ParticleSystem;
use crate::particle::Particle;

use ggez;
use ggez::{Context, GameResult, conf, event, graphics, GameError};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyMods};
use ggez::input::mouse::{MouseButton};
use nalgebra::{Vector2, Point2};
use rand::Rng;


struct MainState{
    show_info: bool,
    particles: ParticleSystem,


}

impl MainState{
    pub fn new(ctx: &mut Context) -> Result<Self, GameError> {
        return Ok(MainState {
            show_info: true,
            particles:  ParticleSystem::new(ctx)?,
        })
    }

    pub fn show_info(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) {
        if self.show_info{
            let mut fps = String::from("FPS: ");
            fps.push_str(ctx.time.fps().to_string().as_str());
            fps.push_str("\nNumber of particles: ");
            fps.push_str(self.particles.len().to_string().as_str());
            let text_fps = graphics::Text::new(fps);
            canvas.draw(&text_fps, graphics::DrawParam::default());
        }
    }
}



// Infinite loop
impl event::EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::CanvasLoadOp::Clear(graphics::Color::from_rgb(0,255,255)));

        self.particles.draw(ctx, &mut canvas);

        self.show_info(ctx, &mut canvas);
        canvas.finish(ctx)?;
        return Ok(());
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        match button{
            // If left mouse button has been clicked, add one particle
            MouseButton::Left => {
                self.particles.add_particle(
                     &mut Particle::new(
                         ctx,
                        Vector2::new(x, y),
                        rand::thread_rng().gen_range(6..30),
                        22,
                        Vector2::new(0.0, 0.0),
                        graphics::Color::from_rgb(rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)),
                         self.particles.len() as u32
                    )?,
                    1
                );
            }
            // Else
            _ => (),
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyInput, _repeat: bool ) -> GameResult {
        match key.keycode {
            // Quit if Shift+Ctrl+Q is pressed.
            Some(KeyCode::Q) => {
                if key.mods.contains(KeyMods::SHIFT) && key.mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    event::request_quit(ctx);
                } else if key.mods.contains(KeyMods::SHIFT) || key.mods.contains(KeyMods::CTRL) {
                    println!("Quit if Shift+Ctrl+Q is pressed");
                } else {
                    println!("Quit if Shift+Ctrl+Q is pressed");
                }
            }

            Some(KeyCode::D) => {
                self.show_info = !self.show_info;
            }


            //Add particles
            Some(KeyCode::P) =>{
                let mouse_pos = ctx.mouse.position();
                self.particles.add_particle(
                    &mut Particle::new(
                        ctx,
                        Vector2::new(mouse_pos.x, mouse_pos.y),
                        rand::thread_rng().gen_range(3..10),
                        22,
                        Vector2::new(0.0, 0.0),
                        graphics::Color::from_rgb(rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)),
                        self.particles.len() as u32
                    )?,
                    1
                );
            }

            _ => (),
        }
        Ok(())
    }
}

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

fn main() -> GameResult {

    // Window configuration
    let context_builder = ggez::ContextBuilder::new("ProgramName", "Marc")
        .window_setup(conf::WindowSetup::default().title("Program").vsync(true).samples(conf::NumSamples::Four))
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT).maximized(true));
    let (mut context, event_loop) = context_builder.build()?;

    // State of the program
    let state = MainState::new(&mut context)?;

    // Runs the infinite loop
    event::run(context, event_loop, state);

}
