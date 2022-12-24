use std::{ops::{Index, IndexMut}, fs::File};
use rand::prelude::*;
use image::ColorType;
use image::png::PNGEncoder;

const WID:usize = 5;
const HEI:usize = 7;

const MIN_WEI:usize = 11;
const MAX_WEI:usize = 22;

pub struct Rune {
    vec: Vec<bool>,
}

impl Index<usize> for Rune {
    type Output = [bool];
    fn index(&self, index: usize) -> &Self::Output {
        let i = index * WID;
        let e = i + WID;
        &self.vec[i..e]
    }
}

impl IndexMut<usize> for Rune {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index * WID;
        let e = i + WID;
        &mut self.vec[i..e]
    }
}

impl Rune {
    pub fn new() -> Rune {
        return Rune {
            vec: vec![false; WID*HEI],
        };
    }
    pub fn randomize(&mut self) {
        let mut w = 0;
        while w < MIN_WEI || w > MAX_WEI {
            self.vec = vec![false; WID*HEI];
            if Self::coin(5.0/7.0) {
                self.random1();
            } else {
                self.random2();
            }
            w = self.weight();
        }
    }
    fn random1(&mut self) {
        self[0][0] = Self::coin( 4.0/5.0 );
		self[0][2] = Self::coin( 4.0/5.0 );
		self[0][4] = Self::coin( 4.0/5.0 ); // 5/5

		self[3][0] = Self::coin( 4.0/5.0 );
		self[3][2] = Self::coin( 4.0/5.0 ); // 5/5
		self[3][4] = Self::coin( 4.0/5.0 ); // 5/5

		self[6][0] = Self::coin( 4.0/5.0 );
		self[6][2] = Self::coin( 4.0/5.0 );
		self[6][4] = Self::coin( 2.0/5.0 );

		self[0][1] = self[0][0] && self[0][2] && Self::coin( 3.0/4.0 ); // 4/4
		self[0][3] = self[0][2] && self[0][4] && Self::coin( 3.0/4.0 ); // 4/4

		self[2][0] = self[0][0] && self[3][0] && Self::coin( 2.0/3.0 );
		self[2][2] = self[0][2] && self[3][2] && Self::coin( 1.0/4.0 );
		self[2][4] = self[0][4] && self[3][4] && Self::coin( 4.0/5.0 ); // 5/5
        
        self[1][0] = self[2][0];
        self[1][2] = self[2][2];
        self[1][4] = self[2][4];

		self[3][1] = self[3][0] && self[3][2] && Self::coin( 3.0/4.0 );
		self[3][3] = self[3][2] && self[3][4] && Self::coin( 2.0/5.0 );

		self[5][0] = self[3][0] && self[6][0] && Self::coin( 3.0/4.0 ); // 2/2
		self[5][2] = self[3][2] && self[6][2] && Self::coin( 2.0/4.0 );
		self[5][4] = self[3][4] && self[6][4] && Self::coin( 3.0/4.0 ); // 2/2
        
        self[4][0] = self[5][0];
        self[4][2] = self[5][2];
        self[4][4] = self[5][4];

		self[6][1] = self[6][0] && self[6][2] && Self::coin( 4.0/5.0 ); // 2/2
		self[6][3] = self[6][2] && self[6][4] && Self::coin( 4.0/5.0 ); // 2/2

		if self[4][2] && (self[3][1] != self[3][3]) {
			self[5][2] = false;
        }
    }
    fn random2(&mut self) {
        self[0][0] = Self::coin( 1.0/2.0 );
		self[0][2] = Self::coin( 1.0/2.0 );
		self[0][4] = Self::coin( 1.0/2.0 );

		self[2][0] = Self::coin( 3.0/4.0 ); // 2/2
		self[2][2] = Self::coin( 3.0/4.0 ); // 2/2
		self[2][4] = Self::coin( 3.0/4.0 ); // 2/2

		self[4][0] = Self::coin( 1.0/2.0 );
		self[4][2] = Self::coin( 1.0/2.0 );
		self[4][4] = Self::coin( 1.0/2.0 );

		self[6][0] = Self::coin( 1.0/2.0 );
		self[6][2] = Self::coin( 1.0/2.0 );
		self[6][4] = Self::coin( 1.0/2.0 );

		self[0][1] = self[0][0] && self[0][2] && Self::coin( 1.0/4.0 ); // 0/2
		self[0][3] = self[0][2] && self[0][4] && Self::coin( 1.0/4.0 ); // 0/2

		self[1][0] = self[0][0] && self[2][0] && Self::coin( 3.0/4.0 ); // 1/1
		self[1][2] = self[0][2] && self[2][2] && Self::coin( 3.0/4.0 ); // 1/1
		self[1][4] = self[0][4] && self[2][4] && Self::coin( 3.0/4.0 ); // 1/1

		self[2][1] = self[2][0] && self[2][2] && Self::coin( 3.0/4.0 ); // 2/2
		self[2][3] = self[2][2] && self[2][4] && Self::coin( 3.0/4.0 ); // 2/2

		self[3][0] = self[2][0] && self[4][0] && Self::coin( 3.0/4.0 ); // 1/1
		self[3][2] = self[2][2] && self[4][2] && Self::coin( 1.0/2.0 );
		self[3][4] = self[2][4] && self[4][4] && Self::coin( 3.0/4.0 ); // 1/1

		self[4][1] = self[4][2];
		self[4][3] = self[4][2];

		self[5][0] = self[4][0] && self[6][0] && Self::coin( 3.0/4.0 ); // 1/1
		self[5][2] = self[4][2] && self[6][2] && Self::coin( 1.0/2.0 ); // 0/1
		self[5][4] = self[4][4] && self[6][4] && Self::coin( 3.0/4.0 ); // 1/1

		self[6][1] = self[6][0] && self[6][2] && Self::coin( 1.0/4.0 ); // 0/0
		self[6][3] = self[6][2] && self[6][4] && Self::coin( 1.0/4.0 ); // 0/0
    }
    fn weight(&self) -> usize {
        let mut w = 0;
        for y in 0..HEI {
            for x in 0..WID {
                if self[y][x] {
                    w += 1;
                }
            }
        }
        w
    }
    fn coin(chance:f64) -> bool {
        rand::thread_rng().gen::<f64>() < chance
    }
    fn render(&self, pixels: &mut [u8], x: usize, y: usize, w: usize) {
        for j in 0..HEI {
            for i in 0..WID {
                let idx = (x + i) + (y + j) * w;
                pixels[idx] = match self[j][i] {
                    true => 0,
                    false => 255,
                }
            }
        }
    }
}


fn main() {
    let space = 3;
    let num_x = 10;
    let num_y = 10;
    let img_wid = num_x * (space + WID) + space;
    let img_hei = num_y * (space + HEI) + space;
    let mut pixels = vec![255; img_wid * img_hei];
    
    for y in 0..num_y {
        for x in 0..num_x {
            let mut rune = Rune::new();
            rune.randomize();
            rune.render(&mut pixels, x * (space + WID) + space, y * (space + HEI) + space, img_wid);
        }
    }
    let file = File::create("out.png").unwrap();
    let encoder = PNGEncoder::new(file);
    encoder.encode(&pixels, img_wid as u32, img_hei as u32, ColorType::Gray(8)).unwrap();
}
