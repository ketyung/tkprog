use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program::invoke,
       // sysvar::{rent::Rent, Sysvar},
    
    },
    
    spl_token::instruction::{initialize_mint,mint_to},
  
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


fn mint_token(accounts: &[AccountInfo],token_count : u64 )-> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let signer_account = next_account_info(account_info_iter)?;

    let token_account = next_account_info(account_info_iter)?; 
   
    /*
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    if !rent.is_exempt(signer_account.lamports(), signer_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }*/

    let token_program = next_account_info(account_info_iter)?;
   
    if *token_account.owner != spl_token::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

   
    msg!("To.mint::{}", token_count);

    msg!("signer.acc:{:?}", signer_account.key);

   // msg!("sol in acc:{:?}", signer_account.lamports );
       
    msg!("tk.acc:{:?}", token_account.key);
    
    msg!("tk_prog.acc:{:?}", token_program.key);
    
    let ix = initialize_mint(
        &spl_token::ID,
        &token_account.key,
        &signer_account.key,
        Some(signer_account.key),
        9,
    ).unwrap();

    invoke(&ix,  &[
        token_account.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;

    let ix = mint_to(
        &spl_token::ID,
        &token_account.key, // mint pubkey
        &signer_account.key, // account pubkey
        &signer_account.key, // owner pubkey
        &[],            // signers
        token_count,
    )?;

    invoke(&ix,  &[
        token_account.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;


    Ok(())
}


