use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

/// Различните грешки, които ще очакваме да върнете като резултат от някои невалидни операции.
/// Повече детайли по-долу.
///
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

/// Четирите посоки, в които може една стая да има съседи. Може да добавите още trait
/// имплементации, за да си улесните живота.
///
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::South, Direction::East, Direction::West]
            .iter()
            .cloned()
    }
}

/// Една стая в подземията. Дефинира се само с име, макар че в по-интересна имплементация може да
/// държи item-и, противници...
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub connections: HashMap<Direction, String>,
}

/// Контейнер за стаите и не само. Ще работим предимно със тази структура.
///
pub struct Dungeon {
    pub rooms: HashMap<String, Rc<Room>>,
}

impl Dungeon {
    /// Конструиране на празен Dungeon, в който няма никакви стаи.
    ///
    pub fn new() -> Self {
        Dungeon {
            rooms: HashMap::new(),
        }
    }

    /// Добавяне на стая към Dungeon с име `name`. Връща `Ok(())` при успех. Ако вече има стая с
    /// такова име, очакваме да върнете `Errors::DuplicateRoom` с името.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        if self.rooms.contains_key(name) {
            Err(Errors::DuplicateRoom(name.to_string()))
        } else {
            self.rooms.insert(
                name.to_string(),
                Rc::new(Room {
                    name: name.to_string(),
                    connections: HashMap::new(),
                }),
            );
            Ok(())
        }
    }

    /// Прочитане на дадена стая -- когато извикаме `get_room`, очакваме reference към `Room`
    /// структурата с това име.
    ///
    /// Ако няма такава стая, очакваме `Errors::UnknownRoom` с подаденото име.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        if let Some(room) = self.rooms.get(room_name) {
            Ok(room)
        } else {
            Err(Errors::UnknownRoom(room_name.to_string()))
        }
    }

    /// Добавяне на съсед на дадена стая. След извикването на функцията, очакваме стаята с име
    /// `room_name` да има връзка в посока `direction` със стаята с име `other_room_name`.
    ///
    /// Също така очакваме `other_room_name` да има връзка с `room_name` в *обратната* посока.
    ///
    /// Успешен резултат е `Ok(())`. В случай, че която от двете стаи не същестува, очакваме грешка
    /// `Errors::UnknownRoom` със съответното име на липсваща стая. Ако и двете липсват, спокойно
    /// върнете тази, която проверявате първо.
    ///
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        if !self.rooms.contains_key(room_name) {
            return Err(Errors::UnknownRoom(room_name.to_string()));
        }
        if !self.rooms.contains_key(other_room_name) {
            return Err(Errors::UnknownRoom(other_room_name.to_string()));
        }
        let room = Rc::get_mut(self.rooms.get_mut(room_name).unwrap()).unwrap();
        room.connections
            .insert(direction, other_room_name.to_string());

        let other_room = Rc::get_mut(self.rooms.get_mut(other_room_name).unwrap()).unwrap();
        other_room
            .connections
            .insert(direction.opposite(), room_name.to_string());

        Ok(())
    }

    /// Четене на съседа на стаята с име `room_name` в посока `direction`. Тук има няколко
    /// варианта на изход:
    ///
    /// - Ако подадената стая не съществува, очакваме грешка `Errors::UnknownRoom`
    /// - Ако подадената стая няма съсед в тази посока, Ok(None) е смисления резултат
    /// - Иначе, чакаме reference към `Room` структурата на въпросния съсед, опакована в `Ok(Some(`.
    ///
    pub fn get_next_room(
        &self,
        room_name: &str,
        direction: Direction,
    ) -> Result<Option<&Room>, Errors> {
        if !self.rooms.contains_key(room_name) {
            return Err(Errors::UnknownRoom(room_name.to_string()));
        }
        let room = self.rooms.get(room_name).unwrap();
        if let Some(next_room_name) = room.connections.get(&direction) {
            Ok(Some(self.rooms.get(next_room_name).unwrap()))
        } else {
            Ok(None)
        }
    }

    /// Прочитаме структурата на dungeon от нещо, което имплементира `BufRead`. Това може да е
    /// файл, или, ако тестваме, може да е просто колекция от байтове.
    ///
    /// Успешен резултат връща новосъздадения dungeon, пакетиран в `Ok`.
    ///
    /// Вижте по-долу за обяснение на грешките, които очакваме.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        let mut dungeon = Dungeon::new();
        let mut line_count = 0;
        let mut parsing = Parsing::Room;
        let mut next_line_links = false;
        for line in reader.lines() {
            let line = line.unwrap();
            line_count += 1;
            match parsing {
                Parsing::Room => {
                    if line_count == 1 && line != "## Rooms" {
                        return Err(Errors::LineParseError { line_number: 1 });
                    } else if line_count == 1 && line == "## Rooms" {
                        continue;
                    } else if line.is_empty() && line_count != 1 {
                        parsing = Parsing::Links;
                        next_line_links = true;
                    } else if let Some(room_name) = match_prefix("- ", &line) {
                        dungeon.add_room(room_name)?;
                    } else {
                        return Err(Errors::LineParseError {
                            line_number: line_count,
                        });
                    }
                }
                Parsing::Links => {
                    if next_line_links && line != "## Links" {
                        return Err(Errors::LineParseError {
                            line_number: line_count,
                        });
                    } else if next_line_links && line == "## Links" {
                        next_line_links = false;
                    } else if let Some(link_info) = match_prefix("- ", &line) {
                        let parsed_link = match_link(link_info);
                        if let Some(Link {
                            from,
                            direction,
                            to,
                        }) = parsed_link
                        {
                            dungeon.set_link(&from, direction, &to)?;
                        } else {
                            return Err(Errors::LineParseError {
                                line_number: line_count,
                            });
                        }
                    } else {
                        return Err(Errors::LineParseError {
                            line_number: line_count,
                        });
                    }
                }
            }
        }

        if line_count == 0 {
            Err(Errors::LineParseError { line_number: 0 })
        } else {
            Ok(dungeon)
        }
    }

    pub fn read_from_file(filename: &str) -> Result<Self, Errors> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        Dungeon::from_reader(reader)
    }
}

