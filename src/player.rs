/*
* Authors: Neel Kowdley <nkowdley@gmail.com>, David Sweeney <dms163@pitt.edu>
* Class: CS1632
* Project: Deliverable 6: CoffeeMakerQuest in Rust
* File: player.rs
*/
/*Initialize the Player Struct*/
pub struct Player {
	has_cream: bool,
	has_coffee: bool,
	has_sugar:bool
}
/*implement the player*/
#[allow(dead_code)]
impl Player {
	/*Constructor*/
	pub fn new() -> Player {
		Player {
			has_cream : false,
			has_coffee : false,
			has_sugar : false
		}
	}
	/*Set has_cream to true*/
	pub fn get_cream(&mut self) -> () {
		self.has_cream = true;
	}
	/*Set has_coffee to true */
	pub fn get_coffee(&mut self) -> () {
		self.has_coffee = true;
	}
	/*Set has_sugar to true */
	pub fn get_sugar(&mut self) -> () {
		self.has_sugar = true;
	}
	/*Find out if the player can win */
	pub fn can_win(&self) -> bool {
		self.has_sugar && self.has_coffee && self.has_cream
	}
	/*Drink the beverage and get the appropriate message*/
	pub fn drink(&self) -> String {
		/*if the player can win, return the correct message*/
		let message:String;
		if self.can_win() {
			message = "You drink the beverage and are ready to study!\nYou Win!".to_string();
		}
		else if self.has_coffee && !self.has_cream {
			message = "Without cream, you get an ulcer and cannot study.\nYou lose!".to_string(); /*Have just coffee*/
		}
		else if self.has_coffee {
			message = "Without sugar, the coffee is too bitter. You cannot study.\nYou lose!".to_string(); /*Coffee and cream*/
		}
		else if self.has_cream && !self.has_sugar {
			message = "You can drink the cream, but without caffeine, you cannot study.\nYou lose!".to_string(); /*cream */
		}
		else if self.has_cream {
			message = "You can drink the sweetened cream, but without caffeine, you cannot study.\nYou lose!".to_string(); /*Cream and sugar*/
		}
		else if self.has_sugar {
			message = "You eat the sugar, but without caffeine, you cannot study.\nYou lose!".to_string(); /*sugar*/
		}
		else {
			message = "You drink the air, as you have no coffee, sugar, or cream.\nThe air is invigorating, but not invigorating enough. You cannot study.\nYou lose!".to_string();
		}
		message
	}
	pub fn show_inventory(&self) -> String {
		let mut message:String = "".to_string();
		if !self.has_sugar && !self.has_cream && !self.has_coffee {
			message.push_str("\nYou do not have any items in your inventory.");
		}
		if self.has_cream {
			message.push_str("\nYou have some fresh cream.");
		}
		if self.has_sugar {
			message.push_str("\nYou have some tasty sugar.");
		}
		if self.has_coffee {
			message.push_str("\nYou have a cup of delicious coffee.");
		}
		message

	}
}
/*Test Cases*/
/*Initial Test Case to prove that Cargo Test works*/
#[test]
fn it_works() {
    assert_eq!(4, 4);
}
/*Test that the Constructor does everything it needs to*/
#[test]
#[should_panic(expected = "assertion failed")] /*Mark that this test cases assertions should fail*/
fn test_constructor() {
	let p = Player::new();
	/*Verify the player does not have cream, coffee or sugar*/
	assert!(p.has_cream);
	assert!(p.has_coffee);
	assert!(p.has_sugar);
}

/*
* Test that get cream gets the cream and sets the has_cream to true
* Also verify that no other items get set
*/
#[test]
fn test_get_cream() {
	let mut p = Player::new();
	p.get_cream();
	/*verify the player only has cream*/
	assert!(p.has_cream);
	assert!(!p.has_coffee);
	assert!(!p.has_sugar);
}

/*
* Test that get_coffee sets the has_coffee to true
* Also verify that no other items get set to true
*/
#[test]
fn test_get_coffee() {
	let mut p = Player::new();
	p.get_coffee();
	/*verify the player only has coffee*/
	/*Also try using assert_eq for a false*/
	assert_eq!(p.has_cream,false);
	assert_eq!(p.has_coffee,true);
	assert_eq!(p.has_sugar,false);
}

/*
* Test that get_sugar sets the has_sugar to true
* Also verify that no other items get set to true
*/
#[test]
fn test_get_sugar() {
	let mut p = Player::new();
	p.get_sugar();
	/*verify the player only has sugar*/
	/*Also try using assert_eq for a false*/
	assert_eq!(p.has_cream,false);
	assert_eq!(p.has_coffee,false);
	assert_eq!(p.has_sugar,true);
}

