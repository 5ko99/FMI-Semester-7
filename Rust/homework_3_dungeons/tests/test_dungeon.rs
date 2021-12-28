use std::{fs::File, io::{BufRead, BufReader}};

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

#[test]
fn read_dungeon1_file() {
    let dungeon = Dungeon::read_from_file("tests/test_dungeon1.txt").unwrap();

    assert_eq!(dungeon.rooms.len(), 3);
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Magic Lab");
    assert_eq!(dungeon.get_next_room("Magic Lab", Direction::East).unwrap().unwrap().name, "Hallway");
}

#[test]
fn read_dungeon2_file() {
    let dungeon = Dungeon::read_from_file("tests/test_dungeon2.txt").unwrap();

    assert_eq!(dungeon.rooms.len(), 6);
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::North).unwrap().unwrap().name, "Exit");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::North).unwrap().unwrap().name, "Bedroom");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::South).unwrap().unwrap().name, "Magic Lab");
    assert_eq!(dungeon.get_next_room("Bedroom", Direction::West).unwrap().unwrap().name, "Exit");
    assert_eq!(dungeon.get_next_room("Bedroom", Direction::South).unwrap().unwrap().name, "Magic Lab");
    assert_eq!(dungeon.get_next_room("Magic Lab", Direction::North).unwrap().unwrap().name, "Bedroom");
    assert_eq!(dungeon.get_next_room("Exit", Direction::South).unwrap().unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Exit", Direction::East).unwrap().unwrap().name, "Bedroom");
    assert_eq!(dungeon.get_next_room("Exit", Direction::North).unwrap().unwrap().name, "Special Room");
    assert_eq!(dungeon.get_next_room("Special Room", Direction::South).unwrap().unwrap().name, "Exit");
}

#[test]
fn test_path_1() {
    let dungeon = Dungeon::read_from_file("tests/test_dungeon2.txt").unwrap();
    let path = dungeon.find_path("Entrance", "Magic Lab").unwrap().unwrap();

    assert_eq!(path.len(), 3);
    assert_eq!(path[0].name, "Entrance");
    assert_eq!(path[1].name, "Hallway");
    assert_eq!(path[2].name, "Magic Lab");
}

#[test]
fn test_path_cycle() {
    let mut dungeon = Dungeon::new();
    dungeon.add_room("A").unwrap();
    dungeon.set_link("A", Direction::West, "A").unwrap();
    let path = dungeon.find_path("A","A").unwrap().unwrap();
    assert_eq!(path.len(),1);
    assert_eq!(path[0].name,"A");
}

#[test]
fn test_path_2() {
    let dungeon = Dungeon::read_from_file("tests/test_dungeon1.txt").unwrap();
    let path = dungeon.find_path("Entrance", "Magic Lab").unwrap().unwrap();

    assert_eq!(path.len(), 3);
    assert_eq!(path[0].name, "Entrance");
    assert_eq!(path[1].name, "Hallway");
    assert_eq!(path[2].name, "Magic Lab");
}

#[test]
fn test_basic_1() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

const TEST_INPUT_1: &str = "
## Rooms
- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
";

#[test]
fn test_basic_2() {
    // .trim() за да премахнем първия и последния ред:
    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

#[test]
fn test_basic_3() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Treasure Room").unwrap();
    dungeon.set_link("Entrance", Direction::West, "Treasure Room").unwrap();

    let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();
    assert!(path.len() > 0);
}