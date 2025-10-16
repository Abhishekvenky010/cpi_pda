use solana_program::{account_info::Accountinfo,Pubkey};
entrypoint!(process_instruction)
fn process_instruction(
    program_id : &Pubkey,
    accounts : &[Account_info],
    instruction_data: &[u8]
)->ProgramResult{
  let iter = &mut accounts.iter();
  let pda = next_account_info(iter)?;
  let user_acc = next_account_info(iter)?;
  let system_program = next_account_info(iter)?;
  let seeds = &[user_acc.key.as_ref(),b"user"]
  let ix = create_account(
    user_acc.key,pda.key,1000000000,8,program_id
  )
  invoke_signed(ix,accounts,&[seeds])
}

