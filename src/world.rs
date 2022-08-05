use crate::particle::Particle;
use nalgebra::Vector2;
use ggez::graphics;
use ggez::GameError;
use ggez::Context;
use ggez::mint::Point2;
use ggez::graphics::Canvas;


pub struct CircularWorld{
    radius: f32,
    position: Vector2<f32>,
    circle_mesh: graphics::Mesh,
}

impl CircularWorld{
    pub fn new(ctx: &mut Context, radius: f32, position: Vector2<f32>) -> Result<Self, GameError>{
        Ok(CircularWorld{
            radius: radius,
            position: position,
            circle_mesh: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2{x: position.x, y: position.y},
                radius,
                0.1,
                graphics::Color::from_rgb(0,0,0)

            )?,


        })

    }

    pub fn draw(&self, canvas: &mut Canvas){
        canvas.draw(&self.circle_mesh, graphics::DrawParam::default());
    }


    pub fn solve_collision(&self, particle1: &mut Particle, particle2: &mut Particle){
        // Direction of the collision
        let collision_vector = (particle1.get_current_pos() - particle2.get_current_pos()).normalize();

        // How close are both particles
        let current_distance = (particle1.get_current_pos() - particle2.get_current_pos()).magnitude();

        // Distance, when there's no collision
        let desired_distance = particle1.get_radius() + particle2.get_radius(); // r1 + r2

        // Distance the particles need to move to get the desired distance
        let distance_to_move = desired_distance as f32 - current_distance;

        // Move both particles until they not collide anymore
        let new_pos = particle1.get_current_pos() + (collision_vector * distance_to_move * 0.5);
        particle1.set_pos(new_pos);


        let new_pos = particle2.get_current_pos() - (collision_vector * distance_to_move * 0.5);
        particle2.set_pos(new_pos);

        if current_distance < (particle1.get_radius() + particle2.get_radius()) as f32 * 0.9{
            particle1.set_previous_pos(particle1.get_current_pos());
            particle2.set_previous_pos(particle2.get_current_pos());

        }

    }

    // World margin
    pub fn apply_margin(&self, particle: &mut Particle){
        let distance_from_center = (particle.get_current_pos() - self.position).magnitude();

        if distance_from_center > self.radius - particle.get_radius() as f32{
            let normalized_vec = (particle.get_current_pos() - self.position).normalize();
            particle.set_pos(particle.get_current_pos() - (normalized_vec * (distance_from_center - self.radius + particle.get_radius() as f32)));

        }
    }
}



