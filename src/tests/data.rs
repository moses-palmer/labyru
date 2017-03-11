use room::Room;


pub struct TestRoom {}


impl Rooms for TestRoom {}


impl Clone for TestRoom {
    fn clone(&self) -> TestRoom {
        TestRoom {}
    }
}


impl Default for TestRoom {
    fn default() -> TestRoom {
        TestRoom {}
    }
}
