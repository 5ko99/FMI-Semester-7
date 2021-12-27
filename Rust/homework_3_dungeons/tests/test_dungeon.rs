use homework_3_dungeons::entities::{Direction, Dungeon};

#[test]
fn create_empty_dungeon() {
    let mut dungeon = Dungeon::new();
    assert_eq!(dungeon.rooms.len(), 0);
}

#[test]
fn add_room_to_empty_dungeon() {
    let mut dungeon = Dungeon::new();
    dungeon.add_room("Room 1").unwrap();
    assert_eq!(dungeon.rooms.len(), 1);
    dungeon.add_room("Room 2").unwrap();
    assert_eq!(dungeon.rooms.len(), 2);
}

#[test]
fn duplicate_room() {
    let mut dungeon = Dungeon::new();
    dungeon.add_room("Room 1").unwrap();
    assert_eq!(dungeon.rooms.len(), 1);
    dungeon.add_room("Room 1").unwrap_err();
    assert_eq!(dungeon.rooms.len(), 1);
}

#[test]
fn add_connection_and_get_room() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon
        .set_link("Entrance", Direction::East, "Hallway")
        .unwrap();

    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
    assert_eq!(
        dungeon
            .get_next_room("Hallway", Direction::West)
            .unwrap()
            .unwrap()
            .name,
        "Entrance"
    );
}

#[test]
fn override_connection() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Magic Lab").unwrap();

    dungeon
        .set_link("Entrance", Direction::East, "Hallway")
        .unwrap();
    dungeon
        .set_link("Hallway", Direction::West, "Magic Lab")
        .unwrap();

    assert_eq!(
        dungeon
            .get_next_room("Entrance", Direction::East)
            .unwrap()
            .unwrap()
            .name,
        "Hallway"
    );
    assert_eq!(
        dungeon
            .get_next_room("Hallway", Direction::West)
            .unwrap()
            .unwrap()
            .name,
        "Magic Lab"
    );
}

#[test]
fn get_room_not_found() {
    let mut dungeon = Dungeon::new();
    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon
        .set_link("Entrance", Direction::East, "Hallway")
        .unwrap();

    assert_eq!(dungeon.get_next_room("Entrance", Direction::West).unwrap(), None);
}

#[test]
fn concet_one_room_with_self() {
    let mut dungeon = Dungeon::new();
    dungeon.add_room("Entrance").unwrap();
    dungeon
        .set_link("Entrance", Direction::East, "Entrance").unwrap();
    assert_eq!(dungeon.rooms.len(), 1);
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::West).unwrap().unwrap().name, "Entrance");
}