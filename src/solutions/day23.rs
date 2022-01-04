use itertools::Itertools;
use Ord;

#[derive(Clone, Default, Debug)]
struct Room {
    elems: Vec<i8>,
    size: usize,
    target_elem: i8,
}

#[derive(Clone, Default, Debug)]
struct Hallway {
    elems: [i8; 11],
}

#[derive(Clone, Default, Debug)]
struct State {
    hallway: Hallway,
    rooms: [Room; 4],
    energy: u64,
}

impl Room {
    fn can_enter(&self) -> bool {
        for elem in &self.elems {
            if *elem != self.target_elem {
                return false;
            }
        }
        true
    }

    fn is_final(&self) -> bool {
        if self.elems.len() < self.size {
            return false;
        }
        for elem in &self.elems {
            if *elem as usize != self.target_elem as usize {
                return false;
            }
        }
        true
    }
}

impl Hallway {
    fn has_path(&self, from: usize, to: usize) -> bool {
        if from <= to {
            for pos in (from + 1)..=to {
                if self.elems[pos] >= 0 {
                    return false;
                }
            }
        } else {
            for pos in to..from {
                if self.elems[pos] >= 0 {
                    return false;
                }
            }
        }
        true
    }
}

impl State {
    fn make_state(input: &[i8], expand: bool) -> Self {
        let mut state = State::default();
        state.hallway.elems.iter_mut().for_each(|e| *e = -1);
        input.iter().enumerate().for_each(|(i, elem)| {
            state.rooms[i % 4].elems.push(*elem);
        });
        if expand {
            state.rooms[0].elems.splice(1..1, [3, 3]);
            state.rooms[1].elems.splice(1..1, [2, 1]);
            state.rooms[2].elems.splice(1..1, [1, 0]);
            state.rooms[3].elems.splice(1..1, [0, 2]);
        }
        state.rooms.iter_mut().enumerate().for_each(|(i, room)| {
            room.size = room.elems.len();
            room.target_elem = i as i8;
            room.elems.reverse();
        });

        state
    }

    fn is_final(&self) -> bool {
        for room in &self.rooms {
            if !room.is_final() {
                return false;
            }
        }
        true
    }

    fn room_pos_in_hallway(&self, room_index: usize) -> usize {
        2 * (room_index + 1)
    }

    fn has_path_from_room_to_hallway(&self, room_index: usize, hallway_pos: usize) -> bool {
        hallway_pos != 2
            && hallway_pos != 4
            && hallway_pos != 6
            && hallway_pos != 8
            && self
                .hallway
                .has_path(self.room_pos_in_hallway(room_index), hallway_pos)
    }

    fn has_path_from_hallway_to_room(&self, hallway_pos: usize, room_index: usize) -> bool {
        self.rooms[room_index].can_enter()
            && self
                .hallway
                .has_path(hallway_pos, self.room_pos_in_hallway(room_index))
    }

    fn has_path_from_room_to_room(&self, from_room_index: usize, to_room_index: usize) -> bool {
        self.rooms[to_room_index].can_enter()
            && self.hallway.has_path(
                self.room_pos_in_hallway(from_room_index),
                self.room_pos_in_hallway(to_room_index),
            )
    }

    fn move_from_room_to_hallway(&self, room_index: usize, hallway_pos: usize) -> Self {
        let steps = ((self.rooms[room_index].size - self.rooms[room_index].elems.len() + 1) as i32
            + (hallway_pos as i32 - self.room_pos_in_hallway(room_index) as i32).abs())
            as u64;
        let mut new_state = self.clone();
        let elem = new_state.rooms[room_index].elems.pop().unwrap();
        new_state.hallway.elems[hallway_pos] = elem;
        new_state.energy = self.energy + energy_multipler(elem, steps);
        new_state
    }

