use std::sync::{Arc, RwLock};

use ecs::{
    DefaultWorld,
    EntityHandler,
    ComponentHandler,
    SystemHandler,
    Read,
    Write,
    Searchable
};

struct Health(u32);
struct Commander;
struct IsPlayer;
struct IsEnemy;
struct CommnaderKeen;

fn main() {
    let world = DefaultWorld::default();

    world.register::<IsPlayer>();
    world.register::<Commander>();
    world.register::<Health>();
    world.register::<IsEnemy>();

    world.add_entity((IsPlayer, Health(123)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, Health(333)));
    world.add_entity((Commander, Health(333)));
    world.add_entity((Commander, Health(333)));
    world.add_entity((Commander, Health(333)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, IsEnemy, Health(1)));
    world.add_entity((Commander, IsEnemy, Health(1)));

    world.add_entity((IsEnemy,));
    world.add_entity((IsEnemy,));
    world.add_entity((IsEnemy,));
    world.add_entity((IsEnemy,));
    world.add_entity((IsEnemy,));
    world.add_entity((IsEnemy, Health(31231233)));
    
    // TODO(Angel): Fix this
    //world.remove_entity(Entity::new(3));

    world.run(print_health_system);
    println!("ASD");
    world.run(print_heath_and_isenemy_system);
}

fn print_health_system(healths: Read<Health>) {
    // Print all healths state.
    let mut counter: i32 = 0;
    for health in healths.iter() {
        counter += 1;
        println!("{:?}", health.read().0);
    }

    println!("Counter {}", counter);
}

fn print_heath_and_isenemy_system(
    healths: Read<Health>,
    enemies: Read<IsEnemy>,
    commander: Read<Commander>,
    payer: Read<IsPlayer>
) {
    let query = (
        healths.iter(),
        enemies.iter(),
        commander.iter(),
        payer.iter()
    ).query();

    for (heath, _, _, _) in query {
        println!("health: {}", heath.read().0)
    }
}
