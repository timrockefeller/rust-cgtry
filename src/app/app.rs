use super::super::window::window::*;
pub struct App{
    windowstate: WindowState
}
impl App{
    pub fn new (){
        let mut ws = WindowState::new("new_window",(1024f64, 768f64));
        ws.evloop.run_forever(event_handler);
    }
}
