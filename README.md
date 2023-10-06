# A Stellar SDK for Substrate projects

This is an SDK for [Stellar](https://stellar.org) that is suitable to be used in a [Substrate](https://substrate.dev/) project. It does not depend on the standard library but on the crate [`sp-std`](https://crates.io/crates/sp-std), which is Substrate's replacement of Rust's standard library.


## Crate Features

This crate has three features:

- `std`: This feature will enable the standard library. It is enabled by default, therefore this crate needs to be imported using `default-features = false` in a Substrate project.
- `offchain`: This is a collection of features usable in an offchain worker, where http requests are possible. It mainly comprises an abstraction layer over parts of the [Horizon API](https://developers.stellar.org/api/).
- `all-types`: This will give access to all types defined in Stellar, even types that are only required internally for the Stellar Consensus Protocol. Otherwise, this crate will only give access to user-facing types such as `Transaction` or `Operation` (see the section about [Stellar types](#stellar-xdr-types))

## Conversion traits

In order to make the SDK API convenient to use with a variety of input types, we make heavy use of conversion traits. For example the type `Horizon` implements the method `fetch_account` whose first parameter is an account id. This parameter can be provided as a string representing the string encoding of the account id:

```rust
horizon.fetch_account("GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ", 1000)?;
```

Since strings in Substrate are usually represented as `Vec<u8>`, also a vector representation of this string can be used as an argument:

```rust
let vec: Vec<u8> = Vec::from("GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ".as_bytes());
horizon.fetch_account(vec, 1000)?;
```

Also `u8` slices and `u8` arrays are possible argument types:

```rust
let slice: &[u8] = "GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ".as_bytes();
let array: [u8; 56] = slice.try_into()?;
horizon.fetch_account(slice, 1000)?;
horizon.fetch_account(array, 1000)?;
```

It is also possible to provide an `AccountId` struct itself:

```rust
let account_id = AccountId::from_encoding("GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ")?;
horizon.fetch_account(account_id, 1000)?;
```

This crate provides the following conversion traits:

#### `IntoAccountId`

This is the trait for parameters that represent an `AccountId`. It is implemented by

- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: The string encoding of the account id (using Stellar's encoding of account ids)
- `AccountId`: An `AccountId`

#### `IntoAmount`

This is the trait for parameters that represent an asset amount. This is used to cleanly distinguish between stroop amounts and lumen amounts (where one lumen is 10_000_000 stroops). It is implemented by

- `LumenAmount`: A wrapped `f64` representing an amount in lumens
- `StroopAmount`: A wrapped `i64` representing an amount in stroops
- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: This is the decimal string of the amount in _lumens_. The string can have up to 7 decimal places. This representation is usually used in responses of the Horizon API

It is recommended to use either `LumenAmount` or the string representation as they can be converted without loss to Stellar's internal stroop representation of amounts.

#### `IntoClaimbleBalanceId`

This is the trait for parameters that represent an `ClaimableBalanceId`. It is implemented by

- `AsBinary`: this can be either the raw binary value (`AsBinary::Binary`) of the claimable balance id (36 bytes) or its hex encoding as a string (`AsBinary::Hex`)
- `ClaimableBalanceId`: A `ClaimableBalanceId`

#### `IntoDataValue`

This is the trait for parameters that represent an `DataValue`. It is implemented by

- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: The data value string
- `DataValue`: An `DataValue`

#### `IntoHash`

This is the trait for parameters that represent an `Hash`. It is implemented by

- `AsBinary`: this can be either the raw binary value (`AsBinary::Binary`) of the hash (32 bytes) or its hex encoding as a string (`AsBinary::Hex`)
- `Hash`: A `Hash`

#### `IntoMuxedAccountId`

This is the trait for parameters that represent a `MuxedAccount`. Be aware that the type `MuxedAccount` contains simple account ids as well as proper multiplexed account ids. It is implemented by

- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: The string encoding of the muxed account id (using Stellar's encoding of muxed account ids)
- `AccountId`: A simple `AccountId` repesented as a `MuxedAccount`
- `MuxedAccount`: An `MuxedAccount`

#### `IntoPublicKey`

This is the trait for parameters that represent a `PublicKey`. Be aware that the types `AccountId` and `PublicKey` are aliases (they are synonyms in Stellar). The trait `IntoPublicKey` exists for convenience and to make it more clear when an account id is used as a public key. It is implemented by

- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: The string encoding of the public key (using Stellar's encoding of public keys)
- `PublicKey`: A `PublicKey` (which is an alias for `AccountId`)

#### `IntoSecretKey`

This is the trait for parameters that represent a `SecretKey`. It is implemented by

- string-like types implementing `AsRef<[u8]>` such as `&str`, `[u8; N]`, `[u8]`, `Vec<u8>`: The string encoding of the secret key (using Stellar's encoding of secret keys)
- `SecretKey`: A `SecretKey`

#### `IntoTimePoint`

This is the trait for parameters that represent a point in time used by transaction time bounds. It is implemented by

- `MilliSecondEpochTime`: a unix timestamp represented in milliseconds
- `SecondEpochTime`: a unix timestamp represented in seconds
- `()`: for an unlimited time bound

## Create, Sign and Submit Transactions

This is one of the main workflows when using this SDK. Note that these features only become available when using the crate feature `offchain`. It usually consists of the following steps:

1. Create an instance of the `Horizon` type and provide the base url of the horizon server.
2. Fetch the next sequence number of the transaction's source account from the horizon server. The next sequence number is one higher than the current sequence number of the account and must be used as the transaction sequence number in order to be valid.
3. Build the transaction and provide
   - The source account id
   - The fetched sequence number
   - The fees per operation
   - The timebounds (optional)
   - The memo (optional)
4. Append operations to the transaction. The transaction needs to be mutable. Each operation has constructor methods such as `Operation::new_payment` (for a payment operation) or `Operation::new_bump_sequence` (for a bump sequence number operation). The constructors make use of the [conversion traits](#conversion-traits). Note that one needs to chain the method call `set_source_account` if one wants to set a dedicated source account for an operation.
5. Convert the transaction into a transaction envelope. This will create an envelope without signatures. Note that this operation mutates the fee of the transaction and multiplies the fee per operation (provided when building the transaction) with the number of appended operations. The transaction in the transaction envelope should not be mutated anymore after this step, otherwise the signatures added subsequently will not be valid anymore.
6. Sign the transaction envelope with the required secret keys. Signatures are only valid for certain Stellar networks passphrases (so that a transaction can be signed either for the public network or the test network). For that reason the method for signing a transaction requires a netwok passphrase.
7. Submit the signed envelope to the Stellar network using the Horizon API.

The following code shows a complete example.

```rust
use substrate_stellar_sdk::{
    horizon::Horizon, network::TEST_NETWORK, Asset, IntoSecretKey, Memo, MilliSecondEpochTime,
    MuxedAccount, Operation, Price, PublicKey as StellarPublicKey, SecondEpochTime, StroopAmount,
    TimeBounds, Transaction, TransactionEnvelope, XdrCodec,
};

// ...
// in some function:

const ACCOUNT_ID1: &str = "GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ";
const ACCOUNT_ID2: &str = "GBNKQVTFRP25TIQRODMU5GJGSXDKHCEUDN7LNMOS5PNM427LMR77NV4M";
const ACCOUNT_ID3: &str = "GCACWDM2VEYTXGUI3CUYLBJ453IBEPQ3XEJKA772ARAP5XDQ4NMGFZGJ";

const SIGNER1: &str = "SCVKZEONBSU3XD6OTHXGAP6BTEWHOU4RPZQZJJ5AVAGPXUZ5A4D7MU6S";
const SIGNER3: &str = "SDOKV37I4TI655LMEMDQFOWESJ3LK6DDFKIVTYKN4YYTSAYFIBPP7MYI";

// Step 1: instantiate horizon server
let horizon = Horizon::new("https://horizon-testnet.stellar.org");

// Step 2: fetch next sequence number of account
let next_sequence_number = horizon.fetch_next_sequence_number(ACCOUNT_ID1, 10_000)?;

debug::info!("Sequence number: {}", next_sequence_number);

// Step 3: build transaction
let mut transaction = Transaction::new(
    ACCOUNT_ID1,
    next_sequence_number,
    Some(321),
    Some(TimeBounds::from_time_points(
        SecondEpochTime(162620000),
        MilliSecondEpochTime(1_667_257_200_000),
    )),
    Some(Memo::from_text_memo("Hello World!")?),
)?;

// Step 4: add operations
transaction.append_operation(
    Operation::new_payment(
        ACCOUNT_ID2,
        Asset::from_asset_code("USD", ACCOUNT_ID3)?,
        StroopAmount(1_234_560_000),
    )?
    .set_source_account(ACCOUNT_ID3)?,
)?;

transaction.append_operation(Operation::new_manage_sell_offer(
    Asset::from_asset_code("DOMINATION", ACCOUNT_ID2)?,
    Asset::native(),
    "152.103",
    Price::from_float(4.58)?,
    Some(1742764),
)?)?;

// Step 5: transform into transaction envelope and sign
let mut envelope = transaction.into_transaction_envelope();

// Step 6: sign transaction envelope
envelope.sign(
    &TEST_NETWORK,
    vec![&SIGNER1.into_secret_key()?, &SIGNER3.into_secret_key()?],
)?;

// Step 7: submit transaction
let submission_response = horizon.submit_transaction(&envelope, 60_000, true);
debug::info!("Response: {:?}", submission_response);
```

## Stellar XDR Types

Stellar defines a bunch of [data types](https://github.com/stellar/stellar-core/tree/master/src/xdr) on a protocol level. These types are serialized and deserialized using the [XDR standard](https://datatracker.ietf.org/doc/html/rfc4506.html).

This crates contains all these Stellar defined types (in `substrate_stellar_sdk::types`) including a complete XDR encoder and decoder for these types. For that purpose every type implements the trait `XdrCodec`, which provides the following methods:

- `fn to_xdr(&self) -> Vec<u8>`: encode as binary XDR
- `fn to_base64_xdr(&self) -> Vec<u8>`: encode as XDR, afterwards encode result as base64
- `from_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError>`: decode binary XDR
- `from_base64_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError>`: decode as base64, then decode result as XDR

### Autogenerator

The types and the XDR decoder are automatically generated via the tool in `/autogenerator`. This generator will download the latest Stellar types from the Stellar Core GitHub repository and will generate the types and XDR decoder.

### Crate Feature `all-types`

By default not all Stellar types will be generated. Missing types are special types that are only used internally for the consensus mechanism. In order to generate all types, use the feature flag `all-types`.
