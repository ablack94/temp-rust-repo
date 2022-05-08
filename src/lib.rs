use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};


/**
 * Constants
 */
static ZERO_PUBKEY : Pubkey = Pubkey::new_from_array([0u8; 32]);

/*
#[derive(Clone)]
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
}

impl<'a> AccountInfo<'a>
{
    pub const fn new(key: &'a Pubkey) -> AccountInfo<'a>
    {
        return AccountInfo{key: key};
    }
}
*/


struct AllKeys<'a>
{
    mango_program_id: AccountInfo<'a>,
    owner_token_input_ai: AccountInfo<'a>,
}

impl<'a> AllKeys<'a>
{
    fn new(accounts: &[AccountInfo<'a>]) -> AllKeys<'a>
    {
        let mut it = accounts.iter();
        return AllKeys{
            mango_program_id: it.next().unwrap().clone(),
            owner_token_input_ai: it.next().unwrap().clone(),
        }
    }
}

struct MangoDepositKeys<'a>
{
    mango_program_id: AccountInfo<'a>,
    open_order_ais: Vec<AccountInfo<'a>>,
}

impl<'a> MangoDepositKeys<'a>
{
    fn new(all_keys: &AllKeys<'a>, zero_ai: &AccountInfo<'a>) -> MangoDepositKeys<'a>
    {
        return MangoDepositKeys{
            mango_program_id: all_keys.mango_program_id.clone(),
            open_order_ais: vec![
                all_keys.owner_token_input_ai.clone(),
                zero_ai.clone(),
            ],
        };
    }
}

pub fn process2(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
)
{
    let zero_pk : Pubkey = Pubkey::new_from_array([0u8; 32]);
    let mut zero_lamports: u64 = 0u64;
    let mut zero_data: [u8; 0] = [];
    {
    let zero_ai : AccountInfo = AccountInfo::new(
        &zero_pk,
        false,
        false,
        &mut zero_lamports,
        &mut zero_data,
        &ZERO_PUBKEY,
        false,
        0
    );

    let all_keys = AllKeys::new(accounts);
    let deposit_keys = MangoDepositKeys::new(&all_keys, &zero_ai);
    }

}

pub fn process<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _instruction_data: &[u8],
)
{
    process2(
        _program_id,
        accounts,
        _instruction_data);
}

fn main()
{
    let acc1_pk : Pubkey = Pubkey::new_from_array([0u8; 32]);
    let mut acc1_lamports : u64 = 0u64;
    let mut acc1_data : [u8; 0] = [];

    let acc2_pk : Pubkey = Pubkey::new_from_array([1u8; 32]);
    let mut acc2_lamports : u64 = 1u64;
    let mut acc2_data : [u8; 0] = [];

    process(
        &ZERO_PUBKEY,
        &[
            AccountInfo::new(
                &acc1_pk,
                false,
                false,
                &mut acc1_lamports,
                &mut acc1_data,
                &ZERO_PUBKEY,
                false,
                0
            ),
            AccountInfo::new(
                &acc2_pk,
                false,
                false,
                &mut acc2_lamports,
                &mut acc2_data,
                &ZERO_PUBKEY,
                false,
                0
            ),
        ],
        &[]
    );

}
