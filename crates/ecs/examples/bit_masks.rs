use std::{
    thread,
    sync::{Arc, RwLock}
};

use ecs::{
    DefaultWorld,
    EntityHandler,
    ComponentHandler,
    SystemHandler,
    Entity,
    Read,
    UniqueRead,
    UniqueWrite,
    Write,
    Searchable,
    TaskWaitable
};

struct Health(u32);
struct Commander;
struct IsPlayer;
struct IsEnemy;
struct CommnaderKeen;
struct Renderer(f64);

fn main() {
    let world = DefaultWorld::default();

    world.register::<IsPlayer>();
    world.register::<Commander>();
    world.register::<Health>();
    world.register::<IsEnemy>();
    world.register_unique(Renderer(33.0));

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
    world.add_entity((IsEnemy, Health(31231233)));
    
    world.remove_entity(Entity::new(3));

    (
        world.run(update_system),
    ).wait();

    (
        world.run(print_health_system),
        world.run(print_heath_and_isenemy_system)
    ).wait();

    (
        world.run(test_write_renderer),
    ).wait();

    (
        world.run(test_renderer),
    ).wait();
}

fn update_system(
    healhts: Write<Health>,
    enemies: Read<IsEnemy>,
    commander: Read<Commander>
) {
    let query = (
        healhts.iter(),
        enemies.iter(),
        commander.iter()
    ).query();

    for (health, _, _) in query {
        health.write().0 = 1337;
    }
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
    commander: Read<Commander>
) {
    let query = (
        healths.iter(),
        enemies.iter(),
        commander.iter()
    ).query();

    let handle = thread::current();
    println!("Print heath thread name: {:?}", handle.name());

    for (heath, _, _) in query {
        println!("health: {}", heath.read().0)
    }
}

fn test_renderer(
    renderer: UniqueRead<Renderer>,
    _enemies: Read<IsEnemy>) {

    println!("Funcando! {}", renderer.read().0);
}

fn test_write_renderer(
    renderer: UniqueWrite<Renderer>,
    _enemies: Read<IsEnemy>) {
    renderer.write().0 = 12223.0;
}
