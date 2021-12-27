use std::{collections::HashMap, rc::Rc, ops::{DerefMut, RangeBounds}};


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
        & mut self,
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
        room.connections.insert(direction, other_room_name.to_string());

        let other_room = Rc::get_mut(self.rooms.get_mut(other_room_name).unwrap()).unwrap();
        other_room.connections.insert(direction.opposite(), room_name.to_string());

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
}