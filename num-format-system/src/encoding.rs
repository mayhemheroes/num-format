#![allow(dead_code)]

use std::borrow::Cow;

use crate::error::Error;

// See https://docs.rs/encoding_rs/0.8.16/encoding_rs/
pub static LATIN_1: &'static encoding_rs::Encoding = encoding_rs::WINDOWS_1252;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Encoding(&'static encoding_rs::Encoding);

impl Encoding {
    pub(crate) fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, Error> {
        let (cow, _encoding, is_err) = self.0.decode(bytes);
        if is_err {
            return Err(Error::decoding(bytes, self.name()));
        }
        Ok(cow)
    }

    pub(crate) fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl Encoding {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Encoding, Error> {
        if let Some(encoding) = encoding_rs::Encoding::for_label_no_replacement(bytes) {
            return Ok(Encoding(encoding));
        }

        // Helpful: https://github.com/servo/libparserutils/blob/master/build/Aliases
        let encoding = match bytes {
            // Assume empty bytes means use LATIN-1 ...
            b"" => LATIN_1, // Is this correct?

            // Naming issues...
            b"Big5HKSCS" => encoding_rs::BIG5,
            b"CP949" => encoding_rs::EUC_KR,
            // See https://en.wikipedia.org/wiki/GB_18030 and
            // https://www.ibm.com/support/knowledgecenter/en/ssw_aix_72/com.ibm.aix.nlsgdrf/ibm-eucCN.htm
            b"eucCN" => encoding_rs::GB18030,
            b"eucJP" => encoding_rs::EUC_JP,
            b"eucKR" => encoding_rs::EUC_KR,

            // These are not correct, but seem to only
            // use LATIN-1 characters for number formatting ...
            b"ARMSCII-8" => LATIN_1,
            b"CP1131" => LATIN_1,
            b"ISCII-DEV" => LATIN_1,
            b"PT154" => LATIN_1,

            // If all of the above fail, return an error ...
            _ => {
                let label = String::from_utf8_lossy(bytes);
                return Err(Error::unsupported_encoding(&label));
            }
        };

        Ok(Encoding(encoding))
    }
}
