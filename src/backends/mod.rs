// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Backend implementations for Ironwood UI Framework
//!
//! This module contains various backend implementations that can render
//! Ironwood views on different platforms. Each backend implements the
//! ViewExtractor trait for the views it supports.
//!
//! Available backends:
//! - `mock`: Testing backend that extracts views into simple data structures

pub mod mock;

pub use mock::{MockBackend, MockButton, MockHStack, MockSpacer, MockText, MockVStack};

// End of File
