# A Substrate compatible SDK for Stellar

## Main workflow: create, sign and submit transaction

```rust
use substrate_stellar_sdk::{
    horizon::Horizon, network::TEST_NETWORK, Asset, IntoSecretKey, Memo, MilliSecondEpochTime,
    MuxedAccount, Operation, Price, PublicKey as StellarPublicKey, SecondEpochTime, StroopAmount,
    TimeBounds, Transaction, TransactionEnvelope, XdrCodec,
};


const ACCOUNT_ID1: &str = "GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ";
const ACCOUNT_ID2: &str = "GBNKQVTFRP25TIQRODMU5GJGSXDKHCEUDN7LNMOS5PNM427LMR77NV4M";
const ACCOUNT_ID3: &str = "GCACWDM2VEYTXGUI3CUYLBJ453IBEPQ3XEJKA772ARAP5XDQ4NMGFZGJ";

const SIGNER1: &str = "SCVKZEONBSU3XD6OTHXGAP6BTEWHOU4RPZQZJJ5AVAGPXUZ5A4D7MU6S";
const SIGNER3: &str = "SDOKV37I4TI655LMEMDQFOWESJ3LK6DDFKIVTYKN4YYTSAYFIBPP7MYI";

// Step 1: instantiate horizon server
let horizon = Horizon::new("https://horizon-testnet.stellar.org");

// Step 2: fetch next sequence number of account
let next_sequence_number = horizon
    .fetch_next_sequence_number(ACCOUNT_ID1, 10_000)
    .unwrap();

debug::info!("Sequence number: {}", next_sequence_number);

// Step 3: build transaction
let mut transaction = Transaction::new(
    ACCOUNT_ID1,
    next_sequence_number,
    Some(2 * 321),
    Some(TimeBounds::from_time_points(
        SecondEpochTime(162620000),
        MilliSecondEpochTime(1_667_257_200_000),
    )),
    Some(Memo::from_text_memo("Hello World!").unwrap()),
)
.unwrap();

// Step 4: add operations
transaction
    .append_operation(
        Operation::new_payment(
            Some(ACCOUNT_ID3),
            ACCOUNT_ID2,
            Asset::from_asset_code("USD", ACCOUNT_ID3).unwrap(),
            StroopAmount(1_234_560_000),
        )
        .unwrap(),
    )
    .unwrap();

transaction
    .append_operation(
        Operation::new_manage_sell_offer(
            None::<&str>,
            Asset::from_asset_code("DOMINATION", ACCOUNT_ID2).unwrap(),
            Asset::native(),
            "152.103",
            Price::from_float(4.58).unwrap(),
            Some(1742764),
        )
        .unwrap(),
    )
    .unwrap();

// Step 5: transform into transaction envelope and sign
let mut envelope = transaction.into_transaction_envelope();
envelope
    .sign(
        &TEST_NETWORK,
        vec![
            &SIGNER1.into_secret_key().unwrap(),
            &SIGNER3.into_secret_key().unwrap(),
        ],
    )
    .unwrap();

// Step 6: submit transaction
let submission_response = horizon.submit_transaction(&envelope, 60_000, true);
debug::info!("Response: {:?}", submission_response);
```
