fn cook_order(){}
mod front_of_the_house{
    pub mod hosting{
        pub fn add_to_the_wait_list(){}
        fn seat_at_the_table(){}
    }
    mod serving{
        fn take_order(){}
        pub fn serve_order(){}
        fn take_payment(){}
    }
    
    fn fix_incorrect_order(){
        super::cook_order();
        serving::serve_order();
    }

}

pub fn eat_at_restaurant(){

    //absolute path
    crate::front_of_the_house::hosting::add_to_the_wait_list();

    //relatiive path
    front_of_the_house::hosting::add_to_the_wait_list();

}
