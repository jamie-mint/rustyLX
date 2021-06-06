pub mod engine;
mod color;

pub mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {
            println!("taking order");
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
