# Overview

This software showcases how to perform file encryption and decryption operations using Rust programming language. It utilizes the crypto crate for cryptographic operations and follows the AES algorithm in CBC mode for encryption and decryption. The program takes command-line arguments specifying the input file, the output file for encrypted content, and the output file for decrypted content. After encrypting the input file, it decrypts the encrypted content and writes the decrypted data to another file.

[Software Demo Video](https://youtu.be/ZWNfvoN23X0)

# Development Environment

```
    Rust
        crypto

    cargo run -- input.txt encrypted.txt decrypted.txt
```

# Useful Websites

- [Rust](https://www.rust-lang.org/)

# Future Work

- Support for Different Cipher Modes
- Graphical User Interface (GUI)
- Integration with Cloud Storage Services
