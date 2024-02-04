use crate::entities::data::{AttackableFrom, EntityType, Health};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
use rand::{thread_rng, Rng};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub static TILE_SIZE: usize = 300;
pub static SUB_TILES: usize = 5;
pub static GROUND_TILE_COUNT: usize = 5;

#[derive(Component)]
pub struct Tile;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("forest-ground.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(16., 16.),
        GROUND_TILE_COUNT,
        1,
        None,
        None,
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Health(20.),
        EntityType::Wall,
        AttackableFrom(vec![EntityType::Mob]),
        RigidBody::Static,
        Collider::cuboid(100., 100.),
        Restitution::new(0.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.4, 0.1, 0.5),
                custom_size: Some(vec2(100., 100.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            ..default()
        },
    ));

    for x in -5..=5isize {
        for y in -5..=5isize {
            if vec2(x as f32, y as f32).length() > 4. {
                blocked_normal_tile(&mut commands, x, y);
            } else {
                normal_tile(&mut commands, x, y, atlas_handle.clone());
            }
        }
    }
}

fn normal_tile(commands: &mut Commands, x: isize, y: isize, atlas_handle: Handle<TextureAtlas>) {
    let mut rng = thread_rng();

    commands
        .spawn((
            Tile,
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE as f32,
                    y as f32 * TILE_SIZE as f32,
                    -5.,
                )),
                ..default()
            },
        ))
        .with_children(|parent| {
            let sub_tile_size = TILE_SIZE as f32 / 5.;

            for x in 0..SUB_TILES {
                for y in 0..SUB_TILES {
                    //offset so 0/0 is in the center of the tile
                    let x = x as f32 - SUB_TILES as f32 / 2.;
                    let y = y as f32 - SUB_TILES as f32 / 2.;

                    println!("{}", x);

                    parent.spawn(SpriteSheetBundle {
                        texture_atlas: atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            index: rng.gen_range(0..GROUND_TILE_COUNT),
                            custom_size: Some(vec2(sub_tile_size, sub_tile_size)),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            x * sub_tile_size,
                            y * sub_tile_size,
                            -5.,
                        )),
                        ..default()
                    });
                }
            }
        });
}

fn blocked_normal_tile(commands: &mut Commands, x: isize, y: isize) {
    commands.spawn((
        Tile,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.2, 0.1),
                custom_size: Some(vec2(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE as f32,
                y as f32 * TILE_SIZE as f32,
                -5.,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(TILE_SIZE as f32, TILE_SIZE as f32),
        Restitution::new(0.),
    ));
}
