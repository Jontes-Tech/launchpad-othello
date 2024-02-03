use crate::controllers::Controller;

pub mod othello;

pub trait Script: Send {
    fn name(&self) -> &'static str;

    fn new() -> Self where Self: Sized;

    fn initialize(&mut self, _controller: &dyn Controller) {}

    fn on_control_press(&mut self, _index: u8, _controller: &dyn Controller) {}

    fn on_press(&mut self, _x: u8, _y: u8, _controller: &dyn Controller) {}

    fn on_release(&mut self, _x: u8, _y: u8, _controller: &dyn Controller) {}
}
