use crate::game::Game;

pub struct Organisms;

impl Organisms {
    pub fn new() -> Self {
        Organisms
    }

    pub fn block(game: &mut Game, x: usize, y: usize) {
        game.set_alive(x, y);
        game.set_alive(x + 1, y);
        game.set_alive(x, y + 1);
        game.set_alive(x + 1, y + 1);
    }

    pub fn tub(game: &mut Game, x: usize, y: usize) {
        game.set_alive(x+1, y);
        game.set_alive(x, y+1);
        game.set_alive(x + 2, y + 1);
        game.set_alive(x + 1, y + 2);
    }

    pub fn bees(game: &mut Game, x: usize, y: usize) {
        game.set_alive(x+1, y);
        game.set_alive(x+2, y);
        game.set_alive( x    , y + 1);
        game.set_alive(x+3 , y + 1);
        game.set_alive(x+1, y + 2);
        game.set_alive(x+2 , y + 2);
    }

    pub fn bread(game: &mut Game, x: usize, y: usize) {
        game.set_alive(x+1, y);
        game.set_alive(x+2, y);
        game.set_alive( x    , y + 1);
        game.set_alive(x+3 , y + 1);
        game.set_alive(x+1, y + 2);
        game.set_alive(x+3 , y + 2);
        game.set_alive(x+2 , y + 3);

    }

    pub fn boat(game: &mut Game, x: usize, y: usize) {
        game.set_alive(x, y);
        game.set_alive(x+1, y);
        game.set_alive( x    , y + 1);
        game.set_alive(x+2 , y + 1);
        game.set_alive(x+1, y + 2);
    }

    pub fn pulse(game: &mut Game, x: usize, y: usize) {
        //Horizontales
        game.set_alive(x+2, y);
        game.set_alive(x+3, y);
        game.set_alive(x+4, y);
        game.set_alive(x+8, y);
        game.set_alive(x+9, y);
        game.set_alive(x+10, y);

        game.set_alive(x+2, y+5);
        game.set_alive(x+3, y+5);
        game.set_alive(x+4, y+5);
        game.set_alive(x+8, y+5);
        game.set_alive(x+9, y+5);
        game.set_alive(x+10, y+5);

        game.set_alive(x+2, y+7);
        game.set_alive(x+3, y+7);
        game.set_alive(x+4, y+7);
        game.set_alive(x+8, y+7);
        game.set_alive(x+9, y+7);
        game.set_alive(x+10, y+7);

        game.set_alive(x+2, y+12);
        game.set_alive(x+3, y+12);
        game.set_alive(x+4, y+12);
        game.set_alive(x+8, y+12);
        game.set_alive(x+9, y+12);
        game.set_alive(x+10, y+12);

        //Verticales
        game.set_alive(x, y+2);
        game.set_alive(x, y+3);
        game.set_alive(x, y+4);

        game.set_alive(x, y+8);
        game.set_alive(x, y+9);
        game.set_alive(x, y+10);

        game.set_alive(x+5, y+2);
        game.set_alive(x+5, y+3);
        game.set_alive(x+5, y+4);

        game.set_alive(x+5, y+8);
        game.set_alive(x+5, y+9);
        game.set_alive(x+5, y+10);

        game.set_alive(x+7, y+2);
        game.set_alive(x+7, y+3);
        game.set_alive(x+7, y+4);

        game.set_alive(x+7, y+8);
        game.set_alive(x+7, y+9);
        game.set_alive(x+7, y+10);
        
        game.set_alive(x+12, y+2);
        game.set_alive(x+12, y+3);
        game.set_alive(x+12, y+4);

        game.set_alive(x+12, y+8);
        game.set_alive(x+12, y+9);
        game.set_alive(x+12, y+10);



    }

    pub fn spaceship(game: &mut Game, x: usize, y: usize) {

        game.set_alive(x+3, y);
        game.set_alive(x+4, y);

        game.set_alive(x+1, y + 1);
        game.set_alive(x + 6, y + 1);
        
        game.set_alive(x, y + 2);
        
        game.set_alive(x, y + 3);
        game.set_alive(x+6, y + 3);
        
        game.set_alive(x, y + 4);
        game.set_alive(x + 1, y + 4);
        game.set_alive(x + 2, y + 4);
        game.set_alive(x + 3, y + 4);
        game.set_alive(x + 4, y + 4);
        game.set_alive(x + 5, y + 4);
    }

    pub fn raro(game: &mut Game, x: usize, y: usize) {

        game.set_alive(x, y);
        game.set_alive(x+1, y);
        game.set_alive(x, y+1);
        game.set_alive(x+1, y+1);

        game.set_alive(x, y+2);
        game.set_alive(x, y+3);
        game.set_alive(x, y+4);
        game.set_alive(x, y+5);

        game.set_alive(x+2, y+2);
        game.set_alive(x+2, y+3);

    }
    
    //Tomando expresiones largas para convertirlas a mi configuración de game-set_alive
    pub fn parse_rle(pattern: &str) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut num = 0;
    
        for c in pattern.chars() {
            match c {
                '0'..='9' => {
                    num = num * 10 + c.to_digit(10).unwrap() as usize;
                }
                'b' => {
                    x += if num == 0 { 1 } else { num };
                    num = 0;
                }
                'o' => {
                    let count = if num == 0 { 1 } else { num };
                    for _ in 0..count {
                        cells.push((x, y));
                        x += 1;
                    }
                    num = 0;
                }
                '$' => {
                    y += if num == 0 { 1 } else { num };
                    x = 0;
                    num = 0;
                }
                '!' => break,
                _ => {}
            }
        }
    
        cells
    }

    
    
    

}
