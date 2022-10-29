// this lib file is the entry point to the crate: the Rust see it as crate module
// parent module of hosting and serving module and child of lib(crate mod by default)
mod front_of_house {

    // hosing is a child module of front_of_house
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // serving is a sibling module of hosting and child of front_of_house 
    // module
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
