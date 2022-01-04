use std::collections::HashMap;

struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn go(&mut self, amount: u32) -> u32 {
        self.position = (self.position + amount) % 10;
        self.score += self.position + 1;
        self.score
    }
}

struct Game {
    player1: Player,
    player2: Player,
    dice_rolls: u32,
    dice_amount: u32,
    even_turn: bool,
}

impl Game {
    fn play(&mut self) -> u32 {
        loop {
            let mut rolled = 0;
            for _ in 0..3 {
                rolled += self.roll();
            }
            if self.even_turn {
                self.player2.go(rolled);
                if self.player2.score >= 1000 {
                    return self.player1.score * self.dice_rolls;
                }
                self.even_turn = false;
            } else {
                self.player1.go(rolled);
                if self.player1.score >= 1000 {
                    return self.player2.score * self.dice_rolls;
                }
                self.even_turn = true;
            }
        }
    }

    fn roll(&mut self) -> u32 {
        let amount_to_return = self.dice_amount + 1;
        self.dice_amount = (self.dice_amount + 1) % 100;
        self.dice_rolls += 1;
        amount_to_return
    }
}

fn part1(starts: &Vec<u32>) -> u32 {
    let mut game = Game {
        player1: Player {
            position: starts[0],
            score: 0,
        },
        player2: Player {
            position: starts[1],
            score: 0,
        },
        dice_rolls: 0,
        dice_amount: 0,
        even_turn: false,
    };
    game.play()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct QGameState {
    current_player_position: u32,
    next_player_position: u32,
    current_player_score: u32,
    next_player_score: u32,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct QGameOutput {
    current_player_wins: u64,
    next_player_wins: u64,
}

struct QGame {
    states: HashMap<QGameState, QGameOutput>,
}

impl QGame {
    fn play(&mut self, state: QGameState) -> QGameOutput {
        if let Some(scores) = self.states.get(&state) {
            return *scores;
        }
        let mut output = QGameOutput::default();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let new_pos = (state.current_player_position - 1 + i + j + k) % 10 + 1;
                    let new_score = state.current_player_score + new_pos;
                    if new_score >= 21 {
                        output.current_player_wins += 1;
                    } else {
                        let next_game_state = QGameState {
                            current_player_position: state.next_player_position,
                            next_player_position: new_pos,
                            current_player_score: state.next_player_score,
                            next_player_score: new_score,
                        };
                        let next_game_output = self.play(next_game_state);
                        output.current_player_wins += next_game_output.next_player_wins;
                        output.next_player_wins += next_game_output.current_player_wins;
                    }
                }
            }
        }
        self.states.insert(state, output);
        output
    }
}

fn part2(starts: &Vec<u32>) -> u64 {
    let mut game = QGame {
        states: HashMap::new(),
    };
    let initial_state = QGameState {
        current_player_position: starts[0] + 1,
        next_player_position: starts[1] + 1,
        current_player_score: 0,
        next_player_score: 0,
    };
    let game_output = game.play(initial_state);
    if game_output.current_player_wins >= game_output.next_player_wins {
        game_output.current_player_wins
    } else {
        game_output.next_player_wins
    }
}

pub fn solve(input: String) {
    let lines = input.lines();
    let starts = lines
        .map(|line| line.split(' ').last().unwrap().parse::<u32>().unwrap() - 1)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&starts));
    println!("Part 2: {}", part2(&starts));
}
