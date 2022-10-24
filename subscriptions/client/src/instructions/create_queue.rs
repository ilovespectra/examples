use {
    crate::*,
    anchor_lang::{prelude::Pubkey, InstructionData},
    clockwork_sdk::client::{Client, ClientResult},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        system_program,
    },
};

pub fn create_queue(
    client: &Client,
    subscriber: Pubkey,
    subscription: Pubkey,
    subscription_queue: Pubkey,
) -> ClientResult<()> {
    let create_queue_ix = Instruction {
        program_id: subscriptions_program::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(subscriber, false),
            AccountMeta::new(subscription_queue, false),
            AccountMeta::new_readonly(subscription, false),
            AccountMeta::new_readonly(clockwork_crank::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: subscriptions_program::instruction::CreateQueue {}.data(),
    };

    send_and_confirm_tx(
        client,
        [create_queue_ix].to_vec(),
        None,
        "create_queue".to_string(),
    )?;

    Ok(())
}
