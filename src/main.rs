use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    let mut player: u8;
    /* initilizing computer with an random number between 1 to 100
     * 1 is included and 101 is excluded
     */
    let computer: u8 = rng.gen_range(1..101);
    let mut play_count: u8 = 6;

    println!("you have only {} chance to win",play_count);
    println!("lets start...");

    // first input for initilization
    player = get_user_input("between 1 to 100".to_string());

    while play_count > 0 {
        if computer == player {
            // ans
            println!("you won");
            play_count -= 1;
            break;
        } else if player > computer {
            // if user input bigger then ans
            player = get_user_input("little small".to_string());
        } else {
            //// if user input smaller then ans
            player = get_user_input("little big".to_string());
        }
        play_count -= 1;
    }

    if play_count != 0 {
        println!("you have completed in only {} chances", play_count);
    } else {
        println!("you have lose")
    }
}

/* function to get user input(string) and also convert into a number and returns it
 * perameter string for function daynamic behaviour
 */
fn get_user_input(str: String) -> u8 {
    let mut temp = String::new();

    println!("enter a number {}", str);

    let _ = io::stdin().read_line(&mut temp);

    let row_input = temp.trim().parse::<u8>();

    // string to int conversion error handling
    match row_input {
        Ok(n) => n,
        Err(err) => {
            println!("{}", err);
            0
        }
    }
}
