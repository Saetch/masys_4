use std::{cell::{RefCell, RefMut}, thread, time::Duration, fs::{File, OpenOptions}, io::Write};

use player::{Player, Choice};
use rand::seq::SliceRandom;


mod player;


fn main() {
    const n: u32 = 6;

    let mut players: Vec<RefCell<Player>> = Vec::new();
    for i in 1..n {
        let player = Player::new(i, 0.0);
        players.push(RefCell::new(player));
    }


    let special_player = Player::new(n, 0.95);
    players.push(RefCell::new(special_player));

    const AMOUNT_FOR_TEST: u32 = 10_000;
    const DRAWS: u32 = 10_000;
    let mut rng = rand::thread_rng();
    let mut result_array = [0; (n* DRAWS ) as usize];

    for _ in 0 .. AMOUNT_FOR_TEST{
        for i in 0..DRAWS {
            all_players_play(&mut players, &mut rng);
            players.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
            for player in players.iter() {
                let player = player.borrow();
                result_array[((player.id - 1) +i*n) as usize] += player.points;
            }
        }
    

        for player in players.iter() {
            let mut player = player.borrow_mut();
            player.points = 0;
            player.last_opponent_move = Choice::StayQuiet;
        }
    }
    let mut curr_draw = 1;
    let mut counter = 0;

    for i in 0..result_array.len() {


        if counter == 0 {
            println!("Draw {}:", curr_draw);

            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("output.txt") {
                let data_to_write = "Draw: ".to_string() + &curr_draw.to_string() + "\n";
        
                if let Err(err) = file.write_all(data_to_write.as_bytes()) {
                    eprintln!("Error writing to file: {}", err);
                }
            } else {
                eprintln!("Error creating or opening the file.");
            }
        }
        
        counter = counter + 1;
        
        if counter == n {
            curr_draw = curr_draw + 1;
            counter = 0;
        }
        if i as u32 %n == 0 || i as u32 %n == n-1 {

            if let Ok(mut file) = OpenOptions::new().append(true).open("output.txt") {
                
                
                if let Err(res) = writeln!(file, "Player {} has {} points after {} draws, average of {} points per game",  i as u32%n + 1, result_array[i as usize] as f64 / AMOUNT_FOR_TEST as f64, (i as u32 /n ) + 1, (result_array[i as usize] as f64 / AMOUNT_FOR_TEST as f64) / ((i as u32/n) + 1) as f64 ){
                    eprintln!("Error writing to file: {}", res);
                } 
            }
        }

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