//! A joke hook that doubles incoming XRP payments 
//! and sends it back

#![no_std]
#![no_main]

use core::ops::Range;

use xrpl_hooks::*;

const GUARD_ID_1: u32 = line!();

#[no_mangle]
pub extern "C" fn cbak(_: i64) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: i64) -> i64 {
    let mut hook_acc_id: AccountId = uninit_buf!();
    // Account field from the originating transaction
    let mut otnx_acc_id: AccountId = uninit_buf!();

    match is_txn_outgoing::<GUARD_ID_1>(&mut hook_acc_id, &mut otnx_acc_id) {
        Err(_) => rollback(b"Could not fetch accounts id", 0),
        Ok(true) => accept(b"Outgoing transaction. Passing.", 0),
        Ok(false) => {}
    };

    let mut digest: Buffer<96> = uninit_buf!();

    const PART_1: Range<usize> = Range {start: 0, end: 32};
    const PART_2: Range<usize> = Range {start: 32, end: 64};
    const PART_3: Range<usize> = Range {start: 64, end: 96};

    ledger_last_hash(&mut digest[PART_1])
        .expect(b"Failed to fetch last closed ledger");

    let key: StateKey = uninit_buf!();

    // If this load fails then we don't care, the hash is just 0
    let _ = state(&mut digest[PART_2], &key);

    nonce(&mut digest[PART_3]).unwrap();

    let mut hash: Hash = uninit_buf!();
    util_sha512h(&mut hash, &digest)
        .expect(b"Could not compute digest for coin flip");

    state_set(&hash, &key)
        .expect(b"Could not set state");

    // First digit of lcl hash is our biased coin flip, you lose 60% of the time :P
    if hash[0] % 10 < 6 {
        accept(b"Tails, you lose. Om nom nom xrp.", 4)
    }

    // Before we start calling hook-api functions we should tell 
    // the hook how many tx we intend to create.
    // We are going to emit 1 transaction.
    etxn_reserve(1).unwrap();

    // Fetch the sent Amount.
    // Amounts can be 384 bits or 64 bits. 
    // If the Amount is an XRP value it will be 64 bits.
    let mut amount_buffer: Amount = uninit_buf!();
    let amount_len = otxn_field(&mut amount_buffer, FieldId::Amount).unwrap();
    if amount_len != 8 {
        rollback(b"Rejecting incoming non-XRP transaction", 5)
    }

    // Doubler pays back 2x received
    let drops_to_send = amount_to_drops(&amount_buffer).unwrap().saturating_mul(2);
    let fee_base = etxn_fee_base(PREPARE_PAYMENT_SIMPLE_SIZE as _).unwrap();
    let mut tx: TxnPaymentSimple = uninit_buf!();
    prepare_payment_simple(&mut tx, drops_to_send, fee_base, &otnx_acc_id, 0, 0)
        .unwrap();

    let mut emithash: Hash = uninit_buf!();
    emit(&mut emithash, &tx).unwrap();

    accept(b"Heads, you won! Funds emitted!", 0);
}