    fn move_from_hallway_to_room(&self, hallway_pos: usize, room_index: usize) -> Self {
        let steps = ((hallway_pos as i32 - self.room_pos_in_hallway(room_index) as i32).abs()
            + (self.rooms[room_index].size - self.rooms[room_index].elems.len()) as i32)
            as u64;
        let mut new_state = self.clone();
        let elem = new_state.hallway.elems[hallway_pos];
        new_state.rooms[room_index].elems.push(elem);
        new_state.hallway.elems[hallway_pos] = -1;
        new_state.energy = self.energy + energy_multipler(elem, steps);
        new_state
    }

    fn move_from_room_to_room(&self, from_room_index: usize, to_room_index: usize) -> Self {
        let steps = ((self.rooms[from_room_index].size - self.rooms[from_room_index].elems.len()
            + 1) as i32
            + (self.room_pos_in_hallway(from_room_index) as i32
                - self.room_pos_in_hallway(to_room_index) as i32)
                .abs()
            + (self.rooms[to_room_index].size - self.rooms[to_room_index].elems.len()) as i32)
            as u64;
        let mut new_state = self.clone();
        let elem = new_state.rooms[from_room_index].elems.pop().unwrap();
        new_state.rooms[to_room_index].elems.push(elem);
        new_state.energy = self.energy + energy_multipler(elem, steps);
        new_state
    }

    fn try_move_any_into_room(&self) -> Option<Self> {
        for (i, elem) in self.hallway.elems.iter().enumerate() {
            if *elem >= 0 {
                if self.has_path_from_hallway_to_room(i, *elem as usize) {
                    return Some(self.move_from_hallway_to_room(i, *elem as usize));
                }
            }
        }
        for (i, room) in self.rooms.iter().enumerate() {
            if !room.can_enter() {
                let elem = room.elems.last().unwrap();
                if self.has_path_from_room_to_room(i, *elem as usize) {
                    return Some(self.move_from_room_to_room(i, *elem as usize));
                }
            }
        }
        None
    }

    fn try_move_from_room_to_hallway(&self, room_index: usize, hallway_pos: usize) -> Option<Self> {
        if self.has_path_from_room_to_hallway(room_index, hallway_pos) {
            Some(self.move_from_room_to_hallway(room_index, hallway_pos))
        } else {
            None
        }
    }

    fn lowest_energy(&self) -> Option<u64> {
        if self.is_final() {
            return Some(self.energy);
        }

        if let Some(new_state) = self.try_move_any_into_room() {
            return new_state.lowest_energy();
        }

        let mut min_energy = u64::MAX;
        for (room_index, room) in self.rooms.iter().enumerate() {
            if !room.can_enter() {
                for hallway_pos in 0..self.hallway.elems.len() {
                    if let Some(next_state) =
                        self.try_move_from_room_to_hallway(room_index, hallway_pos)
                    {
                        if let Some(new_energy) = next_state.lowest_energy() {
                            min_energy = Ord::min(min_energy, new_energy);
                        }
                    }
                }
            }
        }

        if min_energy < u64::MAX {
            Some(min_energy)
        } else {
            None
        }
    }
}

fn energy_multipler(elem: i8, steps: u64) -> u64 {
    match elem {
        0 => steps,
        1 => 10 * steps,
        2 => 100 * steps,
        _ => {
            assert!(elem == 3);
            1000 * steps
        }
    }
}

fn part1(input: &[i8]) -> u64 {
    let state = State::make_state(input, false);
    if let Some(output) = state.lowest_energy() {
        output
    } else {
        assert!(false);
        0
    }
}

fn part2(input: &[i8]) -> u64 {
    let state = State::make_state(input, true);
    if let Some(output) = state.lowest_energy() {
        output
    } else {
        assert!(false);
        0
    }
}

pub fn solve(input: String) {
    let input = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_uppercase() {
                Some(c as i8 - 'A' as i8)
            } else {
                None
            }
        })
        .collect_vec();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
