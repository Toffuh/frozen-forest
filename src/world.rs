use crate::entities::data::{AttackableFrom, EntityType, Health};
use crate::PhysicsLayers;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
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
                    (
                        highlight_hovered_tiles,
                        (activate_hovered_tiles, activate_tiles).chain(),
                    ),
                )
                    .chain(),
            )
            .add_event::<HoverTileEvent>()
            .add_event::<ActivateTileEvent>();
    }
}

pub static TILE_SIZE: usize = 300;
pub static SUB_TILES: usize = 5;
pub static GROUND_TILE_COUNT: usize = 5;

#[derive(Event)]
pub struct HoverTileEvent(Entity);

#[derive(Event)]
pub struct ActivateTileEvent(Entity);

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct CloseTile;

#[derive(Resource)]
pub struct TileAssets {
    forest_tile_map: Handle<TextureAtlas>,
}

fn load_assets(
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

    commands.insert_resource(TileAssets {
        forest_tile_map: atlas_handle,
    })
}

fn setup(mut commands: Commands, tile_assets: Res<TileAssets>) {
    commands.spawn((
        Health(20.),
        EntityType::Wall,
        AttackableFrom(vec![EntityType::Mob]),
        RigidBody::Static,
        Collider::cuboid(100., 100.),
        CollisionLayers::all_masks::<PhysicsLayers>()
            .add_groups([PhysicsLayers::Mob, PhysicsLayers::Entity]),
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
                closed_tile(&mut commands, x, y);
            } else {
                open_tile(&mut commands, x, y, &tile_assets.forest_tile_map);
            }
        }
    }
}

fn open_tile(commands: &mut Commands, x: isize, y: isize, atlas_handle: &Handle<TextureAtlas>) {
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

fn closed_tile(commands: &mut Commands, x: isize, y: isize) {
    commands.spawn((
        Tile,
        CloseTile,
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
        CollisionLayers::all_masks::<PhysicsLayers>().add_group(PhysicsLayers::ClosedTile),
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
            SpatialQueryFilter::new().with_masks([PhysicsLayers::ClosedTile]),
        );

        for tile in entities {
            hover_tile_event.send(HoverTileEvent(tile))
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
    mouse_button_input: Res<Input<MouseButton>>,
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
    tile_assets: Res<TileAssets>,
) {
    activate_tile_event
        .read()
        .map(|event| event.0)
        .for_each(|tile| {
            let transform = closed_tiles
                .get(tile)
                .expect("tried to delete Tile without transform");

            let grid_pos = transform.translation.xy() / TILE_SIZE as f32;

            commands.entity(tile).despawn();
            open_tile(
                &mut commands,
                grid_pos.x as isize,
                grid_pos.y as isize,
                &tile_assets.forest_tile_map,
            )
        })
}
