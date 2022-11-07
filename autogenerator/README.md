# Generator of Stellar XDR type Encoder/Decoder

The code in this directory generates a Rust decoder and encoder of all XDR types used in Stellar. It generates the code in the file `src/xdr/types.rs`

The code in this folder is derived from https://github.com/stellar/js-xdr 

# How to use

## Requirements

NodeJs, NPM, Docker, Cargo (for formatting)

## Step 1: Generate Rust code

This downloads the latest version of Stellar XDR type specification and generates
according Rust code

```
npm install
npm run build
```

### How it works

- downloads the Stellar XDR type specification from GitHub repository `stellar/stellar-core`
- uses the [Stellar's own parser](https://github.com/stellar/xdrgen.git) to generate a JavaScript version of the XDR type specification
  - this runs in a Docker container, a local Ruby installation is not required
  - this code is in the folder `x2JavaScript`
- executes the generated JavaScript code to generate the Rust code
  - this code is in the folder `js-xdr`
- copy static Rust files to the generated Rust code to complete the crate
  - the static Rust files are in the folder `static`

# Assumptions

- this code assumes that the only way cycles in types can occur is if an enum or struct type directly references itself (no indirect cycles)
  - otherwise we need to have `Box` way too often and types get cluttered
  - this assumption is currently true as the only cycle occurs in the type `ClaimPredicate` – and that cycle is direct
  - this assumption might not hold true in the future anymore
    - in that case start to introduce `Box` everytime an enum or struct references a non-primitive type that is not wrapped into a `Vec`
