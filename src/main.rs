use bevy::{
    input::{keyboard::*, mouse::*},
    prelude::*,
};

mod voxel;
use voxel::{VoxelBundle, StateOfMatter};

const SPEED: f32 = 10.;
// TODO: Make this a setting in the options menu eventually
// As of now, this has to be incredibly low.
// Better yet, I think this might best be used as a scalar for sensitivity changes in general.
const SENSITIVITY: f32 = 0.001;
const PHYSICS_TIMESTEP: f32 = 1. / 60.; // Give old mate CPU a break, yeah?

// Region: Components

struct Player;
struct Velocity(Vec3);
struct Camera;

// Region: Systems

fn main() {
    // Maybe run generate_map() or something here.

    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.22, 0.69, 0.87)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_flight.system())
        .add_system(velocity.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // lock cursor -- this doesn't play nice with dwm! it just fails to determine the cursor
    // position. I don't know why, since dwm has no problem locking my cursor with any other
    // game...
    // Maybe I should try in KDE (on X and Wayland) to further diagnose this...
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);

    /*
    // Spawn cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    */
    
    // Spawn our first "voxel"
    commands.spawn_bundle(VoxelBundle {
        cube_mesh: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.1, 0.7, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        state_of_matter: StateOfMatter::Solid,
    });

    // Spawn light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // For whatever reason, changing the cardinal direction you are facing does not change the
    // local x, y, or z until runtime :/. As such, the player must always face -Z for the controls
    // the work as intended.
    // 
    // Maybe I should take some notes from bevy_flying_camera...
    let player_transform = Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y);
    // Spawn the player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: player_transform,
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec3::ZERO))
        .with_children(|parent| {
            // Camera
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: player_transform,
                    ..Default::default()
                })
                .insert(Camera);
        });
}

// TODO: Clamp camera angle so you can't snap your neck
// ...and maybe refactor this. It kinda blows, but I don't know if it's what we need
// or not, maybe the player will have a head that is childed to the body and that handles
// rotation? asdlfjkhal;skdfj;laksj
fn player_flight(
    keyboard_state: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut player_query: Query<(Entity, &Children), With<Player>>,
    mut movement_queries: QuerySet<(Query<&mut Transform>, Query<(&Transform, &mut Velocity)>)>
) {
    // We update camera angle first...
    if let Ok((player, children)) = player_query.single_mut() {
        // TODO: refactor into helper function when implementing walking
        // it's the same behavior, why not reuse it?
        for mouse_move in mouse_motion_events.iter() {
            let delta = -SENSITIVITY * mouse_move.delta;
            if let Ok(mut transform) = movement_queries.q0_mut().get_mut(player) {
                transform.rotate(Quat::from_rotation_y(delta.x));
            }
            for &child in children.iter() {
                if let Ok(mut transform) = movement_queries.q0_mut().get_mut(child) {
                    transform.rotate(Quat::from_rotation_x(delta.y));
                }
            }
        }
        // ...then we add the according movement vector
        if let Ok((transform, mut velocity)) = movement_queries.q1_mut().get_mut(player) {
            let mut dir_vec = Vec3::ZERO;
            for key in keyboard_state.get_pressed() {
                dir_vec += match key {
                    KeyCode::W => -transform.local_z(),
                    KeyCode::A => -transform.local_x(),
                    KeyCode::S => transform.local_z(),
                    KeyCode::D => transform.local_x(),
                    KeyCode::Space => transform.local_y(),
                    KeyCode::LShift => -transform.local_y(),
                    _ => Vec3::ZERO,
                }
            }
            velocity.0 = SPEED * dir_vec.normalize_or_zero();
        }
    }
}

/// Changes transform of all entities with a velocity component
fn velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
