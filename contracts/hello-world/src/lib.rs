#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Address, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

#[contracttype]
pub enum LeaderboardKey {
    Player(Address),
    AllPlayers,
}

#[contract]
pub struct OnChainLeaderboard;

#[contractimpl]
impl OnChainLeaderboard {
    // Submit or update player score
    pub fn submit_score(env: Env, player_address: Address, name: String, score: u32) {
        let player = Player { name, score };
        env.storage().instance().set(&LeaderboardKey::Player(player_address.clone()), &player);

        // Update the player list if it's not already present
        let mut all_players: Vec<Address> = env.storage().instance().get(&LeaderboardKey::AllPlayers).unwrap_or(Vec::new(&env));
        if !all_players.contains(&player_address) {
            all_players.push_back(player_address);
            env.storage().instance().set(&LeaderboardKey::AllPlayers, &all_players);
        }
    }

    // View player's score
    pub fn get_score(env: Env, player_address: Address) -> Player {
        env.storage().instance().get(&LeaderboardKey::Player(player_address)).unwrap()
    }

    // View all leaderboard entries (unordered)
    pub fn get_all_players(env: Env) -> Vec<(Address, Player)> {
        let addresses: Vec<Address> = env.storage().instance().get(&LeaderboardKey::AllPlayers).unwrap_or(Vec::new(&env));
        let mut entries = Vec::new(&env);

        for addr in addresses.iter() {
            if let Some(player) = env.storage().instance().get(&LeaderboardKey::Player(addr.clone())) {
                entries.push_back((addr.clone(), player));
            }
        }

        entries
    }
}
