// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
// 
// Email: hex0x0000@protonmail.com

pub mod error;
pub mod plugin;

pub use actix_multipart;
pub use actix_web;
pub use async_trait::async_trait;
pub use serde_json;
pub use tiny_args;
pub use toml;

/// Alias of [`serde_json::Value`] to avoid confusions with [`toml::Value`]
pub type Json = serde_json::Value;

/// Alias of [`toml::Value`] to avoid confusions with [`serde_json::Value`]
pub type Toml = toml::Value;

#[cfg(test)]
mod tests {}
