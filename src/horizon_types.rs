//! Type definition of horiz balance: (), limit: (), buying_liabilities: (), selling_liabilities: (), sponsor: (), last_modified_ledger: (), is_a, sponsor: (), last_modified_ledger: (), is_authorized: (), is_authorized_to_maintain_liabilities: (), is_clawback_enabled: (), asset_code: (), asset_issuer: () uthorized: (), is_authorized_to_maintain_liabilities: (), is_clawback_enabled: (), asset_type: (), asset_code: (), asset_issuer: ()  balance: (), limit: (), buying_liabilities: (), selling_liabilities: (), sponsor: (), last_modified_ledger: (), is_authorized: (), is_authorized_to_maintain_liabilities: (), is_clawback_enabled: (), asset_type: (), asset_code: (), asset_issuer: () on API responses
//!
//! Taken from [The horizon protocol specification](https://github.com/stellar/go/blob/master/protocols/horizon/main.go)

use serde::Deserialize;
use serde_json::Value;
use sp_std::vec::Vec;

use crate::String;

/// The type of a Hypertext Application Language (HAL) link
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct HalLink {
    pub href: String,
    pub templated: Option<bool>,
}

/// The `AccountResponseLinks` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountResponseLinks {
    #[serde(rename = "self")]
    pub _self: HalLink,
    pub transactions: HalLink,
    pub operations: HalLink,
    pub payments: HalLink,
    pub effects: HalLink,
    pub offers: HalLink,
    pub trades: HalLink,
    pub data: HalLink,
}

/// The `AccountThresholds` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountThresholds {
    pub low_threshold: u8,
    pub med_threshold: u8,
    pub high_threshold: u8,
}

/// The `AccountFlags` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountFlags {
    pub auth_required: bool,
    pub auth_revocable: bool,
    pub auth_immutable: bool,
    pub auth_clawback_enabled: bool,
}

/// The `Balance` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Balance {
    pub balance: String,
    pub limit: Option<String>,
    pub buying_liabilities: String,
    pub selling_liabilities: String,
    pub sponsor: Option<String>,
    pub last_modified_ledger: Option<u32>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    pub is_clawback_enabled: Option<bool>,
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

/// The `Signer` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Signer {
    pub weight: i32,
    pub key: String,
    #[serde(rename = "type")]
    pub signer_type: String,
    pub sponsor: Option<String>,
}

