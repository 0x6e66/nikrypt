# nikrypt

This repository seeks to be a cryptography library for learning purposes. First and foremost it is an exercise for me to get more familiar with cryptography. Although I try my best to ensure that the implementations are correct, I am not trying to make a side-channel resistant implementation. So please, for god's sake do **not** use this a a crypto-library in any production environment.

## List of implemented algorithms
- Cryptography
    - [ChaCha20](src/crypto/chacha/)
    - [AES](src/crypto/aes/)
    - [SHA2](src/crypto/sha2)
        - SHA-224
        - SHA-256
        - SHA-384
        - SHA-512
        - SHA-512/t
    - [HMAC](src/crypto/hmac.rs)
        - HMAC-SHA-224
        - HMAC-SHA-256
        - HMAC-SHA-384
        - HMAC-SHA-512
    - [PBKDF](src/crypto/pbkdf.rs)
        - PBKDF2-HMAC-SHA-224
        - PBKDF2-HMAC-SHA-256
        - PBKDF2-HMAC-SHA-384
        - PBKDF2-HMAC-SHA-512
