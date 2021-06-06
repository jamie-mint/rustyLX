pub struct DoubleBuffer {
    mivar: Vec<u32>,
    dirty: bool,
}

impl DoubleBuffer {
    pub fn fill_red(&mut self, value: i32) {

    }
}

pub mod hosting {

    struct mibuf {
        mivar: Vec<i32>
    }

    static MIVAR: i32 = 32;
    // static MIVEC: vec![] = {12; 32};

    pub fn add_to_waitlist() {
        println!("waitlist added");
        super::serving::take_order()
    }

    fn seat_at_table() {}
}

mod serving {
    pub fn take_order() {
        println!("taken")
    }

    fn serve_order() {}

    fn take_payment() {}
}

