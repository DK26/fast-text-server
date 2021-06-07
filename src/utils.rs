
use encoding::{
    DecoderTrap, 
    Encoding, 
    all
};

pub type UTF8String = String;

pub fn reverse_str(src: &str) -> String {
    src.chars().rev().collect()
}

pub fn to_utf8_lossy(src: &[u8]) -> UTF8String {
    String::from_utf8_lossy(src).to_string()
}

pub trait Reverse {
    fn reverse(&self) -> String;
}


pub trait DecodeUTF8 {
    fn decode(&self, encoding: &str, trap: DecoderTrap) -> UTF8String;
}


pub trait AsUTF8 {
    fn as_utf8(&self) -> UTF8String;
}

impl AsUTF8 for &[u8] {
    fn as_utf8(&self) -> UTF8String {
        to_utf8_lossy(self)
    }
}

impl Reverse for str {
    fn reverse(&self) -> String {
        reverse_str(&self)
    }
}

impl DecodeUTF8 for &[u8] {
    fn decode(&self, encoding: &str, trap: DecoderTrap) -> UTF8String {
        decode_bytes(self, encoding, trap)
    }
}


pub fn decode_bytes(src: &[u8], encoding: &str, trap: DecoderTrap) -> UTF8String { 

    let encoding = String::from(encoding).trim().to_lowercase();

    let mut src_decoded = String::new();

    let result: String = match encoding.as_str() {
        "utf8" | "utf-8" => { src.as_utf8() },
        "iso-8859-1" => {
            all::ISO_8859_1.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-2" => {
            all::ISO_8859_2.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded    
        },
        "iso-8859-3" => {
            all::ISO_8859_3.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-4" => {
            all::ISO_8859_4.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-5" => {
            all::ISO_8859_5.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-6" => {
            all::ISO_8859_6.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
         "iso-8859-7" => {
            all::ISO_8859_7.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
         "iso-8859-8" => { 
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
         "iso-8859-8-i" => {
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-10" => {
            all::ISO_8859_10.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-13" => {
            all::ISO_8859_13.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-14" => {
            all::ISO_8859_14.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-15" => {
            all::ISO_8859_15.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-8859-16" => {
            all::ISO_8859_16.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "ibm866" => {
            all::IBM866.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "koi8-r" => {
            all::KOI8_R.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "koi8-u" => {
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "macintosh" | "mac-roman" => {
            all::MAC_ROMAN.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "windows-874" => {
            all::WINDOWS_874.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1250" => {
            all::WINDOWS_1250.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1251" => {
            all::WINDOWS_1251.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1252" => {
            all::WINDOWS_1252.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1253" => {
            all::WINDOWS_1253.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1254" => {
            all::WINDOWS_1254.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1255" => {
            all::WINDOWS_1255.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded  
        },
        "windows-1256" => {
            all::WINDOWS_1256.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded 
        },
        "windows-1257" => {
            all::WINDOWS_1257.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded 
        },
        "windows-1258" => {
            all::WINDOWS_1258.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded 
        },
        "mac-cyrillic" | "x-mac-cyrillic" => {
            all::MAC_CYRILLIC.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "ascii" => {
            all::ASCII.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "big5-2003" => {
            all::BIG5_2003.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "euc-jp" => {
            all::EUC_JP.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "gb18030" => {
            all::GB18030.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "gbk" => {
            all::GBK.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "hz" => {
            all::HZ.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "iso-2022_jp" => {
            all::ISO_2022_JP.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "uft-16be" => {
            all::UTF_16BE.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "uft-16le" => {
            all::UTF_16LE.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "windows-31j" => {
            all::WINDOWS_31J.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        },
        "windows-949" => {
            all::WINDOWS_949.decode_to(src, trap, &mut src_decoded).unwrap();
            src_decoded
        }
        _ => { src.as_utf8() }
    };

    result 
}
