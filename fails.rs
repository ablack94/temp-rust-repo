

/**
 * Constants
 */
static ZERO_PUBKEY : Pubkey = Pubkey::new_from_array([0u8; 32]);

#[derive(Clone)]
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
    pub cell: std::rc::Rc::<std::cell::RefCell<&'a mut u64>>,
}

impl<'a> AccountInfo<'a>
{
    pub fn new(key: &'a Pubkey, cell: &'a mut u64) -> AccountInfo<'a>
    {
        return AccountInfo{
            key: key,
            cell: std::rc::Rc::<std::cell::RefCell<&'a mut u64>>::new(std::cell::RefCell::new(cell)),
        };
    }
}

pub struct Pubkey
{
    data: [u8;32],
}

impl Pubkey
{
    pub const fn new_from_array(value: [u8;32]) -> Pubkey
    {
        return Pubkey{
            data: value,
        };
    }
}




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
    let mut data : u64 = 0;
    let pk : Pubkey = Pubkey::new_from_array([1u8; 32]);

    let zero_ai : AccountInfo = AccountInfo::new(
        &pk,
        &mut data,
        //false,
        //false,
        //&mut zero_lamports,
        //&mut zero_data,
        //&ZERO_PUBKEY,
        //false,
        //0
    );

    let all_keys = AllKeys::new(accounts);
    let deposit_keys = MangoDepositKeys::new(&all_keys, &zero_ai);

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
    let mut data1 : u64 = 0;
    let mut data2 : u64 = 1;

    process(
        &ZERO_PUBKEY,
        &[
            AccountInfo::new(&ZERO_PUBKEY, &mut data1),
            AccountInfo::new(&ZERO_PUBKEY, &mut data2),
        ],
        &[]
    );

}
