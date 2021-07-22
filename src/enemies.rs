
use bevy::prelude::*;
use bevy::core::FixedTimestep;

use rand::prelude::*;

use crate::{SCALE_UP, Materials,  Direction,  Velocity, Gravity, 
    Player, StrikeBox, PlayerAction, Energy, Attacking, 
    Enemy, Proximity, WinSize, ENEMYSPEEDFAST, ENEMYSPEEDMED, ENEMYSPEEDSLOW, 
    ENEMYSPEEDSTOP};


pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "enemy", 
                SystemStage::single(enemy_spawn.system(),)
                )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(ai_enemy.system()),
                       )
            .add_system(scroll_enemy.system())
            .add_system(animate_enemy.system())
            .add_system(near_player.system())
            .add_system(control_enemy.system())
            .add_system(attacking_enemy.system());
    }
}


fn enemy_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    ){
    commands
        .spawn_bundle(SpriteSheetBundle{
            texture_atlas: materials.skely_one_sprite.clone(),
            transform: Transform{
                translation: Vec3::new(700.0, 0.0, 1.8),
                scale: Vec3::new(SCALE_UP, SCALE_UP, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
    .insert(Enemy{
        action: PlayerAction::Stand,
        direction: Direction::Right,
        vel_mod: ENEMYSPEEDSTOP,
    })
    .insert(Timer::from_seconds(0.1, true))
    .insert(Gravity{
        falling: false,
    })
    .insert(Velocity{
        velocity: Vec3::new(1.0, 0.0, 0.0),
    })
    .insert(Energy{
        power: 25, //skelly is low energy.
    })
    .insert(Proximity{
        near_player: false,
    })
    .insert(Attacking{
        attack: false,
    })
    .insert(StrikeBox{
        h: 40.0,
        w: 115.0,
        attack_h: 40.0,
        attack_w: 130.0,
    });

}

fn near_player(
    window: Res<WinSize>,
    mut q: QuerySet<(
        Query<&mut Transform, With<Enemy>>,
        Query<&mut Transform, With<Player>>,
        Query<&mut Proximity, With<Enemy>>,
        )>,
    ){
        let mut enemy_x: f32 = 0.0;
        let mut player_x: f32 = 0.0;
        for transform in q.q0_mut().iter_mut(){
            enemy_x = transform.translation.x; 
        }
        for transform in q.q1_mut().iter_mut(){
            player_x = transform.translation.x + (window.w / 2.0);
        }
        for mut proximity in q.q2_mut().iter_mut(){
            if enemy_x < player_x && enemy_x > -player_x{
                proximity.near_player = true;
            }else{
                proximity.near_player = false;
            } 
        }
}

fn animate_enemy(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer, &mut TextureAtlasSprite, 
        &Handle<TextureAtlas>, &mut Enemy, &mut Transform,
             )>,
             ){

    for(mut timer, mut sprite, texture_atlas_handle, mut enemy, transform) in query.iter_mut(){
        timer.tick(time.delta());
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            match enemy.action{
                PlayerAction::Walk => {
                    enemy.vel_mod = ENEMYSPEEDSLOW;
                    match sprite.index{
                        18 => sprite.index = 19,
                        19 => sprite.index = 20,
                        20 => sprite.index = 21,
                        _ => sprite.index = 18,
                    }
                }
                
                PlayerAction::Charge =>{
                    enemy.vel_mod = ENEMYSPEEDFAST;
                    match sprite.index{
                        14 => sprite.index = 15,
                        15 => sprite.index = 16,
                        16 => sprite.index = 17,
                        _ => sprite.index = 14,
                    } 
                    if sprite.index == 17{
                        enemy.vel_mod = ENEMYSPEEDSLOW;
                        enemy.action = PlayerAction::Walk;
                    }
                }

                PlayerAction::Chop =>{
                    match sprite.index{
                        27 => sprite.index = 28,
                        28 => sprite.index = 29,
                        29 => sprite.index = 30, 
                        _ => sprite.index = 27,
                    } 
                    if sprite.index == 30{
                        enemy.vel_mod = ENEMYSPEEDSTOP;
                        enemy.action = PlayerAction::Stand;
                    }
                }

                PlayerAction::Block =>{
                    enemy.vel_mod = ENEMYSPEEDSTOP;
                    match sprite.index{
                        23 => sprite.index = 24,
                        24 => sprite.index = 25,
                        25 => sprite.index = 26,
                        _ => sprite.index = 23,
                    }
                    if sprite.index == 26{
                        enemy.vel_mod = ENEMYSPEEDSTOP;
                        enemy.action = PlayerAction::Stand;
                    }
                }
                _ => sprite.index = 18,
            }
        }
    }
}

fn scroll_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform, &Velocity, &Proximity)>,
    mut player_query: Query<(&Player, &StrikeBox)>,
        
    ){
    if let Ok((player, strike_box)) = player_query.single_mut(){
        //what direction is player facing? If player moves, the whole 
        //game moves with him.  So enemy must scroll with him.
        let dir = match player.direction{
            Direction::Right => -1.0,
            Direction::Left => 1.0,
            _ => 0.0,
        };
        let mut vel = dir * player.vel_mod;
        for (mut enemy, mut transform, enemy_velocity, proximity) in enemy_query.iter_mut(){

            //once enemy is on screen, the enemy closes in on player and scrolling
            //stops. 
            if proximity.near_player{
                if transform.translation.x > player.current_x + (strike_box.w / 2.0){
                    vel += -1.0 * enemy.vel_mod;
                    //flip sprite 
                    transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                }else if transform.translation.x < player.current_x - (strike_box.w / 2.0){
                    vel += 1.0 * enemy.vel_mod;
                    //flip sprite
                    transform.rotation = Quat::default();
                }
                //animate walking, but only if standing.else probably doing some 
                //attack. 
                if enemy.action == PlayerAction::Stand{
                    enemy.action = PlayerAction::Walk;
                }

            }else{
                //once enemy is not on screen, player movement controls enemy
                vel = dir * player.vel_mod;
            }

            //multiplyer is the z value of the background. The further back
            //into the background, the slower the multiplier.
            let multiplier = transform.translation.z;
            transform.translation.x  += 
                (vel * multiplier) * enemy_velocity.velocity.x * time.delta_seconds();

        }
    }
}

