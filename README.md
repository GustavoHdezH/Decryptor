<p align="center">
  <img  src="docs/Decryptor.png">
</p>
<div style="text-align: center;">
    <span style="display:inline-block">
        <img src="https://img.shields.io/badge/verified-FCC624?logo=linux&logoColor=white&logoWidth=30" alt="Linux Badge">
    </span>
</div>

---
Decryptor is a command-line tool that simplifies the process of decrypting files. It is designed to specifically handle
popular encryption formats such as .gpg and .age, providing a fast and efficient solution for developers and users who 
need to access their information securely.

This project allows you to decrypt files encrypted with GNU Privacy Guard (GPG) and Age and works with a pair of keys:
* __Public key__:
  * Can be shared with anyone. It is used to encrypt messages or files that you want to send to the owner.
* __Private key__:
  * Must be kept secret and is only known to its owner. It is used to decrypt messages or files that have been
    encrypted with the corresponding public key.

## Pre-requirements for Linux

---
Create ssh key segure
* [Rust](https://www.rust-lang.org/tools/install)
* [GPG](https://gnupg.org/download/)
* [Age](https://github.com/FiloSottile/age)

## Setup
This tool is compatible with:
* RSA
* Ed25519

### RSA Setup

```bash
  ssh-keygen -t rsa -b 4086 -C "name_identifier"
```

### Ed25519 Setup

```bash
  ssh-keygen -t ed25519 -a 100 -C "name_identifier"
```

## Usage

1. To compile the project, execute the following command:
```bash
    cargo build --release
```

2. If dependencies need to be cleaned for a clean execution, the following execution must be performed:

```bash
  cargo clean
```

3. Project execution: the following instruction must be executed:

```bash
  cargo run
```

## Resources

* Documentation:
  * [Set up variables][env]
  * [Set up GPG][gpg]

--------

<div align="center"> 
  <p>
    Made &#x2692;&#xfe0f; with &hearts; by Gussy &#x2693;  |  CXC 2020-2025
  </p>
</div>

<!-- Inicio de enlaces de este documento -->
[env]: docs/env_configuration.md
[gpg]: docs/gpg_configuration.md
<!-- Fin de enlaces de este documento -->