/// match_prefix("- ", "- Foo") //=> Some("Foo")
/// match_prefix("- ", "Bar")   //=> None
///
fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    if input.starts_with(prefix) {
        Some(&input[prefix.len()..])
    } else {
        None
    }
}

pub struct Link {
    from: String,
    to: String,
    direction: Direction,
}

fn match_link<'b>(input: &'b str) -> Option<Link> {
    let mut link = Link {
        from: String::new(),
        to: String::new(),
        direction: Direction::North,
    };

    let mut iter = input.split_terminator(" -> ");
    link.from = iter.next().unwrap().to_string();
    link.direction = match iter.next().unwrap() {
        "North" => Direction::North,
        "South" => Direction::South,
        "East" => Direction::East,
        "West" => Direction::West,
        _ => return None,
    };
    link.to = iter.next().unwrap().to_string();
    Some(link)
}

enum Parsing {
    Room,
    Links,
}

impl Dungeon {
    /// Търси път от `start_room_name` до `end_room_name` и го връща във вектор, пакетиран във
    /// `Ok(Some(` ако намери.
    ///
    /// Ако няма път между тези две стаи, връща `Ok(None)`.
    ///
    /// Ако четенето на стаи в един момент върне грешка, очакваме да върнете грешката нагоре.
    ///
    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parents = HashMap::new();
        queue.push_front(start_room_name);
        visited.insert(start_room_name);
        parents.insert(start_room_name, None);
        while let Some(room_name) = queue.pop_back() {
            if room_name == end_room_name {
                let mut path = Vec::new();
                let mut current_room = room_name;
                while let Some(parent) = parents.get(current_room) {
                    if !parent.is_none() {
                        path.push(self.rooms.get(current_room).unwrap().as_ref());
                        current_room = parent.unwrap();
                    } else {
                        break;
                    }
                }
                path.push(self.rooms.get(start_room_name).unwrap().as_ref());
                path.reverse();
                return Ok(Some(path));
            }
            for direction in Direction::iter() {
                if let Ok(Some(neigh)) = self.get_next_room(room_name, direction) {
                    let neigh_name = neigh.name.as_str();
                    if !visited.contains(&neigh_name) {
                        queue.push_front(neigh_name);
                        visited.insert(neigh_name);
                        parents.insert(neigh_name, Some(room_name));
                    }
                }
            }
        }
        Ok(None)
    }
}
