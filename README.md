# nikrypt

This repository seeks to be a cryptography library for learning purposes. First and foremost it is an exercise for me to get more familiar with cryptography. Although I try my best to ensure that the implementations are correct, I am not trying to make a side-channel resistant implementation. So please, for god's sake do **not** use this a a crypto-library in any production environment.

## List of implemented algorithms
- Cryptography
    - [ChaCha20](src/crypto/chacha/)
    - [AES](src/crypto/aes/)
    - [SHA-256](src/hash/sha2)
    - [HMAC](src/crypto/hmac.rs)
    - [PBKDF2](src/crypto/pbkdf.rs)
