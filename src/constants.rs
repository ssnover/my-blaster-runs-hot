//Player Constants
pub const PLAYER_SPRITE_SCALE: f32 = 2.0;
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_HEIGHT: f32 = 15.0;
pub const PLAYER_WIDTH: f32 = 10.0;
//Enemy Constants
pub const ENEMY_SPRITE_SCALE: f32 = 0.2;
pub const ENEMY_REPULSION_RADIUS: f32 = 10.0;
pub const ENEMY_REPULSION_FORCE: f32 = 1.0;
pub const PLAYER_ATTRACTION_FORCE: f32 = 3.0;
pub const PLAYER_HEALTH: u32 = 3;
pub const PLAYER_LIVES: u32 = 3;
pub const ENEMY_SPEED: f32 = 100.0;

//Blaster Constants
pub const BLASTER_SHOT_HEAT_ADDITION: f32 = 5.;
pub const BLASTER_POWER_SHOT_THRESHOLD: f32 = 90.;
pub const MAX_BLASTER_HEAT: f32 = 100.;
pub const BLASTER_COOLOFF_MULTIPLIER: f32 = 3.5;
pub const COOLDOWN_TIME_SECONDS: f32 = 5.;
pub const BLASTER_SPEED: f32 = 200.0;

//Other Constants
pub const TIME_STEP: f32 = 1. / 60.;
pub const KNOCKBACK_POWER: f32 = 500.0;

//Collision Group Flags
pub const PLAYER_GROUP: u32 = 0b1;
pub const ENEMY_GROUP: u32 = 0b10;
pub const CIVILIAN_GROUP: u32 = 0b100;
pub const BLASTER_GROUP: u32 = 0b1000;
pub const PHYSICAL_GROUP: u32 = 0b10000;
