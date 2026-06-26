#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// Storage keys for the contract
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Stores escrow details by escrow ID
    Escrow(u64),
    /// Counter for generating unique escrow IDs
    EscrowCounter,
}

/// Represents the state of an escrow
#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
#[repr(u32)]
pub enum EscrowStatus {
    AwaitingDelivery = 0,
    Confirmed = 1,
    Disputed = 2,
    Released = 3,
    Refunded = 4,
}

/// Stores all details of a single escrow transaction
#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    pub id: u64,
    pub buyer: Address,         // Maria (the sari-sari store owner)
    pub seller: Address,        // The vegetable supplier
    pub amount: i128,           // Amount in stroops (USDC smallest unit)
    pub status: EscrowStatus,
}

#[contract]
pub struct PalengkePassContract;

#[contractimpl]
impl PalengkePassContract {
    /// Creates a new escrow. The buyer deposits funds that are locked until
    /// they confirm delivery or raise a dispute.
    ///
    /// # Arguments
    /// * `buyer` - Address of the buyer (e.g., Maria)
    /// * `seller` - Address of the supplier
    /// * `amount` - Amount to lock in escrow (in USDC stroops)
    ///
    /// # Returns
    /// The unique escrow ID
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        amount: i128,
    ) -> u64 {
        // Buyer must authorize this transaction
        buyer.require_auth();

        // Amount must be positive
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Get and increment the escrow counter
        let escrow_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EscrowCounter)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&DataKey::EscrowCounter, &(escrow_id + 1));

        // Create the escrow record
        let escrow = Escrow {
            id: escrow_id,
            buyer: buyer.clone(),
            seller: seller.clone(),
            amount,
            status: EscrowStatus::AwaitingDelivery,
        };

        // Store the escrow
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);

        // Emit event for tracking
        env.events().publish(
            (symbol_short!("created"), escrow_id),
            (buyer, seller, amount),
        );

        escrow_id
    }

    /// Buyer confirms delivery, releasing funds to the seller.
    ///
    /// # Arguments
    /// * `escrow_id` - The ID of the escrow to confirm
    pub fn confirm_delivery(env: Env, escrow_id: u64) {
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("Escrow not found");

        // Only the buyer can confirm
        escrow.buyer.require_auth();

        // Must be in awaiting delivery state
        if escrow.status != EscrowStatus::AwaitingDelivery {
            panic!("Escrow not in awaiting delivery state");
        }

        // Update status to confirmed, then released
        escrow.status = EscrowStatus::Released;

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);

        // Emit release event (actual token transfer would happen here in production)
        env.events().publish(
            (symbol_short!("released"), escrow_id),
            escrow.seller.clone(),
        );
    }

    /// Buyer raises a dispute, freezing the funds for resolution.
    ///
    /// # Arguments
    /// * `escrow_id` - The ID of the escrow to dispute
    pub fn raise_dispute(env: Env, escrow_id: u64) {
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("Escrow not found");

        // Only the buyer can raise a dispute
        escrow.buyer.require_auth();

        // Must be in awaiting delivery state
        if escrow.status != EscrowStatus::AwaitingDelivery {
            panic!("Escrow not in awaiting delivery state");
        }

        escrow.status = EscrowStatus::Disputed;

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);

        env.events().publish(
            (symbol_short!("disputed"), escrow_id),
            escrow.buyer.clone(),
        );
    }

    /// Resolves a disputed escrow. In production, this would be called by an
    /// arbitrator or through a governance mechanism.
    ///
    /// # Arguments
    /// * `escrow_id` - The ID of the disputed escrow
    /// * `release_to_seller` - If true, release to seller; if false, refund buyer
    pub fn resolve_dispute(env: Env, escrow_id: u64, release_to_seller: bool) {
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("Escrow not found");

        // Must be in disputed state
        if escrow.status != EscrowStatus::Disputed {
            panic!("Escrow not in disputed state");
        }

        if release_to_seller {
            escrow.status = EscrowStatus::Released;
            env.events().publish(
                (symbol_short!("resolved"), escrow_id),
                (Symbol::new(&env, "seller"), escrow.seller.clone()),
            );
        } else {
            escrow.status = EscrowStatus::Refunded;
            env.events().publish(
                (symbol_short!("resolved"), escrow_id),
                (Symbol::new(&env, "buyer"), escrow.buyer.clone()),
            );
        }

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    /// Retrieves the current state of an escrow.
    ///
    /// # Arguments
    /// * `escrow_id` - The ID of the escrow to query
    ///
    /// # Returns
    /// The Escrow struct with all details
    pub fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("Escrow not found")
    }
}
