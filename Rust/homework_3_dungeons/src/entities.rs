use std::collections::HashMap;

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
#[derive(Debug,Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Една стая в подземията. Дефинира се само с име, макар че в по-интересна имплементация може да
/// държи item-и, противници...
///
pub struct Room {
    pub name: String,
    // Каквито други полета ви трябват
}

/// Контейнер за стаите и не само. Ще работим предимно със тази структура.
///
pub struct Dungeon {
    pub rooms: HashMap<String, Room>,
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
            self.rooms.insert(name.to_string(), Room { name: name.to_string() });
            Ok(())
        }
    }

    /// Прочитане на дадена стая -- когато извикаме `get_room`, очакваме reference към `Room`
    /// структурата с това име.
    ///
    /// Ако няма такава стая, очакваме `Errors::UnknownRoom` с подаденото име.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        todo!()
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
        todo!()
    }

    /// Четене на съседа на стаята с име `room_name` в посока `direction`. Тук има няколко
    /// варианта на изход:
    ///
    /// - Ако подадената стая не съществува, очакваме грешка `Errors::UnknownRoom`
    /// - Ако подадената стая няма съсед в тази посока, Ok(None) е смисления резултат
    /// - Иначе, чакаме reference към `Room` структурата на въпросния съсед, опакована в `Ok(Some(`.
    ///
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        todo!()
    }
}