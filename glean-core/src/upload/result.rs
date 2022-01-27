// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Result values of attempted ping uploads encoded for FFI use.
///
/// In a perfect world this would live in `glean-ffi`,
/// but because we also want to convert from pure integer values to a proper Rust enum
/// using Rust's `From` and `Into` trait, we need to have it in this crate
/// (The coherence rules don't allow to implement an external trait for an external type).
///
/// Due to restrictions of cbindgen they are re-defined in `glean-core/ffi/src/upload.rs`.
///
/// NOTE:
/// THEY MUST BE THE SAME ACROSS BOTH FILES!
pub mod ffi_upload_result {
    /// A recoverable error.
    pub const UPLOAD_RESULT_RECOVERABLE: u32 = 0x1;

    /// An unrecoverable error.
    pub const UPLOAD_RESULT_UNRECOVERABLE: u32 = 0x2;

    /// A HTTP response code.
    ///
    /// The actual response code is encoded in the lower bits.
    pub const UPLOAD_RESULT_HTTP_STATUS: u32 = 0x8000;
}
use ffi_upload_result::*;

/// The result of an attempted ping upload.
#[derive(Debug)]
pub enum UploadResult {
    /// A recoverable failure.
    ///
    /// During upload something went wrong,
    /// e.g. the network connection failed.
    /// The upload should be retried at a later time.
    RecoverableFailure {
        #[doc(hidden)]
        /// Unused field. Required because UniFFI can't handle variants without fields.
        unused: i8,
    },

    /// An unrecoverable upload failure.
    ///
    /// A possible cause might be a malformed URL.
    UnrecoverableFailure {
        #[doc(hidden)]
        /// Unused field. Required because UniFFI can't handle variants without fields.
        unused: i8,
    },

    /// A HTTP response code.
    ///
    /// This can still indicate an error, depending on the status code.
    HttpStatus {
        /// The HTTP status code
        code: i32,
    },
}

impl From<u32> for UploadResult {
    fn from(status: u32) -> Self {
        match status {
            status if (status & UPLOAD_RESULT_HTTP_STATUS) == UPLOAD_RESULT_HTTP_STATUS => {
                // Extract the status code from the lower bits.
                let http_status = status & !UPLOAD_RESULT_HTTP_STATUS;
                UploadResult::http_status(http_status as i32)
            }
            UPLOAD_RESULT_RECOVERABLE => UploadResult::recoverable_failure(),
            UPLOAD_RESULT_UNRECOVERABLE => UploadResult::unrecoverable_failure(),

            // Any unknown result code is treated as unrecoverable.
            _ => UploadResult::unrecoverable_failure(),
        }
    }
}

impl UploadResult {
    /// Gets the label to be used in recording error counts for upload.
    ///
    /// Returns `None` if the upload finished succesfully.
    /// Failures are recorded in the `ping_upload_failure` metric.
    pub fn get_label(&self) -> Option<&str> {
        match self {
            UploadResult::HttpStatus { code: 200..=299 } => None,
            UploadResult::HttpStatus { code: 400..=499 } => Some("status_code_4xx"),
            UploadResult::HttpStatus { code: 500..=599 } => Some("status_code_5xx"),
            UploadResult::HttpStatus { .. } => Some("status_code_unknown"),
            UploadResult::UnrecoverableFailure { .. } => Some("unrecoverable"),
            UploadResult::RecoverableFailure { .. } => Some("recoverable"),
        }
    }

    /// A recoverable failure.
    ///
    /// During upload something went wrong,
    /// e.g. the network connection failed.
    /// The upload should be retried at a later time.
    pub fn recoverable_failure() -> Self {
        Self::RecoverableFailure { unused: 0 }
    }

    /// An unrecoverable upload failure.
    ///
    /// A possible cause might be a malformed URL.
    pub fn unrecoverable_failure() -> Self {
        Self::UnrecoverableFailure { unused: 0 }
    }

    /// A HTTP response code.
    ///
    /// This can still indicate an error, depending on the status code.
    pub fn http_status(code: i32) -> Self {
        Self::HttpStatus { code }
    }
}