/*
* Test that if the player has all three items,
* they can win the game
*/
#[test]
fn test_can_win() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get all the items*/
	p.get_cream();
	p.get_coffee();
	p.get_sugar();
	assert!(p.can_win());
}
/*
* Test that if the player has only cream
* they cannot win the game
*/
#[test]
fn test_can_win_cream() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get only cream*/
	p.get_cream();
	assert_eq!(p.can_win(),false);
}
/*
* Test that if the player has cream and sugar
* they cannot win the game
*/
#[test]
fn test_can_win_cream_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream and sugar*/
	p.get_cream();
	p.get_sugar();
	assert_eq!(p.can_win(),false);
}
/*
* Test that if the player has nothing
* they cannot win the game
*/
#[test]
fn test_can_win_nothing() {
	/*initialize the player as mutable*/
	let p = Player::new();
	assert_eq!(p.can_win(),false);
}
/*
* Test that if the player has sugar and coffee
* they cannot win the game
*/
#[test]
fn test_can_win_coffee_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get coffee and sugar*/
	p.get_coffee();
	p.get_sugar();
	assert_eq!(p.can_win(),false);
}
/*
* Test that if the player has all three items
* they can win the game, and the correct string is returned
*/
#[test]
fn test_drink_win() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get all the items*/
	p.get_cream();
	p.get_coffee();
	p.get_sugar();
	/*Drink what you have*/
	let message:String = p.drink();
	assert!(p.can_win());
	assert_eq!(message,"You drink the beverage and are ready to study!\nYou Win!");
}
/*
* Test that if the player has only coffee
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_coffee() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get only coffee*/
	p.get_coffee();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cannot win the game*/
	assert_eq!(message,"Without cream, you get an ulcer and cannot study.\nYou lose!"); /*verify the message is correct*/
}
/*
* Test that if the player has coffee and cream
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_coffee_cream() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get coffee and cream*/
	p.get_coffee();
	p.get_cream();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cannot win the game*/
	assert_eq!(message,"Without sugar, the coffee is too bitter. You cannot study.\nYou lose!"); /*verify the message is correct*/
}
/*
* Test that if the player has cream
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_cream() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_cream();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cannot win the game*/
	assert_eq!(message,"You can drink the cream, but without caffeine, you cannot study.\nYou lose!"); /*verify the message is correct*/
}

/*
* Test that if the player has cream and sugar
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_cream_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream and sugar*/
	p.get_cream();
	p.get_sugar();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cannot win the game*/
	assert_eq!(message,"You can drink the sweetened cream, but without caffeine, you cannot study.\nYou lose!"); /*verify the message is correct*/
}

/*
* Test that if the player has sugar
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get sugar*/
	p.get_sugar();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cannot win the game*/
	assert_eq!(message,"You eat the sugar, but without caffeine, you cannot study.\nYou lose!"); /*verify the message is correct*/
}

/*
* Test that if the player has nothing
* they cannot win the game, and the correct string is returned
*/
#[test]
fn test_drink_air() {
	/*initialize the player*/
	let p = Player::new();
	/*Drink what you have*/
	let message:String = p.drink();
	assert_eq!(p.can_win(),false); /*Verify the player cantanot win the game*/
	assert_eq!(message,"You drink the air, as you have no coffee, sugar, or cream.\nThe air is invigorating, but not invigorating enough. You cannot study.\nYou lose!"); /*verify the message is correct*/
}

/*
* Test that if the player has nothing
* their inventory returns nothing
*/
#[test]
fn test_show_inventory_nothing() {
	/*initialize the player*/
	let p = Player::new();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou do not have any items in your inventory."); /*verify the message is correct*/
}
/*
* Test that if the player has cream
* their inventory returns cream
*/
#[test]
fn test_show_inventory_cream() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_cream();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have some fresh cream."); /*verify the message is correct*/
}
/*
* Test that if the player has coffee
* their inventory returns coffee
*/
#[test]
fn test_show_inventory_coffee() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_coffee();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have a cup of delicious coffee."); /*verify the message is correct*/
}
/*
* Test that if the player has sugar
* their inventory returns sugar
*/
#[test]
fn test_show_inventory_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_sugar();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have some tasty sugar."); /*verify the message is correct*/
}

/*
* Test that if the player has cream and sugar
* their inventory returns cream and sugar
*/
#[test]
fn test_show_inventory_cream_and_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_sugar();
	p.get_cream();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have some fresh cream.\nYou have some tasty sugar."); /*verify the message is correct*/
}
/*
* Test that if the player has cream and coffee
* their inventory returns cream and coffee
*/
#[test]
fn test_show_inventory_cream_and_coffee() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_cream();
	p.get_coffee();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have some fresh cream.\nYou have a cup of delicious coffee."); /*verify the message is correct*/
}
/*
* Test that if the player has cream and coffee
* their inventory returns cream and coffee
*/
#[test]
fn test_show_inventory_cream_and_coffee_and_sugar() {
	/*initialize the player as mutable*/
	let mut p = Player::new();
	/*get cream*/
	p.get_cream();
	p.get_coffee();
	p.get_sugar();
	/*Drink what you have*/
	let message:String = p.show_inventory();
	assert_eq!(message,"\nYou have some fresh cream.\nYou have some tasty sugar.\nYou have a cup of delicious coffee."); /*verify the message is correct*/
}
