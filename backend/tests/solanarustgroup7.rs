use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};

use solanarustgroup7::*;

#[tokio::test]
async fn test_initialize() {
    // Initialize the test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "solanarustgroup7",
        program_id,        
        processor!(solanarustgroup7::entry),
    )
    .start_with_context()
    .await;

    // Build the instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![],
        data: solanarustgroup7::instruction::Initialize.data(), // Call the `initialize` function
    };

    // Create a transaction
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    // Process the transaction
    context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap();

    println!("Program initialized successfully!");
}

#[tokio::test]
async fn test_create_campaign() {
    // Initialize the test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "solanarustgroup7", // Program name
        program_id,         // Program ID
        processor!(solanarustgroup7::entry), // Entry point
    )
    .start_with_context()
    .await;

    // Create accounts
    let campaign_keypair = Keypair::new();

    // Create the instruction
    let instruction = solanarustgroup7::instruction::CreateCampaign {
        name: "Test Campaign".to_string(),
        description: "This is a test description".to_string(),
        target_amount: 1000,
    };

    // --------Accounts--------
    let accounts = solanarustgroup7::accounts::CreateCampaign {
        campaign: campaign_keypair.pubkey(),
        creator: context.payer.pubkey(),
        system_program: solana_sdk::system_program::ID,
    }
    .to_account_metas(None);

    //--------transaction--------
    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts,
            data: instruction.data(), 
        }],
        Some(&context.payer.pubkey()),
        &[&context.payer, &campaign_keypair],
        context.last_blockhash,
    );

    //--------Process the transaction--------
    context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap();

    //--------Fetch and verify the campaign account--------
    let campaign_account: Account = context
        .banks_client
        .get_account(campaign_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let campaign: Campaign =
        Campaign::try_deserialize(&mut campaign_account.data.as_ref()).unwrap();

    assert_eq!(campaign.name, "Test Campaign");
    assert_eq!(campaign.description, "This is a test description");
    assert_eq!(campaign.target_amount, 1000);
    assert_eq!(campaign.current_funds, 0);

    println!("Campaign created successfully!");
}
