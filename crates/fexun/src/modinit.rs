// Note to self: Man, there's so much stupid boilerplate code in this project. Fucking kill me.

use fexun_winit::start_wingpu;

pub fn start() {
    start_wingpu(); // This starts the following components concurrently: Window, Renderer, EventLoop. I know it's stupid but I'm too lazy to fix it.
}