fn ai_enemy(
    mut enemy_query: Query<(&mut Enemy, &Transform, &Energy, &Proximity)>,
    mut player_query: Query<(&Player)>,
    ){
    //create the random gen

    //this only goes every second.  See spawn fixedTimestep
    let mut rng = thread_rng();


    if let Ok(player) = player_query.single_mut(){
        for(mut enemy, enemy_transform, energy, proximity) in enemy_query.iter_mut(){
            //don't do anything unless we are on screen.
            //and if we are being hit by player (Bumped) take no action.
            if proximity.near_player{
                let mut distance = (player.current_x - enemy_transform.translation.x).abs();
                //possible range between 400 - 57 so divide by 50 to get 8 levels
                //of proximity. 8 will be farther away than 0.
                let imparative = distance as i32 / 50;
                //only acti if just walking
                if enemy.action == PlayerAction::Walk{
                    match imparative{
                        0 => {
                            let pick = rng.gen_range(0..2);
                            match pick{
                            0 => enemy.action = PlayerAction::Block,
                            1 => enemy.action = PlayerAction::Charge,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }

                        1 => {
                            let pick = rng.gen_range(0..2);
                            match pick{
                            0 => enemy.action = PlayerAction::Block,
                            1 => enemy.action = PlayerAction::Charge,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }

                        2 => {
                            let pick = rng.gen_range(0..2);
                            match pick{
                            0 => enemy.action = PlayerAction::Walk,
                            1 => enemy.action = PlayerAction::Chop,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }

                        3 => {
                            let pick = rng.gen_range(0..3);
                            match pick{
                            0 => enemy.action = PlayerAction::Charge,
                            1 => enemy.action = PlayerAction::Block,
                            2 => enemy.action = PlayerAction::Chop,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }

                        4 => {
                            let pick = rng.gen_range(0..3);
                            match pick{
                            0 => enemy.action = PlayerAction::Charge,
                            1 => enemy.action = PlayerAction::Charge,
                            2 => enemy.action = PlayerAction::Walk,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }
                        
                        5 => {
                            let pick = rng.gen_range(0..4);
                            match pick{
                            0 => enemy.action = PlayerAction::Chop,
                            1 => enemy.vel_mod = ENEMYSPEEDMED, 
                            2 => enemy.action = PlayerAction::Walk,
                            3 => enemy.vel_mod = ENEMYSPEEDFAST,
                            _ => enemy.action = PlayerAction::Walk,
                            }
                        }

                        _ => {
                            //nothing
                            }
                    }
                }
            }
        }
    }
}

fn attacking_enemy(
    mut enemy_query: Query<(&mut Enemy, &mut Attacking)>,
    ){
    for(enemy, mut attacking) in enemy_query.iter_mut(){
        match enemy.action{
            PlayerAction::Charge => attacking.attack = true,
            PlayerAction::Chop => attacking.attack = true,
            _ => attacking.attack = false,
        }
    }
}

fn control_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform, 
                      &mut Velocity, &mut Gravity)>,
    mut player_query: Query<(&Player, &mut Attacking)>,
    
    ){
    for(mut enemy, mut transform, mut enemy_velocity, 
       mut enemy_gravity) in enemy_query.iter_mut(){

        

        let delta_seconds = f32::min(0.3, time.delta_seconds());

        match enemy.action{
            PlayerAction::Bumped => {
                if let Ok((player, mut attacking)) = player_query.single_mut(){
                    attacking.attack = false; //turn off attack
                    transform.translation.y += 30.0;//get enemy off player 
                    enemy_velocity.velocity.y = 30.0;
                    enemy_gravity.falling = true;
                    if player.direction == Direction::Left{
                        transform.translation.x -= 40.0; //get it off platform
                    }else{
                        transform.translation.x += 40.0; //get it off platform
                    }
                }
                //turn off Bumped 
                enemy.action = PlayerAction::Stand;
            }
            _ =>{
                //nothing
            }
        }
    }
}