/// The `Account` type
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AccountResponse {
    pub _links: AccountResponseLinks,
    pub id: String,
    pub account_id: String,
    pub sequence: String,
    pub subentry_count: i32,
    pub inflation_destination: Option<String>,
    pub home_domain: Option<String>,
    pub last_modified_ledger: u32,
    pub last_modified_time: String,
    pub thresholds: AccountThresholds,
    pub flags: AccountFlags,
    pub balances: Vec<Balance>,
    pub signers: Vec<Signer>,
    pub data: Value,
    pub num_sponsoring: u32,
    pub num_sponsored: u32,
    pub sponsor: Option<String>,
    pub paging_token: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_account_response() {
        let account_response = b"{\"_links\":{\"self\":{\"href\":\"https://horiz\
          on-testnet.stellar.org/accounts/GCC65Q6A\
          C3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY\"},\"transactions\":{\"href\":\"https\
          ://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32\
          ZED25NITJI67JEBY/transactions{?cursor,limit,order}\",\"templated\":true},\"operation\
          s\":{\"href\":\"https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RG\
          EUXWJJELFQG35HH32ZED25NITJI67JEBY/operations{?cursor,limit,order}\",\"templated\":t\
          rue},\"payments\":{\"href\":\"https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3\
          PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/payments{?cursor,limit,order}\",\"t\
          emplated\":true},\"effects\":{\"href\":\"https://horizon-testnet.stellar.org/accounts/\
          GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/effects{?cursor,limit,o\
          rder}\",\"templated\":true},\"offers\":{\"href\":\"https://horizon-testnet.stellar.org/a\
          ccounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/offers{?cursor,\
          limit,order}\",\"templated\":true},\"trades\":{\"href\":\"https://horizon-testnet.stella\
          r.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/trades{?\
          cursor,limit,order}\",\"templated\":true},\"data\":{\"href\":\"https://horizon-testnet.s\
          tellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/dat\
          a/{key}\",\"templated\":true}},\"id\":\"GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25N\
          ITJI67JEBY\",\"account_id\":\"GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JE\
          BY\",\"sequence\":\"1464747056693248\",\"subentry_count\":0,\"last_modified_ledger\":3410\
          38,\"last_modified_time\":\"2021-07-07T03:03:39Z\",\"thresholds\":{\"low_threshold\":0,\"\
          med_threshold\":0,\"high_threshold\":0},\"flags\":{\"auth_required\":false,\"auth_revoca\
          ble\":false,\"auth_immutable\":false,\"auth_clawback_enabled\":false},\"balances\":[{\"b\
          alance\":\"10000.0000000\",\"buying_liabilities\":\"0.0000000\",\"selling_liabilities\":\"\
          0.0000000\",\"asset_type\":\"native\"}],\"signers\":[{\"weight\":1,\"key\":\"GCC65Q6AC3PXMEU\
          B4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY\",\"type\":\"ed25519_public_key\"}],\"data\":\
          {},\"num_sponsoring\":0,\"num_sponsored\":0,\"paging_token\":\"GCC65Q6AC3PXMEUB4L7O3RGE\
          UXWJJELFQG35HH32ZED25NITJI67JEBY\"}";
        let account_response: AccountResponse = serde_json::from_slice(account_response).unwrap();

        let expected_hal_links = AccountResponseLinks {
          _self: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY"),
                  templated: None
              },
              transactions: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/transactions{?cursor,limit,order}"),
                  templated: Some(true)
              },
              operations: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/operations{?cursor,limit,order}"),
                  templated: Some(true)
              },
              payments: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/payments{?cursor,limit,order}"),
                  templated: Some(true)
              },
              effects: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/effects{?cursor,limit,order}"),
                  templated: Some(true)
              },
              offers: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/offers{?cursor,limit,order}"),
                  templated: Some(true)
              },
              trades: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/trades{?cursor,limit,order}"),
                  templated: Some(true)
              },
              data: HalLink {
                  href: String::from("https://horizon-testnet.stellar.org/accounts/GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY/data/{key}"),
                  templated: Some(true)
              },
          };

        let expexted_response = AccountResponse {
            _links: expected_hal_links,
            id: String::from("GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY"),
            account_id: String::from("GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY"),
            sequence: String::from("1464747056693248"),
            subentry_count: 0,
            inflation_destination: None,
            home_domain: None,
            last_modified_ledger: 341038,
            last_modified_time: String::from("2021-07-07T03:03:39Z"),
            thresholds: AccountThresholds {
                low_threshold: 0,
                med_threshold: 0,
                high_threshold: 0,
            },
            flags: AccountFlags {
                auth_required: false,
                auth_revocable: false,
                auth_immutable: false,
                auth_clawback_enabled: false,
            },
            balances: vec![Balance {
                balance: String::from("10000.0000000"),
                limit: None,
                buying_liabilities: String::from("0.0000000"),
                selling_liabilities: String::from("0.0000000"),
                sponsor: None,
                last_modified_ledger: None,
                is_authorized: None,
                is_authorized_to_maintain_liabilities: None,
                is_clawback_enabled: None,
                asset_type: String::from("native"),
                asset_code: None,
                asset_issuer: None,
            }],
            signers: vec![Signer {
                key: String::from("GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY"),
                signer_type: String::from("ed25519_public_key"),
                sponsor: None,
                weight: 1,
            }],
            data: Value::Object(serde_json::map::Map::new()),
            num_sponsoring: 0,
            num_sponsored: 0,
            sponsor: None,
            paging_token: String::from("GCC65Q6AC3PXMEUB4L7O3RGEUXWJJELFQG35HH32ZED25NITJI67JEBY"),
        };

        assert_eq!(expexted_response, account_response);
    }

    #[test]
    fn parse_centre_account_response() {
        let account_response = b"{\"_links\":{\"self\":{\"href\":\"https://hori\
          zon.stellar.org/accounts/GA5ZSEJYB37JRC5A\
          VCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\"},\"transactions\":{\"href\":\"https://horiz\
          on.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\
          /transactions{?cursor,limit,order}\",\"templated\":true},\"operations\":{\"href\":\"http\
          s://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5R\
          E34K4KZVN/operations{?cursor,limit,order}\",\"templated\":true},\"payments\":{\"href\":\
          \"https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJ\
          APP5RE34K4KZVN/payments{?cursor,limit,order}\",\"templated\":true},\"effects\":{\"href\
          \":\"https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IH\
          OJAPP5RE34K4KZVN/effects{?cursor,limit,order}\",\"templated\":true},\"offers\":{\"href\
          \":\"https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IH\
          OJAPP5RE34K4KZVN/offers{?cursor,limit,order}\",\"templated\":true},\"trades\":{\"href\"\
          :\"https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHO\
          JAPP5RE34K4KZVN/trades{?cursor,limit,order}\",\"templated\":true},\"data\":{\"href\":\"h\
          ttps://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAP\
          P5RE34K4KZVN/data/{key}\",\"templated\":true}},\"id\":\"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM3\
          35X2KGX3IHOJAPP5RE34K4KZVN\",\"account_id\":\"GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3\
          IHOJAPP5RE34K4KZVN\",\"sequence\":\"144373126631784451\",\"subentry_count\":5,\"home_dom\
          ain\":\"centre.io\",\"last_modified_ledger\":35792496,\"last_modified_time\":\"2021-06-0\
          8T05:40:32Z\",\"thresholds\":{\"low_threshold\":2,\"med_threshold\":2,\"high_threshold\":\
          2},\"flags\":{\"auth_required\":false,\"auth_revocable\":false,\"auth_immutable\":false,\
          \"auth_clawback_enabled\":false},\"balances\":[{\"balance\":\"314699333.9300000\",\"limit\
          \":\"922337203685.4775807\",\"buying_liabilities\":\"922022504351.5475807\",\"selling_li\
          abilities\":\"0.0000000\",\"last_modified_ledger\":36266128,\"is_authorized\":true,\"is_\
          authorized_to_maintain_liabilities\":true,\"asset_type\":\"credit_alphanum12\",\"asset\
          _code\":\"USDCAllow\",\"asset_issuer\":\"GDIEKKIQWMIZ4LD3RP3ABPN7X5KEAEWYMR634BRHB7EUL\
          IMEVREWLF3G\"},{\"balance\":\"1434.4215940\",\"buying_liabilities\":\"0.0000000\",\"sellin\
          g_liabilities\":\"0.0000000\",\"asset_type\":\"native\"}],\"signers\":[{\"weight\":1,\"key\":\
          \"GAPIIRF3JXQAY63NSD6ALZ6JVUYDXCQIQIH5MZA3XYMR7LADTFKRBXZB\",\"type\":\"ed25519_publi\
          c_key\"},{\"weight\":1,\"key\":\"GBBPS4JXB72222FPAHBGPUH3M6XIPG3CFDLIWDVPTAXJFH4WLYL46\
          HQW\",\"type\":\"ed25519_public_key\"},{\"weight\":1,\"key\":\"GCHQ3ISXNIIKPQDSOROE4GBA63X\
          ZXTMI65C3HTKALFW6VY4KT3NTCELV\",\"type\":\"ed25519_public_key\"},{\"weight\":0,\"key\":\"G\
          A5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\",\"type\":\"ed25519_public_\
          key\"}],\"data\":{},\"num_sponsoring\":0,\"num_sponsored\":0,\"paging_token\":\"GA5ZSEJYB3\
          7JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\"}";
        let account_response: AccountResponse = serde_json::from_slice(account_response).unwrap();

        let expected_hal_links = AccountResponseLinks {
        _self: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
                templated: None
            },
            transactions: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/transactions{?cursor,limit,order}"),
                templated: Some(true)
            },
            operations: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/operations{?cursor,limit,order}"),
                templated: Some(true)
            },
            payments: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/payments{?cursor,limit,order}"),
                templated: Some(true)
            },
            effects: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/effects{?cursor,limit,order}"),
                templated: Some(true)
            },
            offers: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/offers{?cursor,limit,order}"),
                templated: Some(true)
            },
            trades: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/trades{?cursor,limit,order}"),
                templated: Some(true)
            },
            data: HalLink {
                href: String::from("https://horizon.stellar.org/accounts/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN/data/{key}"),
                templated: Some(true)
            },
        };

        let expexted_response = AccountResponse {
            _links: expected_hal_links,
            id: String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            account_id: String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            sequence: String::from("144373126631784451"),
            subentry_count: 5,
            inflation_destination: None,
            home_domain: Some(String::from("centre.io")),
            last_modified_ledger: 35792496,
            last_modified_time: String::from("2021-06-08T05:40:32Z"),
            thresholds: AccountThresholds {
                low_threshold: 2,
                med_threshold: 2,
                high_threshold: 2,
            },
            flags: AccountFlags {
                auth_required: false,
                auth_revocable: false,
                auth_immutable: false,
                auth_clawback_enabled: false,
            },
            balances: vec![
                Balance {
                    balance: String::from("314699333.9300000"),
                    limit: Some(String::from("922337203685.4775807")),
                    buying_liabilities: String::from("922022504351.5475807"),
                    selling_liabilities: String::from("0.0000000"),
                    sponsor: None,
                    last_modified_ledger: Some(36266128),
                    is_authorized: Some(true),
                    is_authorized_to_maintain_liabilities: Some(true),
                    is_clawback_enabled: None,
                    asset_type: String::from("credit_alphanum12"),
                    asset_code: Some(String::from("USDCAllow")),
                    asset_issuer: Some(String::from(
                        "GDIEKKIQWMIZ4LD3RP3ABPN7X5KEAEWYMR634BRHB7EULIMEVREWLF3G",
                    )),
                },
                Balance {
                    balance: String::from("1434.4215940"),
                    limit: None,
                    buying_liabilities: String::from("0.0000000"),
                    selling_liabilities: String::from("0.0000000"),
                    sponsor: None,
                    last_modified_ledger: None,
                    is_authorized: None,
                    is_authorized_to_maintain_liabilities: None,
                    is_clawback_enabled: None,
                    asset_type: String::from("native"),
                    asset_code: None,
                    asset_issuer: None,
                },
            ],
            signers: vec![
                Signer {
                    key: String::from("GAPIIRF3JXQAY63NSD6ALZ6JVUYDXCQIQIH5MZA3XYMR7LADTFKRBXZB"),
                    signer_type: String::from("ed25519_public_key"),
                    sponsor: None,
                    weight: 1,
                },
                Signer {
                    key: String::from("GBBPS4JXB72222FPAHBGPUH3M6XIPG3CFDLIWDVPTAXJFH4WLYL46HQW"),
                    signer_type: String::from("ed25519_public_key"),
                    sponsor: None,
                    weight: 1,
                },
                Signer {
                    key: String::from("GCHQ3ISXNIIKPQDSOROE4GBA63XZXTMI65C3HTKALFW6VY4KT3NTCELV"),
                    signer_type: String::from("ed25519_public_key"),
                    sponsor: None,
                    weight: 1,
                },
                Signer {
                    key: String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
                    signer_type: String::from("ed25519_public_key"),
                    sponsor: None,
                    weight: 0,
                },
            ],
            data: Value::Object(serde_json::map::Map::new()),
            num_sponsoring: 0,
            num_sponsored: 0,
            sponsor: None,
            paging_token: String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        };

        assert_eq!(expexted_response, account_response);
    }
}
