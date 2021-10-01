use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program::invoke,
    },
    
    spl_token::instruction::{initialize_mint},//,mint_to} 
  
    arrayref::{array_ref,  array_refs},

};


pub fn process_instruction(_program_id: &Pubkey,accounts: &[AccountInfo], instruction_data: &[u8],) 
-> ProgramResult {

    const L : usize = 9; 
    let output = array_ref![instruction_data, 0, L];
    
    let (mode,amount) = array_refs![output, 1, 8 ];
  
    let mode = u8::from_le_bytes(*mode);

    let token_count = u64::from_le_bytes(*amount);

  
    match mode {

        1 => {

            mint_token(accounts, token_count)
        },

        _ => {
            msg!("None mode:{}", mode );
            Ok(())
        },

    }
}

fn mint_token(accounts: &[AccountInfo], _token_count : u64 )-> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let signer_account = next_account_info(account_info_iter)?;

    let token_account = next_account_info(account_info_iter)?; 
    
    let token_program = next_account_info(account_info_iter)?;

    if *token_account.owner != spl_token::id() {

        return Err(ProgramError::IncorrectProgramId);
    }

    let init_mint_ins = initialize_mint(
        &spl_token::ID,
        &token_account.key,
        &signer_account.key,
        Some(signer_account.key),
        3,
    )
    .unwrap();

    invoke(&init_mint_ins,  &[
        token_account.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;

    Ok(())
}


