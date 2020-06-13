// Components for simple use of GTK+ Glade UI bindings in Rust
//
// Written in 2020 by
//     Dr. Maxim Orlovsky <dr.orlovsky@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![feature(try_trait)]

#[macro_use]
extern crate derive_wrapper;

mod error;
pub use error::Error;

#[macro_export]
macro_rules! glade_load {
    ($builder:ident, $file:literal) => {
        $builder.get_object($file).ok_or($crate::Error::ParseFailed)
    };
}

pub trait View
where
    Self: Sized,
{
    fn load_glade() -> Result<Self, Error>;
}
