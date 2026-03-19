use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score:u32,
    current_score:u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer
}

impl Default for GameState {
    fn default() -> Self {    
        Self{
            high_score:0,
            current_score:0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once)
        }
    }
}

fn main() {

    let mut game = Game::new();

    let player1 = game.add_sprite("player 1", SpritePreset::RacingCarRed);
    player1.translation = Vec2::new(-300.0, 0.0);
    player1.scale = 0.9;
    player1.collision = true;
    let yellow_car = game.add_sprite("yellow_car", SpritePreset::RacingCarYellow);
    yellow_car.translation = Vec2::new(100.0, 0.0);
    yellow_car.scale = 0.9;
    yellow_car.collision = true;


    game.add_logic(game_logic);

    game.run(GameState::default());
}


fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    
for event in engine.collision_events.drain(..) {
    if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
        for label in [event.pair.0, event.pair.1] {
            if label != "player 1" {
                engine.sprites.remove(&label);
            }
        }

        game_state.current_score += 10;
        println!("current score:  {}", game_state.current_score)
    }
    
}
//movement

let player = engine.sprites.get_mut("player 1").unwrap();
const MOVEMENT_SPEED: f32 = 100.0;
if engine
.keyboard_state
.pressed_any(&[KeyCode::ArrowUp, KeyCode::KeyW]) {
    player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
} 

else if engine
.keyboard_state
.pressed_any(&[KeyCode::ArrowDown, KeyCode::KeyS]) {
    player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
}

else if engine
.keyboard_state
.pressed_any(&[KeyCode::ArrowLeft, KeyCode::KeyA]) {
    player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
}

else if engine
.keyboard_state
.pressed_any(&[KeyCode::ArrowRight, KeyCode::KeyD]) {
    player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
}

    
}
