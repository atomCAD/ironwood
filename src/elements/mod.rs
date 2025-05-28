// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Elements are simple display components that:
//!
//! - Hold only styling and content data
//! - Have no internal state or messages
//! - Implement only the View trait (for rendering)
//! - Serve as building blocks for more complex user interfaces
//!
//! These elements are pure data structures that describe what should
//! be displayed, with all styling and content configured at creation time.

pub mod layout;
pub mod text;

pub use layout::{Alignment, HStack, Spacer, VStack};
pub use text::Text;

// End of File
