
```
ablack94@ab-ubuntu-laptop:~/t3test$ cargo build
   Compiling t3 v0.1.0 (/home/ablack94/t3test)
warning: unused imports: `entrypoint::ProgramResult`, `msg`, `next_account_info`
 --> src/lib.rs:2:20
  |
2 |     account_info::{next_account_info, AccountInfo},
  |                    ^^^^^^^^^^^^^^^^^
3 |     entrypoint::ProgramResult,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
4 |     msg,
  |     ^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `deposit_keys`
  --> src/lib.rs:91:9
   |
91 |     let deposit_keys = MangoDepositKeys::new(&all_keys, &zero_ai);
   |         ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_deposit_keys`
   |
   = note: `#[warn(unused_variables)]` on by default

error[E0597]: `zero_pk` does not live long enough
  --> src/lib.rs:80:9
   |
71 |       accounts: &[AccountInfo],
   |       -------- has type `&[AccountInfo<'1>]`
...
79 |       let zero_ai : AccountInfo = AccountInfo::new(
   |  _________________________________-
80 | |         &zero_pk,
   | |         ^^^^^^^^ borrowed value does not live long enough
81 | |         false,
82 | |         false,
...  |
87 | |         0
88 | |     );
   | |_____- argument requires that `zero_pk` is borrowed for `'1`
...
94 |   }
   |   - `zero_pk` dropped here while still borrowed

error[E0597]: `zero_lamports` does not live long enough
  --> src/lib.rs:83:9
   |
71 |       accounts: &[AccountInfo],
   |       -------- has type `&[AccountInfo<'1>]`
...
79 |       let zero_ai : AccountInfo = AccountInfo::new(
   |  _________________________________-
80 | |         &zero_pk,
81 | |         false,
82 | |         false,
83 | |         &mut zero_lamports,
   | |         ^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
...  |
87 | |         0
88 | |     );
   | |_____- argument requires that `zero_lamports` is borrowed for `'1`
...
94 |   }
   |   - `zero_lamports` dropped here while still borrowed

error[E0597]: `zero_data` does not live long enough
  --> src/lib.rs:84:9
   |
71 |       accounts: &[AccountInfo],
   |       -------- has type `&[AccountInfo<'1>]`
...
79 |       let zero_ai : AccountInfo = AccountInfo::new(
   |  _________________________________-
80 | |         &zero_pk,
81 | |         false,
82 | |         false,
83 | |         &mut zero_lamports,
84 | |         &mut zero_data,
   | |         ^^^^^^^^^^^^^^ borrowed value does not live long enough
...  |
87 | |         0
88 | |     );
   | |_____- argument requires that `zero_data` is borrowed for `'1`
...
94 |   }
   |   - `zero_data` dropped here while still borrowed

For more information about this error, try `rustc --explain E0597`.
warning: `t3` (lib) generated 2 warnings
error: could not compile `t3` due to 3 previous errors; 2 warnings emitted

```

```
ablack94@ab-ubuntu-laptop:~/t3test$ rustc works.rs 
warning: unused variable: `deposit_keys`
  --> works.rs:97:9
   |
97 |     let deposit_keys = MangoDepositKeys::new(&all_keys, &zero_ai);
   |         ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_deposit_keys`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `data1`
   --> works.rs:115:13
    |
115 |     let mut data1 : u64 = 0;
    |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_data1`

warning: unused variable: `data2`
   --> works.rs:116:13
    |
116 |     let mut data2 : u64 = 1;
    |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_data2`

warning: variable does not need to be mutable
   --> works.rs:115:9
    |
115 |     let mut data1 : u64 = 0;
    |         ----^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
   --> works.rs:116:9
    |
116 |     let mut data2 : u64 = 1;
    |         ----^^^^^
    |         |
    |         help: remove this `mut`

warning: field is never read: `data`
  --> works.rs:22:5
   |
22 |     data: [u8;32],
   |     ^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: field is never read: `mango_program_id`
  --> works.rs:58:5
   |
58 |     mango_program_id: AccountInfo<'a>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: field is never read: `open_order_ais`
  --> works.rs:59:5
   |
59 |     open_order_ais: Vec<AccountInfo<'a>>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 8 warnings emitted
```
