// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Widgets are complex interactive components that:
//!
//! - Maintain their own internal state
//! - Respond to user interaction messages
//! - Be embedded in parent models as fields
//! - Have their messages mapped to parent messages
//!
//! These widgets implement both the Model trait (for state management)
//! and the View trait (for rendering data).

pub mod button;

pub use button::*;

// End of File
