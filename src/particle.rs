use ggez::{graphics, GameError};
use nalgebra::{Vector2};
use ggez::{Context, GameResult};
use ggez::mint::Point2;


#[derive(Clone, Debug)]
pub struct Particle{
    id: u32,
    current_pos: Vector2<f32>,
    previous_pos: Vector2<f32>,
    radius: u16,
    mass: u32,
    acceleration: Vector2<f32>,
    color: graphics::Color,
    mesh: graphics::Mesh,
}



impl Particle{
    // Constructor
    pub fn new(ctx: &mut Context, pos: Vector2<f32>, radius: u16, mass: u32, acceleration: Vector2<f32>, color: graphics::Color, id: u32) -> Result<Self, GameError> {
        Ok(Particle{
            id,
            current_pos: pos,
            previous_pos: pos,
            radius,
            mass,
            acceleration,
            color,
            mesh:  graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2{x: 0., y: 0.},
                1.0 * radius  as f32,
                0.1,
                        color,
                )?,
        })

    }
    // Draws a particle on the canvas
    pub fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult{
        let params = graphics::DrawParam::new().color(self.color).dest(Point2{x: self.current_pos.x, y: self.current_pos.y});        // Scale doesn't work with circles .scale(ggez::mint::Vector2{x: self.radius as f32, y:self.radius as f32});
        canvas.draw(&self.mesh, params);
        Ok(())
    }

    pub fn update_pos(&mut self, dt: f32) {
        // Get the current velocity
        let velocity: Vector2<f32> = self.current_pos - self.previous_pos;

        // Save the current position
        self.previous_pos = self.current_pos;

        // Update the current position using Verlet integration
        self.current_pos += velocity + self.acceleration * dt  * dt;

        //Reset acceleration
        self.acceleration *= 0.0;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>){
        /*
        Force = Mass * Acceleration
        Acceleration = Force / Mass
        */
        self.acceleration += force / self.mass as f32;
    }

    // Check if the particle is colliding with the given particle
    pub fn is_colliding(&self, other_particle:  &Particle) -> bool{
        return f32::powf((self.get_radius() + other_particle.get_radius()) as f32, 2.0)   > f32::powf((other_particle.get_current_pos().x - self.get_current_pos().x), 2.0) + f32::powf((other_particle.get_current_pos().y - self.get_current_pos().y), 2.0)
    }

    pub fn is_equals_to(&self ,other: &Particle) -> bool{
        return self.get_id() == other.get_id()
    }

    // Getters and setters
    pub fn get_previous_pos(&self)-> Vector2<f32>{
        self.previous_pos
    }
    pub fn get_id(&self) -> u32{
        return self.id;
    }
    pub fn get_radius(&self) -> u16{
        self.radius
    }

    pub fn get_current_pos(&self) -> Vector2<f32>{
        self.current_pos
    }

    pub fn get_mass(&self) -> u32 {
        self.mass
    }

    pub fn get_acceleration(&self) -> Vector2<f32>{
        self.acceleration
    }

    pub fn set_previous_pos(&mut self, prev: Vector2<f32>){
        self.previous_pos = prev;
    }
    pub fn set_pos(&mut self, pos: Vector2<f32>) {
        self.current_pos = pos;
    }

    pub fn set_radius(&mut self, rad: u16){
        self.radius = rad;
    }

    pub fn set_mass(&mut self, mass: u32){
        self.mass = mass;
    }

    pub fn set_acceleration(&mut self, acc: Vector2<f32>){
        self.acceleration = acc;
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

}