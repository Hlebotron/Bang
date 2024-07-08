use std::io::stdin;
use rand::*;
// A characters enum may or may not be required
#[derive(Debug, Clone, Copy)]
struct Card<'d> {
    name: &'d str,
    hearts: u8, 
    diamonds: u8, 
    spades: u8, 
    clubs: u8,
}
impl Card<'_> {
    fn bang<Card>(&self, target: u8) {
        println!("Bang!!!");
    }
    fn pogger<Card>(&self) -> &u8 {
        &self.diamonds
    }
    fn pull(&mut self) -> CardInHand/*Vec<u8>*/ {
        let mut suits: Vec<u8> = vec![self.hearts.clone(), self.diamonds.clone(), self.spades.clone(), self.clubs.clone()];
        let mut suit_index: Vec<&str> = vec!["hearts", "diamonds", "spades", "clubs"];
        let mut filtered_suits: Vec<u8> = Vec::new();
        let mut index: usize = 0;
        for suit_count in suits.clone() { 
            if suit_count != 0 {
                filtered_suits.push(suit_count);
                index += 1;
            } else {
                suit_index.remove(index);
                suits.remove(index);
            }
        }
        let suit_selection = filtered_suits;
        let random_suit_number = rand::thread_rng().gen_range(0..=suit_index.len()-1);
        let random_suit = suit_index[random_suit_number];
        match random_suit {
            "hearts" => {*&mut self.hearts -= 1;()},
            "diamonds" => {*&mut self.diamonds -= 1;()},
            "spades" => {*&mut self.spades -= 1;()},
            "clubs" => {*&mut self.clubs -= 1;()},
            _ => panic!("What"),
        };
        //random_suit
        CardInHand {
            name: &self.name,
            suit: random_suit,
        }
    }
}
#[derive(Debug)]
struct CardInHand<'e> {
    name: &'e str,
    suit: &'e str,
}
#[derive(Debug)]
struct Player<'c> {
    name: String,
    health: i8,
    character: String,
    role: String,
    weapon: String,
    cards: Vec<&'c str>,
}
impl Player<'_> {
    fn select_card<'f>(&'f self, card_list: &'f Vec<&mut Card<'f>>) -> Card{
        let filtered_list: /*Vec<_>*/Vec<&&mut Card> = card_list.iter().filter(|x| (x.clubs + x.diamonds + x.hearts + x.spades) != 0).collect();
        let mut card = **filtered_list[rand::thread_rng().gen_range(0..=filtered_list.len()-1)];
        println!("{:?}", &card);
        card
    }
}

fn read() -> String {
    let mut string = String::new();
    stdin().read_line(&mut string).expect("Failed to read line");
    string
}
fn select<'a>(options: &Vec<&'a str>) -> &'a str {
    let mut index: u8 = 1;
    for option in options.iter() {
        println!("\t{}: {}", &mut index, option);
        index += 1;
    }
    println!("Input:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    let parse_result = buffer.trim().parse::<i8>();
    let options_len = options.len() as i8;
    let choice = match parse_result {
        Ok(result) if result >= 1 && result <= options_len => result,   
        Err(_) | Ok(_) => -1,
    };
    if choice == -1 {
        println!("Invalid input, please specify a valid number");
        let output = select(options);
        output
    } else {
        options[(choice - 1) as usize] 
    }
}
fn int_input() -> i8 {
    println!("Input:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    let parse_result = buffer.trim().parse::<i8>();
    let choice = match parse_result {
        Ok(result) => result,   
        Err(_) => -1,
    };
    if choice == -1 {
        println!("Invalid input, please specify a valid number");
        let output = int_input();
        output
    } else {
        choice
    }
}
fn set_player_count() -> u8 /*Outputs player_count*/ {
    let player_count: u8 = int_input().try_into().unwrap();
    let result = match player_count {
        count if count <= 8 && count >= 4 => count, 
        _ => {println!("Please enter a number between 4 and 8 (inclusive).");return set_player_count();},
    };
    println!("{}", &result);
    result
}

fn main() {
    let mut bang = Card {
            name: "Bang",
            hearts: 1,
            diamonds: 0,
            spades: 0,
            clubs: 0,
    };
    let mut miss = Card { 
            name: "Miss",
            hearts: 3,
            diamonds: 2,
            spades: 0,
            clubs: 1,
    };
    let mut jail = Card { 
            name: "Jail",
            hearts: 3,
            diamonds: 2,
            spades: 1,
            clubs: 2,
    };
    let mut character_list = vec!["Calamity Janet", "El Gringo", "Slab the Killer", "Willy the Kid", "Vulture Sam", "Paul Regret", "Jourdonnais", "Jesse Jones", "Kit Carlson", "Lucky Duke", "Pedro Ramirez", "Bart Cassidy", "Rose Doolan", "Sid Ketchum", "Suzy Lafayette", "Black Jack"];
    let mut role_list = vec!["Sheriff", "Outlaw", "Outlaw", "Renegade"];
    println!("How many players will play this round?");
    let player_count = set_player_count();
    match &player_count {
        4 => (), 
        5 => {role_list.push("Deputy");}, 
        6 => {role_list.push("Deputy");role_list.push("Outlaw");}, 
        7 => {role_list.push("Deputy");role_list.push("Outlaw");role_list.push("Deputy");}, 
        8 => {role_list.push("Deputy");role_list.push("Outlaw");role_list.push("Deputy");role_list.push("Outlaw");}, 
        _ => panic!("How in the heck is something simultaneously between 4 and 8 and NOT between 4 and 8?"),
    }
    let mut player_list: Vec<Player> = Vec::new();
    let mut deck: Vec<&mut Card> = vec![&mut bang, &mut miss, &mut jail]; 
    for i in 1..=player_count {
        let mut health: i8 = 4;
        let role_random = rand::thread_rng().gen_range(0..=role_list.len()-1);
        let role = role_list[*&role_random];
        role_list.remove(role_random);
        println!("Player {i}, your role is {role}");
        if role == "Sheriff" {
            health += 1;
        }
        println!("Player {i}, what's your name?");
        let name = read();
        println!("Your name is: {name}");
        println!("Player {i}, what character do you choose?");
        let mut character_options = Vec::new(); 
        for i in 1..=2 {    
            let char_random = rand::thread_rng().gen_range(0..=character_list.len()-1);
            character_options.push(character_list[char_random]);
            character_list.remove(char_random);
        }
        let character = select(&character_options);
        if character == "El Gringo" || character == "Paul Regret" {
            health -= 1;
        }
        let mut cards = Vec::new();
        for i in 1..=health {
            cards.push("pog");
        }
        player_list.push(Player{name: name.trim().to_string(), health: health, character: character.to_string(), role: role.to_string(), weapon: String::from("Colt 45"), cards: cards});
    };
    /*Card::bang::<&Card>(&bang, 3);
    Card::pogger::<&Card>(&miss);*/
    let mut card: &'_ mut Card<'_> = &mut Player::select_card(&player_list[0], &deck);
    println!("The card is: {:?}", &card);
    //println!("{:?}", card);
    let pulled_card = Card::pull(&mut card);
    println!("{:?}", pulled_card);
    println!("The card after the pull is: {:?}", &card);
}
