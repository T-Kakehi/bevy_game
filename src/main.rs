use bevy::prelude::*;

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .run();
}
