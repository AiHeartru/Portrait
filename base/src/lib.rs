mod portrait;

use portrait::*;

pub struct Base ();

impl Base {
    pub fn gen_portrait() -> String {
        Portrait::gen_portrait(150, 150, [35u8, 37u8, 40u8])
    }
}