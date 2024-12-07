use std::{thread, time::Duration};
#[derive(Debug)]
enum Light{
    Red,
    Yellow,
    Green,
}

impl Light {
    fn traffic(&self, color: &str) -> Self {
        match self{
            Light::Red => {
                if color == "red" {
                    println!("Stop! Wait for 10 Seconds");
                    thread::sleep(Duration::from_secs(10));
                    Light::Yellow
                } else {
                    println!("You can't go anywhere");
                    Light::Red
                }
            },
            Light::Yellow =>{
                if color == "yellow"{
                    println!("Wait till 3 seconds");
                    thread::sleep(Duration::from_secs(3));
                    Light::Green
                } else {
                    println!("You have to wait for infinte time");
                    thread::sleep(Duration::from_secs(8));
                    Light::Yellow
                }
            },
            Light::Green => {
                println!("You are free to go anywhere");
                Light::Green
            }
        }
    }
}

fn main(){
    let mut state = Light::Red;
    let input = vec!["red","yellow","green"];

    for i in input{
        println!("Current state {:?}", state);
        state = state.traffic(i);
    }

    println!("Final State {:?}",state);
}