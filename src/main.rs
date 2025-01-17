use framebuffer::Framebuffer;
use organisms::Organisms;
use minifb::{Window, WindowOptions};
use std::time::Duration;
use game::Game;

mod framebuffer;
mod organisms;
mod game;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 200;
    let framebuffer_height = 100;

    let tick_duration = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let mut game = Game::new(framebuffer_width, framebuffer_height);

    // Inicializar organismos
    /*
    Organisms::block(&mut game, 1, 1);
    Organisms::tub(&mut game, 4, 4);
    Organisms::bees(&mut game, 20, 20);
    Organisms::bread(&mut game, 30, 20);
    Organisms::boat(&mut game, 35, 20);
    Organisms::pulse(&mut game, 60, 20);
    Organisms::spaceship(&mut game, 70, 40);
    */

    //Patron de varios pulse en la pantalla
    for i in (10..framebuffer_height-10).step_by(12) {
        for j in (10..framebuffer_width-10).step_by(12) {
            if (i / 2 + j / 2) % 2 == 0 {
                Organisms::pulse(&mut game, j, i);
            }
        }
    }

    /*for j in (40..framebuffer_width-40).step_by(20) {
        Organisms::raro(&mut game, j, 35);
    }*/

    pub fn add_pattern(game: &mut Game, pattern: &str, x_offset: usize, y_offset: usize) {
        let cells = Organisms::parse_rle(pattern);
    
        for &(x, y) in &cells {
            game.set_alive(x + x_offset, y + y_offset);
        }
    }
    
    //Prueba de una nave rara
    /*
    pub fn add_forward_glider_puffer(game: &mut Game, x: usize, y: usize) {
        let pattern = "174b2o$174b3o$174b2obo$178bo13bo$170bo3b2o3bo12bo$180bo11bo$177bobo$107b3o67bo$107bo64bo4bobo13bo$108bo82b2ob2o4b2o$176b4o20b2o$176b2o12bobo$190bo3bo$190b2o2bo$191b3o$95bo6b2o59b3o26bo$94bobo3bo3bo67bo15bo$93bo3bobo3b2o60b2o5b3o12bobo18b2o$94bobo2bobob2o60b4o4bo13bo2bo17b2o$95bo3bo2b2o61b2ob3ob2o14b2o$99bo4bo65bobo$100bo2bo10b2o68b2o$102bo11b2o68b2o$123b3o$123bo22bo51bo$124bo20b3o29bo19b3o$144b3obo14bo12bobo17b3obo$143b2o3b2o6bo2bo3bo12bobo16b2o3b2o$24b3o115b2obo2b2o6bo3bo2bo13bo16b2obo2b2o$24b3o95b2o19b2obob2o5bo39b2obob2o$25bo96b2o20b3ob2o6bob2o5bo30b3ob2o$26b2o59bo58bo4bo13b2o31bo4bo$27bo59bo59b4o13bo34b4o$26bo14b2o44bo8b3o50bo12bo38bo$41b2o48bo9bo60bo$87b3o2bo7bobo$86bo4bo9bo$24b2o60bo3bo39b2o$24b2o61b3o40b2o$25bo93bo6b2o11b3o52b3o$25bo15bo76bobo3bo3bo10bo54b3o$24bobo15b2o5b2o35bobo28bo3bobo3b2o11bo69b2o$23bo16bobo6b2o35bobo29bobo2bobob2o65bob2o12b2o$24bo2b2o11bo45bobo30bo3bo2b2o66bo3bo$25bo3bo11b2o31bo11bobo34bo4bo22b2o8b2o33b3o$27b2o14bo25bo2bobo2bo3bo5b2o35bo2bo23b2o8b2o$27bo13b3o25bo4b2ob5o14b2o28bo33bobo$13bo28bo26bo3bo4bobo14bo2bo$12bobo25b2o54b2o$11bo3bo13b2o26b2o12b3o121bo22b2o$11bo3bo13b2o8b2o16b2o13bo121bobo21b2o$11bobob3o21b2o154bo$12b2obo2bo140b2o33b2o$16bobo140b2o33b2o$16b2o24b2o47b2o102bo$2bo39b2o47b2o14bo47b3o35bob2o$bobo82bobo17bobo46bo38bob2obo$bobo61b2o19bobo16bo3bo46bo37bo4bo$bo2bo18b2o40b2o21bo17bobo88bobo$2bobo17bo2bo60b2o19bo$5bo17b2o61bo80b2o17b2o3b4o$4b2o79b3o19b4o56b2o12b3obo2bo6bo$4bo71bob2o26bo4bo73bo2bo2b5o$72b3ob2obo28bo76bo2bo$73bo3bobo25bo3bo2bo74bo$74bo3bo26bob3obo72bobo$2bo15b2o37bobo46b3obo74bo$bobo14b2o36bo2bo35bo6b2o$3bo53bobo34bobo3bo3bo70b2o$3bo10b2o77bo3bobo3b2o70b2o$3bo9bo2bo62b2o13bobo2bobob2o$14bobo42bo19b2o14bo3bo2b2o67b3o$4bo10bo42bob2o37bo4bo66bo$2ob3o52bob2o38bo2bo68bo$2b3ob2o49bo3bo40bo102bo$5bo35b2obo13b4o142bo$40b3obo6bobo4b3o143b3o$39bo4bobo4bo$40b2o6bobo3bo32b2o$41bo3bo2bo2b2o34b2o$24b3o17bob2o3bo47bo2bo$24b3o16bobo57bo$6b2o17bo17bobo53bo3bo$6b2o18b2o72b4o$27bo$26bo2$187b3o$155bo2bo28bo$24b2o43b2o88bo28bo$14b2o8b2o42bobo84bo3bo$14b2o9bo44bo85b4o$38b2o$38b2o7$46b2o$46b2o!";
        
        add_pattern(game, pattern, x, y);
    }
    */
    //add_forward_glider_puffer(&mut game, 1, 1);

    


    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        game.update();
        game.draw(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(tick_duration);
    }
}
