//! Sends predefined percent of outgoing transaction amount
//! to Carbon address

#![no_std]
#![no_main]

use xrpl_hooks::*;

const GUARD_ID_1: u32 = line!();

// Carbon address
const CARBON_ADDRESS: &[u8] = b"rfCarbonVNTuXckX6x2qTMFmFSnm6dEWGX";
// This will be the min default to send
const MIN_DROPS: u64 = 1000;
// We send 1% of amount
const PERCENT: u8 = 1;

#[no_mangle]
pub extern "C" fn cbak(_: i64) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: i64) -> i64 {
    let _ = trace(b"Carbon: started", b"", DataRepr::AsUTF8);

    // Before we start calling hook-api functions we should tell 
    // the hook how many tx we intend to create.
    // We are going to emit 1 transaction.
    etxn_reserve(1).unwrap();

    let mut hook_acc_id: AccountId = uninit_buf!();
    // Account field from the originating transaction
    let mut otnx_acc_id: AccountId = uninit_buf!();

    match is_txn_outgoing::<GUARD_ID_1>(&mut hook_acc_id, &mut otnx_acc_id) {
        Err(e) => rollback(b"Could not fetch accounts id", e.code() as _),
        Ok(false) => accept(b"Carbon: Incoming transaction", 2),
        Ok(true) => {},
    };

    let drops_to_send = calc_drops();

    let _ = trace_num(b"", drops_to_send as _);

    // Hooks communicate accounts via the 20 byte account ID, this can be generated from an raddr like so
    // A more efficient way to do this is precompute the account-id from the raddr (if the raddr never changes)
    let mut carbon_accid: AccountId = uninit_buf!();
    let ret = util_accid(&mut carbon_accid, CARBON_ADDRESS).unwrap();
    let _ = trace_num(b"", ret as _);

    // Fees for emitted transactions are based on how many txn your hook is emitted, whether or not this triggering
    // was caused by a previously emitted transaction and how large the new emitted transaction is in bytes
    // We need to precompute this before populating the payment transaction, as it is a field inside the tx
    let fee_base = etxn_fee_base(PREPARE_PAYMENT_SIMPLE_SIZE as _).unwrap();


    let mut tx: TxnPaymentSimple = uninit_buf!();
    prepare_payment_simple(&mut tx, drops_to_send, fee_base, &carbon_accid, 0, 0)
        .unwrap();

    let mut emithash: Hash = uninit_buf!();
    emit(&mut emithash, &tx).unwrap();

    accept(b"Carbon: Emitted transaction", 0);
}

/// Calc drops to send
///
/// Always define all functions with `#[inline(always)]`
/// to avoid unknown function calls in wasm binary
#[inline(always)]
fn calc_drops() -> u64 {
    // Fetch the sent Amount.
    // Amounts can be 384 bits or 64 bits. 
    // If the Amount is an XRP value it will be 64 bits.
    let mut amount_buffer: Amount = uninit_buf!();
    let amount_len = otxn_field(&mut amount_buffer, FieldId::Amount).unwrap();

    if amount_len != 8 {
        // You can trace the behaviour of your hook using the `trace` api
        // which will output to xrpld's trace log
        let _ = trace(b"Carbon: Non-xrp transaction detected, sending default 1000 drops to rfCarbon", b"", DataRepr::AsUTF8);

        MIN_DROPS
    } else {
        // Otherwise we send PERCENT%
        let _ = trace(b"Carbon: XRP transaction detected, computing percent to send to rfCarbon", b"", DataRepr::AsUTF8);

        let otxn_drops = amount_to_drops(&amount_buffer).unwrap();

        let _ = trace_num(b"", otxn_drops as _);

        let drops_to_send = (otxn_drops as f64 * (PERCENT as f64 / 100.0)) as _;

        // If its less we send the default amount
        if drops_to_send < MIN_DROPS {
            return MIN_DROPS
        }
       
        drops_to_send
    }
}
