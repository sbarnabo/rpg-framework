use crate::models::{Player, Item, Quest};

pub fn handle_combat(player: &mut Player, monster_health: i32) -> i32 {
    // Simulate combat, for now just reduce player health by some amount
    player.take_damage(10);
    if player.health == 0 {
        println!("Player has died!");
    }
    return monster_health - 10; // Placeholder combat logic
}

pub fn complete_quest(player: &mut Player, quest: &mut Quest) {
    quest.complete();
    player.experience += 100; // Reward the player with experience
    if player.experience >= 1000 {
        player.level_up();
    }
}
