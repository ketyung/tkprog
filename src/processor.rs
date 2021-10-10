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
    
    spl_associated_token_account::get_associated_token_address,

    spl_token::instruction::{/*initialize_mint,*/mint_to},
  
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
        
    let token_program = next_account_info(account_info_iter)?;
   
    msg!("try no checking sysvar.rent!!");
    /* 

    let sys_var_account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(sys_var_account)?;

    if !rent.is_exempt(signer_account.lamports(), signer_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }
    else {

        msg!("Account is rent exempt!!, sys.var.acc is :{:?}...x", sys_var_account.key);//, sys_var_account.key);
    }
*/

    if *token_account.owner != spl_token::id() {
        msg!("token_account.owner is {:?}, whereas spl_token prog id is::{:?}", 
        token_account.owner, spl_token::id());
        return Err(ProgramError::IncorrectProgramId);
    }
    else {

        msg!("token_acc.owner is::{:?}", token_account.owner);
    }

   
    msg!("To.mint::{}", token_count);

    msg!("signer.acc:{:?}", signer_account.key);

    msg!("tk.acc:{:?}", token_account.key);
    
    msg!("tk_prog.acc:{:?}", token_program.key);

    let ata = get_associated_token_address(&signer_account.key, 
        &token_account.key);
    msg!("associated token acc: {:?}",ata );

    /*
     * 
     *  SPLToken.TOKEN_PROGRAM_ID, // 通常是固定值, token program id
      TEST_MINT, // mint
      ALICE_TOKEN_ADDRESS_1, // 收token的地址 (需要是token account)
      FEE_PAYER.publicKey, // mint 的 auth
      [],
     */

    let ix = mint_to(
        &spl_token::ID,
        &token_account.key, // mint pubkey
        //&ata,  
        &token_account.key, 
        &signer_account.key, // account pubkey
       // &signer_account.key, // owner pubkey
        &[],            // signers
        token_count,
    )?;

    invoke(&ix,  &[
        token_account.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;

    /*
    let ix = initialize_mint(
        &spl_token::ID,
        &token_account.key,
        &signer_account.key,
        Some(signer_account.key),
        2,
    ).unwrap();

    invoke(&ix,  &[
        token_account.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;

   
    */

    Ok(())
}


