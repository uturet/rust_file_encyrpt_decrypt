extern crate crypto;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use crypto::aes::{self, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};

const KEY: &[u8; 32] = b"0123456789abcdef0123456789abcdef";
const IV: &[u8; 16] = b"0123456789abcdef";

// Function to encrypt a file
fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), io::Error> {
    let mut input_file = File::open(input_path)?; // Open input file
    let mut output_file = File::create(output_path)?; // Create output file

    // Create encryptor instance using AES CBC mode
    let mut encryptor = aes::cbc_encryptor(KeySize::KeySize256, KEY, IV, NoPadding);

    // Buffers for reading and writing data
    let mut read_buffer = [0; 4096];
    let mut write_buffer = [0; 4096];

    // Encrypt the file data
    loop {
        let bytes_read = input_file.read(&mut read_buffer)?; // Read data from input file
        if bytes_read == 0 {
            break; // If no more data to read, break the loop
        }
        let read_buffer = &read_buffer[..bytes_read]; // Slice read data
        let mut read_buffer_ref = RefReadBuffer::new(read_buffer); // Create reference read buffer

        let mut write_buffer_ref = RefWriteBuffer::new(&mut write_buffer); // Create reference write buffer

        encryptor.encrypt(&mut read_buffer_ref, &mut write_buffer_ref, true); // Encrypt data

        // Write encrypted data to output file
        output_file.write_all(write_buffer_ref.take_read_buffer().take_remaining())?;
    }

    Ok(()) // Return Ok if encryption is successful
}

// Function to decrypt a file
fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), io::Error> {
    let mut input_file = File::open(input_path)?; // Open input file
    let mut output_file = File::create(output_path)?; // Create output file

    // Create decryptor instance using AES CBC mode
    let mut decryptor = aes::cbc_decryptor(KeySize::KeySize256, KEY, IV, NoPadding);

    // Buffers for reading and writing data
    let mut read_buffer = [0; 4096];
    let mut write_buffer = [0; 4096];

    // Decrypt the file data
    loop {
        let bytes_read = input_file.read(&mut read_buffer)?; // Read data from input file
        if bytes_read == 0 {
            break; // If no more data to read, break the loop
        }
        let read_buffer = &read_buffer[..bytes_read]; // Slice read data
        let mut read_buffer_ref = RefReadBuffer::new(read_buffer); // Create reference read buffer

        let mut write_buffer_ref = RefWriteBuffer::new(&mut write_buffer); // Create reference write buffer

        decryptor.decrypt(&mut read_buffer_ref, &mut write_buffer_ref, true); // Decrypt data

        // Write decrypted data to output file
        output_file.write_all(write_buffer_ref.take_read_buffer().take_remaining())?;
    }

    Ok(()) // Return Ok if decryption is successful
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments
    if args.len() != 4 {
        // Check if correct number of arguments provided
        println!(
            "Usage: {} <input_file> <output_encrypted_file> <output_decrypted_file>",
            args[0]
        );
        return;
    }

    let input_file = &args[1]; // Get input file path
    let encrypted_file = &args[2]; // Get output encrypted file path
    let decrypted_file = &args[3]; // Get output decrypted file path

    // Encrypt the input file
    match encrypt_file(input_file, encrypted_file) {
        Ok(_) => println!("Encryption successful."), // Print success message
        Err(e) => eprintln!("Encryption failed: {}", e), // Print error message
    }

    // Decrypt the encrypted file
    match decrypt_file(encrypted_file, decrypted_file) {
        Ok(_) => println!("Decryption successful."), // Print success message
        Err(e) => eprintln!("Decryption failed: {}", e), // Print error message
    }
}
