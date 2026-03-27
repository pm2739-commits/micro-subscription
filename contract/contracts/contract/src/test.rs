#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, log
};

#[contract]
pub struct MicroSub;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Subscription(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    rate_per_sec: i128,
    start_time: u64,
    active: bool,
}

#[contractimpl]
impl MicroSub {

    // 🔹 Deposit funds
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let key = DataKey::Balance(user.clone());
        let bal: i128 = env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &(bal + amount));
    }

    // 🔹 Start subscription
    pub fn start(env: Env, user: Address, rate_per_sec: i128) {
        user.require_auth();

        let sub = Subscription {
            rate_per_sec,
            start_time: env.ledger().timestamp(),
            active: true,
        };

        env.storage().instance().set(&DataKey::Subscription(user), &sub);
    }

    // 🔹 Stop subscription and charge user
    pub fn stop(env: Env, user: Address) {
        user.require_auth();

        let sub_key = DataKey::Subscription(user.clone());
        let mut sub: Subscription = env.storage()
            .instance()
            .get(&sub_key)
            .expect("No active sub");

        if !sub.active {
            panic!("Already stopped");
        }

        let current_time = env.ledger().timestamp();
        let duration = current_time - sub.start_time;

        let cost = (duration as i128)
            .checked_mul(sub.rate_per_sec)
            .expect("Overflow");

        let bal_key = DataKey::Balance(user.clone());
        let balance: i128 = env.storage().instance().get(&bal_key).unwrap_or(0);

        if balance < cost {
            panic!("Not enough balance");
        }

        env.storage().instance().set(&bal_key, &(balance - cost));

        sub.active = false;
        env.storage().instance().set(&sub_key, &sub);

        log!(&env, "Subscription stopped and charged");
    }

    // 🔹 Check balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(user)).unwrap_or(0)
    }
}
