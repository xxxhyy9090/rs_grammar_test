use std::io;

mod sort;
mod test;
use sort::{get_sort_fn,
           parse_sort_mod};
use crate::sort::print_menu;

fn main() {
    loop{
        let mut choice=String::new();
        let mut input=String::new();
        print_menu();

        io::stdin().read_line(&mut choice).expect("Failed to read line");
        if choice.trim()=="q"{
            break;
        }
        let choice=match  parse_sort_mod(choice.trim()){
            Some(x)=>x,
            None=>{
                println!("Invalid input");
                continue;
            }
        };
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input_sort:Vec<i32>=input.
            trim().
            split(|c:char| c==',' || c.is_whitespace()).
            filter(|x| !x.is_empty()).
            filter_map(|x| x.parse::<i32>().ok()).
            collect();

        if input_sort.is_empty(){
            println!("No results");
            continue;
        }
        let sort_fn=get_sort_fn(choice);
        sort_fn(&mut input_sort);
        println!("The sort results is:{:?}",input_sort);
    }
}
