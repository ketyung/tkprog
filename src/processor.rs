use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program_pack::{Pack},
    },
    
    spl_token::instruction::{initialize_mint},//,mint_to} 
  
};


pub fn process_instruction(program_id: &Pubkey,accounts: &[AccountInfo], instruction_data: &[u8],) 
-> ProgramResult {

    const L : usize = 9; 
    let output = array_ref![instruction_data, 0, L];
    
    let (mode,amount) = array_refs![output, 1, 8 ];
  
    let mode = u8::from_le_bytes(*mode);

    let token_count = u64::from_le_bytes(*amount);

    match mode {

        1 => {

            let signer_account = next_account_info(account_info_iter)?;

            let token_account = next_account_info(account_info_iter)?; // expecting the last acc is the token acc

            mint_token(signer_account, token_account, token_count);

        },

        _ => msg!("None mode:{}", mode ),

    }
}

fn mint_token(signer_account : &AccountInfo, token_account : &AccountInfo, token_count : u64 ){

    let _init_mint_ins = initialize_mint(
        &spl_token::ID,
        &token_account.key,
        &signer_account.key,
        Some(signer_account.key),
        3,
    )
    .unwrap();


}