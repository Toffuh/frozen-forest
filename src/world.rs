use crate::entities::data::{AttackableFrom, EntityType, Health};
use crate::PhysicsLayers;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
use frozen_forest_macro::sprite_sheet;
use iter_tools::Itertools;
use rand::{thread_rng, Rng};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    hover_tile,
                    (highlight_hovered_tiles, activate_hovered_tiles),
                    create_surrounding_tiles,
                    activate_tiles,
                )
                    .chain(),
            )
            .add_event::<HoverTileEvent>()
            .add_event::<ActivateTileEvent>();
    }
}

pub static SUB_TILES: f32 = 15.;
pub static TILE_SIZE: f32 = SUB_TILES * 16.;
pub static GROUND_SPRITE_COUNT: usize = 9;

#[derive(Event)]
pub struct HoverTileEvent(Entity);

#[derive(Event)]
pub struct ActivateTileEvent(Entity);

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct CloseTile;

#[sprite_sheet(count = 9, path = "forest-ground.png")]
pub struct ForestAssets {}

#[sprite_sheet(count = 4, path = "tree.png")]
pub struct TreeAssets {}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(ForestAssets::load(&asset_server, &mut texture_atlases))
}

fn setup(mut commands: Commands, tile_assets: Res<ForestAssets>) {
    commands.spawn((
        Health(20.),
        EntityType::Wall,
        AttackableFrom(vec![EntityType::Mob]),
        RigidBody::Static,
        Collider::rectangle(100., 100.),
        CollisionLayers::new([PhysicsLayers::Mob, PhysicsLayers::Entity], LayerMask::ALL),
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

    for x in -2..=2isize {
        for y in -2..=2isize {
            if x.abs() == 2 || y.abs() == 2 {
                closed_tile(&mut commands, x, y);
            } else {
                open_tile(
                    &mut commands,
                    x,
                    y,
                    &tile_assets.layout,
                    &tile_assets.texture,
                );
            }
        }
    }
}

fn open_tile(
    commands: &mut Commands,
    x: isize,
    y: isize,
    layout_handle: &Handle<TextureAtlasLayout>,
    image_handle: &Handle<Image>,
) {
    let mut rng = thread_rng();

    commands
        .spawn((
            Tile,
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    -5.,
                )),
                ..default()
            },
        ))
        .with_children(|parent| {
            let sub_tile_size = TILE_SIZE / SUB_TILES;

            for x in 0..SUB_TILES as i32 {
                for y in 0..SUB_TILES as i32 {
                    //offset so 0/0 is in the center of the tile
                    let x = x as f32 - SUB_TILES / 2.;
                    let y = y as f32 - SUB_TILES / 2.;

                    parent.spawn(SpriteSheetBundle {
                        atlas: TextureAtlas {
                            layout: layout_handle.clone(),
                            index: rng.gen_range(0..GROUND_SPRITE_COUNT),
                        },
                        texture: image_handle.clone(),
                        sprite: Sprite {
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

fn closed_tile(commands: &mut Commands, x: isize, y: isize) {
    commands.spawn((
        Tile,
        CloseTile,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.2, 0.1),
                custom_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                -5.,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(TILE_SIZE, TILE_SIZE),
        CollisionLayers::new(PhysicsLayers::ClosedTile, LayerMask::ALL),
        Restitution::new(0.),
    ));
}

fn hover_tile(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    spatial_query: SpatialQuery,
    mut hover_tile_event: EventWriter<HoverTileEvent>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let entities = spatial_query.point_intersections(
            cursor_position,
            SpatialQueryFilter::from_mask([PhysicsLayers::ClosedTile]),
        );

        for tile in entities {
            hover_tile_event.send(HoverTileEvent(tile));
        }
    }
}

fn highlight_hovered_tiles(
    mut hover_tile_event: EventReader<HoverTileEvent>,
    mut closed_tiles: Query<(Entity, &mut Sprite), With<CloseTile>>,
) {
    let hovered_entities = hover_tile_event.read().map(|event| event.0).collect_vec();

    for (entity, mut sprite) in closed_tiles.iter_mut() {
        if hovered_entities.contains(&entity) {
            sprite.color = Color::GOLD
        } else {
            sprite.color = Color::RED
        }
    }
}

fn activate_hovered_tiles(
    mut hover_tile_event: EventReader<HoverTileEvent>,
    mut activate_tile_event: EventWriter<ActivateTileEvent>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        hover_tile_event.clear();
        return;
    };

    hover_tile_event.read().for_each(|event| {
        activate_tile_event.send(ActivateTileEvent(event.0));
    });
}

fn activate_tiles(
    mut commands: Commands,
    mut activate_tile_event: EventReader<ActivateTileEvent>,
    closed_tiles: Query<&Transform, With<CloseTile>>,
    tile_assets: Res<ForestAssets>,
) {
    activate_tile_event
        .read()
        .map(|event| event.0)
        .for_each(|tile| {
            let transform = closed_tiles
                .get(tile)
                .expect("tried to delete Tile without transform");

            let grid_pos = transform.translation.xy() / TILE_SIZE;

            commands.entity(tile).despawn();
            open_tile(
                &mut commands,
                grid_pos.x as isize,
                grid_pos.y as isize,
                &tile_assets.layout,
                &tile_assets.texture,
            )
        })
}

fn create_surrounding_tiles(
    mut commands: Commands,
    mut activate_tile_event: EventReader<ActivateTileEvent>,
    tiles: Query<&Transform, With<Tile>>,
) {
    if activate_tile_event.is_empty() {
        return;
    }

    let mut filled_positions = tiles
        .iter()
        .map(|tile| {
            (
                (tile.translation.x / TILE_SIZE) as isize,
                (tile.translation.y / TILE_SIZE) as isize,
            )
        })
        .collect_vec();

    for event in activate_tile_event.read() {
        let tile = tiles.get(event.0).unwrap();

        let grid_pos = tile.translation.xy() / TILE_SIZE;

        for x in -1..=1 {
            for y in -1..=1 {
                let off_set_pos = (
                    (grid_pos.x + x as f32) as isize,
                    (grid_pos.y + y as f32) as isize,
                );

                if !filled_positions.contains(&off_set_pos) {
                    filled_positions.push(off_set_pos);
                    closed_tile(&mut commands, off_set_pos.0, off_set_pos.1)
                }
            }
        }
    }
}
