#![allow(unused_results)]
use std::{collections::HashSet, fmt::Display};

use crate::utils::{number_between, Act};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RoomType {
    Monster,
    Event,
    Elite,
    Rest,
    Merchant,
    Treasure,
    Boss,
}

impl RoomType {
    pub fn random(ascension: u8) -> Self {
        let x = rand::random::<f32>();

        // TODO: Changed during ascensions 1+
        if x < 0.05 { 
            Self::Merchant
        } else if x < 0.27 { 
            Self::Event
        } else if x < 0.39 { 
            Self::Rest
        } else if x < (if ascension >= 2 {0.64} else {0.55}) {
            Self::Elite
        } else { // .45
            Self::Monster
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Act1Boss {
    SlimeBoss,
    Guardian,
    Hexaghost,
}

#[derive(Copy, Clone, Debug)]
pub enum Act2Boss {
    Champ,
    Collector,
    Automaton,
}

#[derive(Copy, Clone, Debug)]
pub enum Act3Boss {
    AwakenedOne,
    DonuAndDeca,
    TimeEater,
}

#[derive(Copy, Clone, Debug)]
pub enum Boss {
    Act1(Act1Boss),
    Act2(Act2Boss),
    Act3(Act3Boss),
}

impl Boss {
    pub fn get_random(act: Act) -> Self {
        let bosses = match act {
            Act::Act1 => [
                Self::Act1(Act1Boss::Guardian),
                Self::Act1(Act1Boss::Hexaghost),
                Self::Act1(Act1Boss::SlimeBoss),
            ],
            Act::Act2 => [
                Self::Act2(Act2Boss::Automaton),
                Self::Act2(Act2Boss::Champ),
                Self::Act2(Act2Boss::Collector),
            ],
            Act::Act3 => [
                Self::Act3(Act3Boss::AwakenedOne),
                Self::Act3(Act3Boss::DonuAndDeca),
                Self::Act3(Act3Boss::TimeEater),
            ],
        };

        bosses[number_between(0, 2)]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct RoomNode {
    pub floor: usize,
    pub x: usize,
}

impl RoomNode {
    pub fn new(floor: usize, x: usize) -> Self {
        Self { floor, x }
    }

    pub fn get_next(&self, paths: [[bool; 19]; 14]) -> Vec<RoomNode> {
        if self.floor == 14 {
            return vec![RoomNode { floor: 15, x: 0 }];
        }
        let floor_paths = paths[self.floor];
        let mut next = vec![];

        for (room_x, path) in self.get_paths() {
            if floor_paths[path] {
                next.push(RoomNode {
                    floor: self.floor + 1,
                    x: room_x,
                })
            }
        }

        next
    }

    pub fn get_prev(&self, paths: [[bool; 19]; 14]) -> Vec<RoomNode> {
        let mut prev = vec![];

        for i in 0..7 {
            let last_floor_node = RoomNode {
                floor: self.floor - 1,
                x: i,
            };
            for node in last_floor_node.get_next(paths) {
                if node.x == self.x {
                    prev.push(last_floor_node)
                }
            }
        }

        prev
    }

    pub fn get_paths(&self) -> Vec<(usize, usize)> {
        if self.x == 0 {
            vec![(0, 0), (1, 1)]
        } else if self.x < 6 {
            vec![
                (self.x - 1, 3 * self.x - 1),
                (self.x, 3 * self.x),
                (self.x + 1, 3 * self.x + 1),
            ]
        } else {
            vec![(5, 17), (6, 18)]
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    paths: [[bool; 19]; 14],
    rooms: [[Option<RoomType>; 7]; 15],
    pub boss: Boss,
    current: Option<RoomNode>,
}

impl Map {
    pub fn go_to_room(&mut self, room: RoomNode) {
        // TODO: Figure out how to represent bosses
        match self.current {
            Some(current_room) => {
                for next_room in current_room.get_next(self.paths) {
                    if next_room == room {
                        self.current = Some(next_room);
                        return;
                    }
                }
            }
            None => {
                for (i, next_room) in self.rooms[0].iter().enumerate() {
                    match next_room {
                        Some(_) => {
                            if room.x == i {
                                self.current = Some(room);
                                return;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
    }

    pub fn new(act: Act, ascension: u8) -> Self {
        let mut map = Self {
            paths: [[false; 19]; 14],
            rooms: [[None; 7]; 15],
            boss: Boss::get_random(act),
            current: None,
        };

        map.make_paths(ascension);
        //println!("first:\n{map}");

        map.remove_redundant_floor_1();
        //println!("After remove redundant:\n{map}");

        for _ in 0..15 {
            if !map.fix_rooms(ascension) {
                break;
            }
        }
        if map.fix_rooms(ascension) {
            //println!("Original map doesn't work, retrying...");
            return Self::new(act, ascension);
        }

        map
    }

    fn add_one_path(&mut self, starting_room: RoomNode, ascension: u8) -> RoomNode {
        let possible = starting_room.get_paths();

        let (x, path_is_on_index) = loop {
            let index = number_between(0, possible.len() - 1);
            let x = possible[index].0;
            let path_is_on_index = possible[index].1;

            // If the corresponding "crossing path" is off:
            let crosses_another = ((path_is_on_index % 3 == 1)
                && self.paths[starting_room.floor][path_is_on_index + 1])
                || ((path_is_on_index % 3 == 2)
                    && self.paths[starting_room.floor][path_is_on_index - 1]);

            if !crosses_another {
                break (x, path_is_on_index);
            }
        };

        //let path_is_on = self.paths[starting_room.floor][path_is_on_index];

        // Set the starting room type
        // If the floor is 8, a treasure room
        // If 13, a rest site
        // Else, randomize

        self.rooms[starting_room.floor + 1][x] = if starting_room.floor + 1 == 8 {
            Some(RoomType::Treasure)
        } else if starting_room.floor + 1 == 14 {
            Some(RoomType::Rest)
        } else {
            Some(RoomType::random(ascension))
        };

        // Make the path
        self.paths[starting_room.floor][path_is_on_index] = true;

        return RoomNode {
            floor: starting_room.floor + 1,
            x,
        };
    }

    fn add_one_full_path(&mut self, mut starting_room: RoomNode, ascension: u8) {
        for _ in 0..14 {
            starting_room = self.add_one_path(starting_room, ascension);
        }
    }

    fn random_starting_room(&self) -> RoomNode {
        RoomNode {
            floor: 0,
            x: number_between(0, 6),
        }
    }

    fn make_paths(&mut self, ascension: u8) {
        let first_starting_room = self.random_starting_room();

        for i in 0..6 {
            let starting_room = if i == 0 {
                first_starting_room
            } else if i == 1 {
                loop {
                    let s = self.random_starting_room();
                    if s != first_starting_room {
                        break s;
                    }
                }
            } else {
                self.random_starting_room()
            };
            self.rooms[0][starting_room.x] = Some(RoomType::Monster);

            self.add_one_full_path(starting_room, ascension);
        }
    }

    fn fix_rooms(&mut self, ascension: u8) -> bool {
        // Returns true if any rooms were changed
        let mut changed = false;

        // Rule 1: No elites or rests before floor 5
        for floor in 0..5 {
            for room in 0..7 {
                let r = match &mut self.rooms[floor][room] {
                    Some(room) => room,
                    _ => continue,
                };
                while matches!(r, RoomType::Rest) || matches!(r, RoomType::Elite) {
                    *r = RoomType::random(ascension);
                    changed = true;
                }
            }
        }
        // Rule 2: Rests on floor 13
        // Taken care of previously

        // Rule 3: Elite, Merchant, and Rests cannot be consecutive
        for floor in (1..15).rev() {
            for x in 0..7 {
                match self.rooms[floor][x] {
                    Some(room) => {
                        match room {
                            RoomType::Elite => (),
                            RoomType::Rest => (),
                            RoomType::Merchant => (),
                            _ => continue,
                        }
                        let prev = RoomNode::new(floor, x).get_prev(self.paths);
                        for prev_room in prev {
                            while self.get_room(prev_room) == Some(room) {
                                self.rooms[prev_room.floor][prev_room.x] =
                                    Some(RoomType::random(ascension));
                                changed = true;
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        // Rule 4: Every room with options must have all unique options
        let mut rule4problems = vec![];
        for floor in 0..13 {
            // Skip floor 7, since all options will be chests
            if floor == 7 {
                continue;
            }
            for x in 0..7 {
                match self.rooms[floor][x] {
                    Some(_) => {
                        let next = RoomNode::new(floor, x).get_next(self.paths);
                        if next.len() == 2 {
                            if self.get_room(next[0]) == self.get_room(next[1]) {
                                rule4problems.push(next[1]);
                            }
                        } else if next.len() == 3 {
                            if self.get_room(next[0]) == self.get_room(next[1]) {
                                rule4problems.push(next[1]);
                            }
                            if self.get_room(next[0]) == self.get_room(next[2]) {
                                rule4problems.push(next[2]);
                            } else if self.get_room(next[2]) == self.get_room(next[1]) {
                                rule4problems.push(next[2]);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
        if !rule4problems.is_empty() {
            changed = true
        }
        self.fix_problematic_rooms(rule4problems, ascension);

        changed
    }

    fn fix_problematic_rooms(&mut self, problems: Vec<RoomNode>, ascension: u8) {
        for problem in problems {
            let cannot_become = self.get_room(problem).unwrap();

            loop {
                let r = RoomType::random(ascension);
                if r != cannot_become {
                    self.rooms[problem.floor][problem.x] = Some(r);
                    break;
                }
            }
        }
    }

    pub fn get_room(&self, room: RoomNode) -> Option<RoomType> {
        if room.floor == 15 {
            Some(RoomType::Boss)
        } else {
            self.rooms[room.floor][room.x]
        }
    }

    pub fn next_rooms(&self) -> Vec<RoomNode> {
        // TODO: Account for winged boots
        if let Some(current) = self.current {
            // If at the boss, only give x = 0 
            if current.floor == 14 {
                vec![RoomNode { floor: 15, x: 0 }]
            } else {
                current.get_next(self.paths)
            }
            
        } else {
            let mut starters = vec![];
            for x in 0..7 {
                if let Some(_) = self.rooms[0][x] {
                    starters.push(RoomNode { floor: 0, x });
                }
            }
            starters
        }
    }

    pub fn next_floor_num(&self) -> usize {
        match self.current {
            Some(current) => current.floor + 1,
            None => 0,
        }
    }

    fn remove_redundant_floor_1(&mut self) {
        let mut second_floor_nodes = HashSet::new();

        for i in 0..7 {
            if let Some(_) = self.rooms[0][i] {
                let room = RoomNode { floor: 0, x: i };
                let next = room.get_paths();
                for (x, path_index) in next {
                    if self.paths[0][path_index] {
                        let room = RoomNode { floor: 1, x };
                        if second_floor_nodes.contains(&room) {
                            // The room is already a destination
                            // Remove the new edge to the room
                            self.paths[0][path_index] = false;
                            
                        } else {
                            second_floor_nodes.insert(room);
                        }
                    }
                }
            }
            let room = RoomNode { floor: 0, x: i };
            // If the first floor room has no more destinations,
            // remove it
            if room.get_next(self.paths).is_empty() {
                self.rooms[0][i] = None;
            }
        }
    }

    pub fn current_floor(&self) -> i8 {
        match self.current {
            Some(node) => node.floor as i8,
            None => -1,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Iterate backwards through the rooms
        for i in (0..15).rev() {
            // Draw the rooms
            let floor = self.rooms[i];
            for x in 0..7 {
                let room = floor[x];
                let letter = match room {
                    Some(room) => format!("{room}"),
                    None => " ".to_string(),
                };
                write!(f, "{letter} ").unwrap();
            }
            write!(f, "\n").unwrap();
            // Draw the paths
            if i == 0 {
                continue;
            }
            let paths = self.paths[i - 1];
            let mut path_string = "             ".to_string();
            for (i, path) in paths.iter().enumerate() {
                if !*path {
                    continue;
                }
                if i % 3 == 0 {
                    path_string.replace_range((2 * i / 3)..((2 * i / 3) + 1), "|");
                } else if i % 3 == 1 {
                    path_string
                        .replace_range(((2 * (i - 1) / 3) + 1)..((2 * (i - 1) / 3) + 2), "/");
                } else {
                    path_string.replace_range((((2 * i) - 1) / 3)..((((2 * i) - 1) / 3) + 1), "\\");
                }
            }
            write!(f, "{path_string}\n").unwrap();
        }
        std::fmt::Result::Ok(())
    }
}

impl Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            RoomType::Monster => "M",
            RoomType::Event => "?",
            RoomType::Elite => "E",
            RoomType::Rest => "R",
            RoomType::Merchant => "$",
            RoomType::Treasure => "T",
            RoomType::Boss => "B",
        };
        write!(f, "{letter}")
    }
}
