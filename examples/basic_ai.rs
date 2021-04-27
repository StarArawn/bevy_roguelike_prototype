use bevy::prelude::*;
use big_brain::prelude::*;

struct Player;

struct Health {
    value: f32,
}

#[derive(Debug, Clone)]
struct LowHealth;

impl LowHealth {
    fn build() -> LowHealthBuilder {
        LowHealthBuilder
    }
}

#[derive(Debug, Clone)]
struct LowHealthBuilder;

impl ScorerBuilder for LowHealthBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, _actor: Entity) {
        cmd.entity(scorer).insert(LowHealth);
    }
}

fn low_health_scorer_system(
    health_query: Query<&Health>,
    mut query: Query<(&Actor, &mut Score), With<LowHealth>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(health) = health_query.get(*actor) {
            score.set(1.0 - (health.value / 100.0));
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
struct DrinkPotion;

impl DrinkPotion {
    pub fn build() -> DrinkPotionBuilder {
        DrinkPotionBuilder
    }
}

#[derive(Debug, Clone)]
struct DrinkPotionBuilder;

impl ActionBuilder for DrinkPotionBuilder {
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        cmd.entity(action).insert(DrinkPotion);
    }
}

fn drink_potion_action_system(
    mut health: Query<&mut Health>,
    mut query: Query<(&Actor, &mut ActionState), With<DrinkPotion>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        if let Ok(mut health) = health.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    health.value = 100.0;
                    dbg!("Drank Potion");
                    *state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

#[derive(Default)]
struct LastUpdate {
    value: f64,
}

fn startup(
    mut commands: Commands,
) {
    commands.spawn()
        .insert(Player)
        .insert(Health {
            value: 100.0,
        })
        .insert(LastUpdate::default())
        .insert(
            Thinker::build()
                .picker(FirstToScore::new(0.8))
                .when(LowHealth::build(), DrinkPotion::build())
                .otherwise(Idle::build())
        );
}

fn damage_player_over_time(
    time: Res<Time>,
    mut query: Query<(&mut Health, &mut LastUpdate), With<Player>>,
) {
    let current_time = time.seconds_since_startup();
    for (mut health, mut last_update) in query.iter_mut() {
        if current_time - last_update.value > 1.0 {
            health.value -= 5.0;
            last_update.value = current_time;

            dbg!(health.value);
        }
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
        .add_system(damage_player_over_time.system())
        .add_system(low_health_scorer_system.system())
        .add_system(idle_system.system())
        .add_system(drink_potion_action_system.system())
        .run();
}
