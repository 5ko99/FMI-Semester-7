use homework_3_dungeons::entities::Dungeon;

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
fn duplicate_room (){
    let mut dungeon = Dungeon::new();
    dungeon.add_room("Room 1").unwrap();
    assert_eq!(dungeon.rooms.len(), 1);
    dungeon.add_room("Room 1").unwrap_err();
    assert_eq!(dungeon.rooms.len(), 1);
}