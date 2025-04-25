#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub is_listed: bool,
}

const NFT_COUNT: Symbol = symbol_short!("NFTC");

#[contract]
pub struct DigitalArtNFTContract;

#[contractimpl]
impl DigitalArtNFTContract {
    pub fn mint(env: Env, title: String, artist: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&NFT_COUNT).unwrap_or(0);
        count += 1;

        let nft = NFT {
            id: count,
            title,
            artist,
            is_listed: false,
        };

        env.storage().instance().set(&count, &nft);
        env.storage().instance().set(&NFT_COUNT, &count);

        count
    }

    pub fn get_nft(env: Env, id: u64) -> NFT {
        env.storage().instance().get(&id).unwrap()
    }

    pub fn list_nft(env: Env, id: u64) {
        let mut nft: NFT = env.storage().instance().get(&id).unwrap();
        nft.is_listed = true;
        env.storage().instance().set(&id, &nft);
    }

    pub fn unlist_nft(env: Env, id: u64) {
        let mut nft: NFT = env.storage().instance().get(&id).unwrap();
        nft.is_listed = false;
        env.storage().instance().set(&id, &nft);
    }
}
