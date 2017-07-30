use indexes::{WatchDiff};
use ops::{Interner};

pub trait Watcher {
    fn get_name(& self) -> String;
    fn on_diff(&mut self, interner:&mut Interner, diff:WatchDiff);
}

pub mod file;
pub mod console;
pub mod system;
pub mod compiler;