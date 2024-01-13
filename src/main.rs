use std::{cell::{RefCell, RefMut}, thread, time::Duration};

use player::{Player, Choice};
use rand::seq::SliceRandom;


mod player;


fn main() {
    const N: u32 = 6;

    let mut players: Vec<RefCell<Player>> = Vec::new();
    for i in 1..N {
        let player = Player::new(i, 0.0);
        players.push(RefCell::new(player));
    }


    let special_player = Player::new(N, 0.95);
    players.push(RefCell::new(special_player));

    const AMOUNT_FOR_TEST: u32 = 1_000;
    const DRAWS: u32 = 10_000;
    let mut rng = rand::thread_rng();
    let mut result_array = [0.0; 2000];
    let step_size = 1 as f64 / 1000 as f64;
    for f in 0 .. 1000{
        let cur_chance = f as f64 * step_size;
        for player in players.iter() {
            let mut player = player.borrow_mut();
            if player.id != N {
                player.talk_chance = cur_chance;
            }
        }
        for _ in 0 .. AMOUNT_FOR_TEST{
            for _ in 0..DRAWS {
                all_players_play(&mut players, &mut rng);

            }
            let mut pointies: f32 = 0.0;
            let mut points_for_cheater = 0;
            players.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
            for player in players.iter() {
                let player = player.borrow();
                if player.id == N {
                    points_for_cheater = player.points;
                } else {
                    pointies += player.points as f32;
                }
            }
            pointies /= (N - 1) as f32;
            result_array[f*2] = pointies;
            result_array[f*2+1] = points_for_cheater as f32;
    
            for player in players.iter() {
                let mut player = player.borrow_mut();
                player.points = 0;
                player.last_opponent_move = Choice::StayQuiet;
            }
        }

        println!("{:.3}: {}  / {}", cur_chance, result_array[f*2] / DRAWS as f32, result_array[f*2+1] / DRAWS as f32);

    }





}


fn all_players_play(players: &mut Vec<RefCell<Player>>, rng: &mut rand::rngs::ThreadRng) {
    players.shuffle(rng);



    let mut _index = 0;
    for i in 0..players.len() / 2 {
        _index = i * 2;
        let mut player_1 = players[_index].borrow_mut();
        let mut player_2 = players[_index + 1].borrow_mut();
        play_round(&mut player_1, &mut player_2);
        
    }
}



#[inline(always)]
fn play_round(player1: &mut RefMut<'_, Player>, player2: &mut RefMut<'_, Player>) {
    let choice_1 = player1.play();
    let choice_2 = player2.play();

    //println!("Player {} chose {:?}, Player {} chose {:?}", player1.get_id(), choice_1, player2.get_id(), choice_2);
    //thread::sleep(Duration::from_millis(1000));
    if choice_1 == Choice::Talk {
        if choice_2 == choice_1 {
            player1.add_points(1);
            player2.add_points(1);
        } else {
            player1.add_points(5);
            player2.add_points(0);
        }
    }else{
        if choice_2 == choice_1 {
            player1.add_points(3);
            player2.add_points(3);
        } else {
            player1.add_points(0);
            player2.add_points(5);
        }
    }

    player1.update_last_opponent_move(choice_2);
    player2.update_last_opponent_move(choice_1);

}