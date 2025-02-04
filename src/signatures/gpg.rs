use crate::extractors::zlib::zlib_decompress;
use crate::signatures::common::{SignatureError, SignatureResult, CONFIDENCE_HIGH};

/// Human readable description
pub const GPG_SIGNED_DESCRIPTION: &str = "GPG signed file";

/// GPG signed files start with these two bytes
pub fn gpg_signed_magic() -> Vec<Vec<u8>> {
    return vec![b"\xA3\x01".to_vec()];
}

/// Validates GPG signatures
pub fn gpg_signed_parser(
    file_data: &Vec<u8>,
    offset: usize,
) -> Result<SignatureResult, SignatureError> {
    // Success result; confidence is high since this signature is only reported what it starts at the beginning of a file
    let result = SignatureResult {
        offset: offset,
        confidence: CONFIDENCE_HIGH,
        description: GPG_SIGNED_DESCRIPTION.to_string(),
        ..Default::default()
    };

    // This is enforced in magic.rs so this check is supurfulous
    if offset == 0 {
        /*
         * GPG signed files are just zlib compressed files with the zlib magic bytes replaced with the GPG magic bytes.
         * Decompress the signed file; no output directory specified, dry run only.
         */
        let decompression_dry_run = zlib_decompress(&file_data, offset, None);

        // If the decompression dry run was a success, this signature is almost certianly valid
        if decompression_dry_run.success == true {
            return Ok(result);
        }
    }

    return Err(SignatureError);
}
