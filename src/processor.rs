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



/*
fn process_exchange(
    accounts: &[AccountInfo],
    amount_expected_by_taker: u64,
    program_id: &Pubkey,
    
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let taker = next_account_info(account_info_iter)?;

    if !taker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let takers_sending_token_account = next_account_info(account_info_iter)?;

    let takers_token_to_receive_account = next_account_info(account_info_iter)?;

    let pdas_temp_token_account = next_account_info(account_info_iter)?;
    
    let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    
    let initializers_main_account = next_account_info(account_info_iter)?;
    let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;

    
    let token_program = next_account_info(account_info_iter)?;

    let transfer_to_initializer_ix = spl_token::instruction::transfer(
        token_program.key,
        takers_sending_token_account.key,
        initializers_token_to_receive_account.key,
        taker.key,
        &[&taker.key],
        amount_expected_by_taker,
    )?;
    msg!("Calling the token program to transfer tokens to the escrow's initializer...");
    invoke(
        &transfer_to_initializer_ix,
        &[
            takers_sending_token_account.clone(),
            initializers_token_to_receive_account.clone(),
            taker.clone(),
            token_program.clone(),
        ],
    )?;
    Ok(())
}


let transfer_to_taker_ix = spl_token::instruction::transfer(
    token_program.key,
    pdas_temp_token_account.key,
    takers_token_to_receive_account.key,
    &pda,
    &[&pda],
    pdas_temp_token_account_info.amount,
)?;
msg!("Calling the token program to transfer tokens to the taker...");
invoke_signed(
    &transfer_to_taker_ix,
    &[
        pdas_temp_token_account.clone(),
        takers_token_to_receive_account.clone(),
        pda_account.clone(),
        token_program.clone(),
    ],
    &[&[&b"escrow"[..], &[bump_seed]]],
)?;

*/