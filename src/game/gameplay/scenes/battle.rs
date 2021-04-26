use bevy::{prelude::*, render::camera::RenderLayers};

use crate::game::{camera::CustomOrthographicCameraBundle, gameplay::{character::Character, enemy::create_battle_enemy, stats::Health}, helpers::z_index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleLocation {
    Mountains,
}

#[derive(Debug, Clone, Copy)]
pub struct BattleEvent {
    pub battle_location: BattleLocation,
    pub enemy_entity: Entity,
}

pub struct BattleView {
    pub entity: Entity,
    pub enemies: Vec<Entity>,
    pub ui_entity: Entity,
}

pub fn get_battle_location_texture(battle_location: BattleLocation) -> &'static str {
    match battle_location {
        BattleLocation::Mountains => "textures/backgrounds/battle1.png",
    }
}

pub struct HealthUI {
    entity: Entity,
}

pub fn handle_battle_events(
    mut battle_events: EventReader<BattleEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    character_query: Query<Entity, With<Character>>,
) {
    for event in battle_events.iter() {
        let texture_handle: Handle<Texture> =
            asset_server.load(get_battle_location_texture(event.battle_location));
        let background_sprite = materials.add(texture_handle.into());

        // Get character entity..
        let character_entity = character_query.single().unwrap();

        let battle_entity = commands
            .spawn()
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|child_builder| {
                child_builder.spawn_bundle(UiCameraBundle::default());
                child_builder
                    .spawn()
                    .insert_bundle(CustomOrthographicCameraBundle::new_2d_with_size(Some(
                        Vec2::new(1920.0, 1080.0),
                    )))
                    .insert(RenderLayers::layer(1));

                child_builder.spawn().insert_bundle(SpriteBundle {
                    material: background_sprite,
                    transform: Transform::from_xyz(0.0, 0.0, z_index::BACKGROUND),
                    ..Default::default()
                }).insert(RenderLayers::layer(1));

                // TODO: Spawn more enemy sprites.
                for _ in 0..1 {
                    create_battle_enemy(
                        event.enemy_entity,
                        child_builder,
                        &asset_server,
                        &mut texture_atlases,
                    );
                }
                
            })
            .id();

        let ui_entity = commands.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(5.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    // Use the `Text::with_section` constructor
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        "Health: ",
                        TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(HealthUI {
                    entity: character_entity,
                });
        }).id();

        // TODO: Find a list of points on the battlefield for party members to occupy on the left side of the screen.
        // And use a query to move them there.

        commands.insert_resource(BattleView {
            entity: battle_entity,
            enemies: vec![event.enemy_entity],
            ui_entity
        });
    }
}

pub fn update_health_text(
    mut ui_query: Query<(&HealthUI, &mut Text)>,
    health_query: Query<&Health>,
) {
    for (health_ui, mut text) in ui_query.iter_mut() {
        if let Ok(health_component) = health_query.get(health_ui.entity) {
            text.sections[0].value = format!("Health: {}", health_component.value);
        }
    }
}


pub fn clear_battle_screen(mut commands: Commands, battle_view: Res<BattleView>) {
    commands.entity(battle_view.entity).despawn_recursive();
}
