use super::arg::{GetItemArgs, StartGameArgs};


pub enum CoreCommand {
    /// Start the game (!core-start)
    StartGame(StartGameArgs),
    /// Add an item to inventory (!core-add)
    ///
    /// If value is not given, this uses the logic for doGetItem (0x0073A464)
    Add(GetItemArgs),
}
