use std::sync::{Arc, RwLock};

use ecs::{
    DefaultWorld,
    EntityHandler,
    ComponentHandler,
    Entity,
    SystemHandler,
    Read,
    Write,
    Accessible
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
    isEnemy: Read<IsEnemy>,
    commander: Read<Commander>
) {
    // Print all healths state.
    let mut counter: i32 = 0;
    for health in healths.iter() {
        counter += 1;
        println!("{:?}", health.read().0);
    }

    println!("Counter {}", counter); 
}
