# Advanced Encryption Standard (AES)

Sources:
- [FIPS-197](https://csrc.nist.gov/files/pubs/fips/197/final/docs/fips-197.pdf)

## Tests
Run all tests with:
```bash
cargo test
```

The test cases are from the following pdf: https://www.kavaliro.com/wp-content/uploads/2014/03/AES.pdf

The is a typo in the pdf of page 3 in the round key number 6:

```
Wrong key in pdf: Round 6: BD 3D C2 B7 B8 7C 47 15 6A 6C 95 27 AC 2E 0E 4E
Correct key:               BD 3D C2 87 B8 7C 47 15 6A 6C 95 27 AC 2E 0E 4E
                                    ^^ -- 8 instead of B
```