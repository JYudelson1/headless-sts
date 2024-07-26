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
        if x < 0.45 {
            Self::Monster
        } else if x < 0.67 {
            Self::Event
        } else if x < 0.83 {
            Self::Elite
        } else if x < 0.95 {
            Self::Rest
        } else {
            Self::Merchant
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct RoomNode {
    pub floor: usize,
    pub x: usize,
}

impl RoomNode {
    pub fn new(floor: usize, x: usize) -> Self {
        Self { floor, x }
    }

    pub fn get_next(&self, paths: [[bool; 19]; 14]) -> Vec<RoomNode> {
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

    pub fn get_paths(&self) -> Vec<(usize, usize)> {
        if self.x == 0 {
            vec![(0, 0), (0, 1)]
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
    boss: Boss,
    current: Option<RoomNode>,
}

impl Map {
    pub fn go_to_room(&mut self, room: RoomNode) {
        // TODO: Figure out how to represent bosses
        match self.current {
            Some(room) => {
                for next_room in room.get_next(self.paths) {
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
        for _ in 0..15 {
            if !map.fix_rooms(ascension) {
                break;
            }
        }
        if map.fix_rooms(ascension) {
            println!("Original map doesn't work, retrying...");
            return Self::new(act, ascension);
        }

        map
    }

    fn add_one_path(&mut self, starting_room: RoomNode, ascension: u8) -> RoomNode {
        let possible = starting_room.get_paths();
        let index = number_between(0, possible.len() - 1);
        let x = possible[index].0;
        let path_is_on_index = possible[index].1;
        let path_is_on = self.paths[starting_room.floor][path_is_on_index];
        if !path_is_on {
            // Set the starting room type
            // If the floor is 8, a treasure room
            // If 13, a rest site
            // Else, randomize

            self.rooms[starting_room.floor + 1][x] = if x == 8 {
                Some(RoomType::Treasure)
            } else if x == 14 {
                Some(RoomType::Rest)
            } else {
                Some(RoomType::random(ascension))
            };

            // Make the path
            self.paths[starting_room.floor][path_is_on_index] = true;
            
        }
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
        for room in 0..7 {
            if let Some(r) = &mut self.rooms[13][room] {
                while matches!(r, RoomType::Rest) {
                    *r = RoomType::random(ascension);
                    changed = true;
                }
            }
        }

        // Rule 3: Elite, Merchant, and Rests cannot be consecutive
        for floor in 0..14 {
            for x in 0..7 {
                match self.rooms[floor][x] {
                    Some(room) => {
                        match room {
                            RoomType::Elite => (),
                            RoomType::Rest => (),
                            RoomType::Merchant => (),
                            _ => continue,
                        }
                        let next = RoomNode::new(floor, x).get_next(self.paths);
                        for next_room in next {
                            while self.get_room(next_room) == Some(room) {
                                self.rooms[next_room.floor][next_room.x] =Some(RoomType::random(ascension));
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
        for floor in 0..14 {
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
        self.rooms[room.floor][room.x]
    }

    pub fn next_rooms(&self) -> Vec<RoomNode> {
        // TODO: Account for winged boots
        if let Some(current) = self.current {
            current.get_next(self.paths)
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
}
