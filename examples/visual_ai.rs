use bevy::{prelude::*};
use big_brain::prelude::*;
use rand::{thread_rng, Rng};

const HALF_WIDTH: f32 = (1270.0 / 2.0) - 32.0;
const HALF_HEIGHT: f32 = (720.0 / 2.0) - 32.0;

struct Moving;

#[derive(Default)]
struct Boredom {
    value: f32,
}

fn tick_boredom(
    time: Res<Time>,
    mut query: Query<&mut Boredom, Without<Moving>>,
) {
    for mut boredom in query.iter_mut() {
        boredom.value += 0.5 * time.delta_seconds();
        if boredom.value > 1.0 {
            boredom.value = 1.0;
        }
    }
}

#[derive(Debug, Clone)]
struct Idle;

impl Idle {
    fn build() -> IdleBuilder {
        IdleBuilder
    }
}

#[derive(Debug, Clone)]
struct IdleBuilder;

impl ActionBuilder for IdleBuilder {
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(Idle);
    }
}

fn idle_system(mut query: Query<&mut ActionState, With<Idle>>) {
    for mut state in query.iter_mut() {
        match *state {
            ActionState::Requested => {
                *state = ActionState::Executing;
            }
            ActionState::Cancelled => {
                *state = ActionState::Success;
            }
            ActionState::Executing => {}
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct IsBored;

impl IsBored {
    fn build() -> IsBoredBuilder {
        IsBoredBuilder
    }
}

#[derive(Debug, Clone)]
struct IsBoredBuilder;

impl ScorerBuilder for IsBoredBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(IsBored);
    }
}

fn is_bored_system(
    boredom_query: Query<&Boredom>,
    mut query: Query<(&Actor, &mut Score), With<IsBored>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(boredom) = boredom_query.get(*actor) {
            score.set(boredom.value);
        }
    }
}
#[derive(Default)]
struct Target {
    value: Vec2,
    potion: Option<Entity>,
}

#[derive(Debug, Clone)]
struct Move;

impl Move {
    fn build() -> MoveBuilder {
        MoveBuilder
    }
}

#[derive(Debug, Clone)]
struct MoveBuilder;

impl ActionBuilder for MoveBuilder {
    fn build(&self, cmd: &mut Commands, action: Entity, actor: Entity) {
        cmd.entity(actor).insert(Moving);
        cmd.entity(action).insert(Move);
    }
}

fn move_system(
    mut commands: Commands,
    time: Res<Time>,
    mut target_query: Query<&mut Target>,
    mut boredom_query: Query<&mut Boredom>,
    mut transform_query: Query<&mut Transform>,
    mut query: Query<(&Actor, &mut ActionState), With<Move>>
) {
    for (actor, mut state) in query.iter_mut() {
        match *state {
            ActionState::Requested => {
                if let Ok(mut target) = target_query.get_mut(actor.0) {
                    let mut random = thread_rng();
                    let random_target = Vec2::new(
                        random.gen_range(-HALF_WIDTH..HALF_WIDTH),
                        random.gen_range(-HALF_HEIGHT..HALF_HEIGHT),
                    );
                    if target.value == Vec2::ZERO {
                        target.value = random_target;
                    }
                }

                *state = ActionState::Executing;
            }
            ActionState::Cancelled => {
                *state = ActionState::Success;
            }
            ActionState::Executing => {
                if let Ok(mut target) = target_query.get_mut(actor.0) {
                    if let Ok(mut transform) = transform_query.get_mut(actor.0) {
                        let mut position = transform.translation.truncate();
                        let direction = (target.value - position).normalize();
                        position += direction * 100.0 * time.delta_seconds();
                        transform.translation = position.extend(transform.translation.z);

                        let distance = position.distance_squared(target.value);
                        if distance < 1.0 {
                            if let Ok(mut boredom) = boredom_query.get_mut(actor.0) {
                                boredom.value = 0.0;
                            }
                            commands.entity(actor.0).remove::<Moving>();
                            if let Some(potion_entity) = target.potion { 
                                commands.entity(potion_entity).despawn();
                            }
                            target.value = Vec2::ZERO;
                            target.potion = None;
                            *state = ActionState::Success;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

struct Potion;

#[derive(Debug, Clone)]
struct SeesPotion;

impl SeesPotion {
    fn build() -> SeesPotionBuilder {
        SeesPotionBuilder
    }
}

#[derive(Debug, Clone)]
struct SeesPotionBuilder;

impl ScorerBuilder for SeesPotionBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(SeesPotion);
    }
}

fn sees_potion_system(
    mut target_query: Query<&mut Target>,
    transform_query: Query<&Transform>,
    potion_query: Query<(Entity, &Transform), With<Potion>>,
    mut query: Query<(&Actor, &mut Score), With<SeesPotion>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(transform) = transform_query.get(*actor) {
            for (entity, potion_transform) in potion_query.iter() {
                let distance = transform.translation.distance_squared(potion_transform.translation);
                if distance < 4096.0 {
                    if let Ok(mut target) = target_query.get_mut(*actor) {
                        target.value = potion_transform.translation.truncate();
                        target.potion = Some(entity);

                        let new_score = (4096.0 / distance).min(1.0);

                        score.set(new_score);
                        return;
                    }
                }
            }
            score.set(0.0);
        }
    }
}


fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player_texture = asset_server.load("textures/player_sprite.png");
    let player_material = materials.add(player_texture.into());

    commands
        .spawn_bundle(SpriteBundle {
            material: player_material,
            ..SpriteBundle::default()
        })
        .insert(Boredom::default())
        .insert(Target::default())
        .insert(
            Thinker::build()
                .picker(FirstToScore::new(0.2))
                .when(IsBored::build(), Move::build())
                .when(SeesPotion::build(), Move::build())
                .otherwise(Idle::build())
        );

    let potion_texture = asset_server.load("textures/health_potion.png");
    let potion_material = materials.add(potion_texture.into());

    let mut random = thread_rng();

    for _ in 0..500 {
        let x = random.gen_range(-HALF_WIDTH..HALF_WIDTH);
        let y = random.gen_range(-HALF_HEIGHT..HALF_HEIGHT);
        commands.spawn_bundle(SpriteBundle {
            material: potion_material.clone(),
            transform: Transform::from_xyz(x, y, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Potion);
    }
}
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("basic_ai"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BigBrainPlugin)
        .add_startup_system(startup.system())
        .add_system(tick_boredom.system())
        .add_system(is_bored_system.system())
        .add_system(move_system.system())
        .add_system(idle_system.system())
        .add_system(sees_potion_system.system())
        .run();
}
