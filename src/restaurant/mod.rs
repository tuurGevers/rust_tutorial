mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza{
                dough: String::from("regular dough"),
                cheese:  String::from("mozzarella"),
                topping:  String::from(topping)
            }
        }
    }

    pub mod help_customer{
        use std::io;
        use crate::restaurant::pizza_order::Pizza;

        fn seat_at_table(){
            println!("Customer seated at table");
        }
        fn serve_customer(cust_pizza: super::Pizza){
            println!("The customer is served a pizza with {}.", cust_pizza.topping)

        }
        pub fn take_order(){
            seat_at_table();
            let mut topping = String::new();
            println!("What kind of toppings would you like");
            io::stdin().read_line(&mut topping).expect("didn't give any input");
            let cust_pizza : super::Pizza =
                super::Pizza::lunch(topping.trim_end());
            serve_customer(cust_pizza);
        }

    }
}

pub fn order_food(){
    crate::restaurant::pizza_order::help_customer::take_order()
}