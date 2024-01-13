use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Player {
    pub(crate) id: u32,
    pub(crate) tft_chance: f64,
    pub(crate) points: u32,
    pub(crate) last_opponent_move: Choice,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Choice{
    StayQuiet,
    Talk
}


impl Player {
    pub(crate) fn new(id: u32, tft_chance: f64) -> Player {
        Player {
            id,
            tft_chance,
            points: 0,
            last_opponent_move: Choice::StayQuiet,
        }
    }

    pub(crate) fn get_id(&self) -> u32 {
        self.id
    }

    pub(crate) fn get_points(&self) -> u32 {
        self.points
    }

    pub(crate) fn get_last_opponent_move(&self) -> Choice {
        self.last_opponent_move
    }

    pub(crate) fn play(&mut self) -> Choice {
        if rand::thread_rng().gen_range(0.0..=1.0) < self.tft_chance {
            match self.last_opponent_move {
                Choice::Talk => Choice::Talk,
                Choice::StayQuiet => Choice::StayQuiet,
            }
        } else {
            Choice::Talk
        }
    }

    pub(crate) fn update_last_opponent_move(&mut self, last_opponent_move: Choice) {
        self.last_opponent_move = last_opponent_move;
    }

    pub(crate) fn add_points(&mut self, points: u32) {
        self.points += points;
    }
}