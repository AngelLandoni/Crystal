use ecs::{DefaultWorld, EntityHandler};

struct Health(u32);
struct IsPlayer;

fn main() {
    let world = DefaultWorld::default();

    // Adds a new entity into the world.
    world.add_entity((IsPlayer, ));
}