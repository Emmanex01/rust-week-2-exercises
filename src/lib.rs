use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    if hex_str.len() % 2 != 0 {
        return Err("Hex string must have an even length".to_string());
    }

    decode(hex_str).map_err(|e| e.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>

   let mut reversed = bytes.to_vec(); // Copy the slice into a Vec<u8>
    reversed.reverse();                // Reverse the Vec in-place
    reversed 
    
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    num.to_le_bytes()

}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    input.trim().parse::<u64>().map_err(|_| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    
    if script.starts_with(&[0x76, 0xa9, 0x14]) {
        return ScriptType::P2PKH;
    }

    // Check for the P2WPKH prefix: OP_0 PUSH_20
    // This is represented by the bytes [0x00, 0x14]
    if script.starts_with(&[0x00, 0x14]) {
        return ScriptType::P2WPKH;
    }

    // If no known prefix is found, classify as Unknown
    ScriptType::Unknown
}

// // TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
// TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    let len = script[1] as usize;

    if script.len() >= 2 + len {
       return &script[2..len+2];
    } else {
        return &[];
    }
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    *balance -= fee;
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
}
    


// TODO: Add necessary derive traits

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant

        match byte {
            0x76 => Ok(Opcode::OpDup),
            0xac => Ok(Opcode::OpChecksig),
             _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    
    }

    }


// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    
    println!(
        "Consuming UTXO: txid = {:x?}, vout = {}, value = {} sats",
        utxo.txid, utxo.vout, utxo.value
    );

    utxo
}


