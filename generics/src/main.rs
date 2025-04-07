use num_traits::{ToPrimitive, Float};

fn solve(a: f64, b: f64) -> f64 {
    // pythagoras theorem
    (a.powi(2) + b.powi(2)).sqrt() // powi(2) = power of 2
}

// another way is to use Generics
// T is an argument and replaces everywhere with T
// T is passed from the caller ie. <f32> 
// Float is a trait, when used here it's a trait bound
// A trait is a set of methods, 
/*
    trait Vehicle {
        // abstract method
        fn start(&self);

        // default method
        fn stop(&self) {
            printkn!("Stopped");
        }
    }
*/
// A struct/enum/primitive can implement a trait
// the implementor has to provide implementations for all of the abstract methods
// the implementor can optionally override default methods
/* 
    struct Car {};

    impl Vehicle for Car {
        // abstract method
        fn start(&self) {
            println!("Start!!!!");
        } 
    }
*/
// Car gets all methods implemented in Vehicle
/*
    // Type T must be something that implements Vehicle trait
    fn Start_and_stop<T: Vehicle>(vehicle: T) {
        vehicle.start();
        vehicle.stop();
    }

    fn main() {
        let car = Car {};

        // because car has implemented Vehicle, 
        // it can be passed as a generic type
        start_and_stop(car);
    }
*/
fn solve2<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap(); 
    let b_f64 = b.to_f64().unwrap(); 
    // pythagoras theorem
    (a_f64.powi(2) + b_f64.powi(2)).sqrt() // powi(2) = power of 2
}

// use different types of floats
// now we pass in a: f32, b: f64
fn solve3<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap(); 
    let b_f64 = b.to_f64().unwrap(); 
    // pythagoras theorem
    (a_f64.powi(2) + b_f64.powi(2)).sqrt() // powi(2) = power of 2
}

// take in any numbers
fn super_solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap(); 
    let b_f64 = b.to_f64().unwrap(); 
    // pythagoras theorem
    (a_f64.powi(2) + b_f64.powi(2)).sqrt() // powi(2) = power of 2
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    // println!("{}", solve(a, b)); // error

    // cannot do arthmetic on different type of numbers
    // a + b; // error

    // one way is to convert a to f64
    let a: f32 = 3.0;
    let b: f64 = 4.0;
    let a_f64 = a as f64;
    println!("{}", solve(a_f64, b)); // works

    // use num_traits
    let a: f32 = 3.0;
    let b: f64 = 4.0;
    let a_f64 = a.to_f64().unwrap(); // to_f64 returns a Option
    println!("{}", solve(a_f64, b)); // works

    // another way is to use Generics
    // if you remove ::<f32> also works, Rust will infer the types
    let a: f32 = 3.0;
    let b: f32 = 4.0;
    println!("{}", solve2::<f32>(a, b)); // works
    println!("{}", solve2(a, b)); // works

    let a: f32 = 3.0;
    let b: f64 = 4.0;
    println!("{}", solve3(a, b)); // works

    let a: i32 = 3;
    let b: u8 = 4;
    println!("{}", super_solve(a, b)); // works
}
