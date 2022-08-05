use crate::particle::Particle;
use ggez::{Context, graphics, GameError};
use nalgebra::Vector2;
use ggez::mint::Point2;
use crate::world;


pub struct ParticleSystem {
    particles_list: Vec<Particle>,
    world: world::CircularWorld,

}


impl ParticleSystem{

    pub fn new(ctx: &mut Context) -> Result<Self, GameError>{
        let (window_width, window_height) = ctx.gfx.drawable_size();

        Ok(ParticleSystem{
            particles_list: Vec::new(),
            world: world::CircularWorld::new(ctx, window_height/2., Vector2::new(window_width/2., window_height/2.))?,

        })
    }


    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> Option<Particle>{
        self.world.draw(canvas);

        for i in  0..self.particles_list.len() {
            let (particle, mut particles) = self.particles_list.split_one_mut(i);

            // Gravity
            particle.apply_force(Vector2::new(0.0, 2000.0));

            // World margin
            self.world.apply_margin(particle);

            // Handling collisions

            // Brute force collision detection

            for other_particle in particles{
                    if !particle.is_equals_to(other_particle) && particle.is_colliding(other_particle){
                        self.world.solve_collision(particle, other_particle);
                    }
            }



            particle.update_pos(ctx.time.average_delta().as_secs_f32());

            particle.draw(canvas);



        }


        None

    }

    pub fn add_particle(&mut self, particle: &mut Particle, number_of_particles: u32){
        for i in 0..number_of_particles {
            let mut new_part = particle.clone();
            new_part.set_id(i + new_part.get_id());
            self.particles_list.push(new_part);
        }
    }

    pub fn len(&self) -> usize{
        self.particles_list.len()
    }

}

type ImplIteratorMut<'a, Item> =
::std::iter::Chain<
    ::std::slice::IterMut<'a, Item>,
    ::std::slice::IterMut<'a, Item>,
>
;
trait SplitOneMut {
    type Item;

    fn split_one_mut (
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>);
}

impl<T> SplitOneMut for [T] {
    type Item = T;

    fn split_one_mut (
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>)
    {
        let (prev, current_and_end) = self.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        (
            &mut current[0],
            prev.iter_mut().chain(end),
        )
    }
}

