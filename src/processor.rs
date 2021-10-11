/**
 * A small program for me to test the token transfer
 * By Christopher K Y Chee ketyung@techchee.com
 */
use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
        program::{invoke,invoke_signed},
       // sysvar::{rent::Rent, Sysvar},
    
  
    },
    
   // spl_associated_token_account::get_associated_token_address,

    spl_token::instruction::{/*initialize_mint,*/mint_to},
  
    arrayref::{array_ref,  array_refs},

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

            mint_token(program_id, accounts, token_count)
        },

        2 => {
            tx_to(program_id, accounts, token_count)
        },
        _ => {
            msg!("None mode:{}", mode );
            Ok(())
        },

    }
}


fn mint_token(program_id: &Pubkey, accounts: &[AccountInfo],token_count : u64 )-> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    let signer_account = next_account_info(account_info_iter)?;

    let token_mint = next_account_info(account_info_iter)?; 
    
    let token_account = next_account_info(account_info_iter)?; 
        
    let token_program = next_account_info(account_info_iter)?;
   
    /* // no need to check!

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

    
   
    msg!("To.mint::{}", token_count);

    /*
    msg!("signer.acc:{:?}", signer_account.key);

    msg!("tk.mint:{:?}", token_mint.key);
  
    msg!("tk.acc:{:?}", token_account.key);
    
    msg!("tk_prog.acc:{:?}", token_program.key);

    let ata = get_associated_token_address(&signer_account.key, 
        &token_account.key);
    msg!("associated token acc: {:?}",ata );
    */
  
    let ix = mint_to(
        token_program.key,
        token_mint.key,
        token_account.key,
        signer_account.key,
        &[],
        token_count,
    )?;


    let signers = &[
        signer_account.key.as_ref(),
    ];

    invoke_signed(
        &ix,
        &[
            token_mint.clone(),
            token_account.clone(),
            signer_account.clone(),
            token_program.clone(),
        ],
        &[signers],
    )?;


    // tx the token to a PDA that is derived from the 
    // account 
    let addr = &[token_account.key.as_ref()];
    let (pda, _bump_seed) = Pubkey::find_program_address(addr, program_id);
    // need to store the token account, the mint 

    let owner_change_ix = spl_token::instruction::set_authority(
        token_program.key,
        token_account.key,
        Some(&pda),
        spl_token::instruction::AuthorityType::AccountOwner,
        signer_account.key,
        &[&signer_account.key],
    )?;
    
    msg!("Calling the token program to transfer token account from [{:?}] ownership...to [{:?}]",
    token_account.key, Some(&pda));

    msg!("Bumpseed is {:?}", _bump_seed);

    invoke(
        &owner_change_ix,
        &[
            token_account.clone(),
            signer_account.clone(),
            token_program.clone(),
        ],
    )?;
    /*
    let ix = mint_to(
        &spl_token::ID,
        &token_mint.key, // mint pubkey
        //&ata,  
        &token_account.key, 
        &signer_account.key, // account pubkey
       // &signer_account.key, // owner pubkey
        &[],            // signers
        token_count,
    )?;

    invoke(&ix,  &[
        token_mint.clone(),
        signer_account.clone(),
        token_program.clone(),
    ])?;*/

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


fn tx_to(program_id: &Pubkey, accounts: &[AccountInfo],token_count : u64) -> ProgramResult{


    let account_info_iter = &mut accounts.iter();

    let pda_account = next_account_info(account_info_iter)?;

    let receiver_account = next_account_info(account_info_iter)?; 
    
    let token_account = next_account_info(account_info_iter)?; 
        
    let token_program = next_account_info(account_info_iter)?;

    let addr = &[token_account.key.as_ref()];
       
    let (pda, bump_seed) = Pubkey::find_program_address(addr, program_id);

    msg!("Goin.to.tx ::{}", token_count);
       
    let tf_to_receiver_ix = spl_token::instruction::transfer(
        token_program.key,
        token_account.key,
        receiver_account.key,
        &pda,
        &[&pda],
        token_count,
    )?;


    invoke_signed(&tf_to_receiver_ix,
        &[
            token_account.clone(),
            receiver_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[&[&token_account.key.as_ref()[..], &[bump_seed]]],
    )?;

    msg!("Successfully tx to ::{:?}", receiver_account.key);
    
    
    Ok(())
}

