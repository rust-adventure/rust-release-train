// #![forbid(unsafe_code)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Update, hello_world_system)
        .run();
}

fn hello_world_system(
    _transform_query: Query<&Transform>,
    _global_transform_query: Query<&GlobalTransform>,
    _sprites: Query<&Sprite>,
    _atlases: Option<Res<Assets<TextureAtlas>>>,
    _materials: Option<Res<Assets<StandardMaterial>>>,
    _audio: Option<Res<Assets<AudioSource>>>,
    mut _gizmos: Gizmos,
    _aabbs: Query<
        &AabbGizmo,
        (
            With<Transform>,
            Changed<Sprite>,
            Without<GlobalTransform>,
        ),
    >,
) {
    println!("hello world");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
