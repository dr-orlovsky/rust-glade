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

#[derive(Clone, PartialEq, Eq, Debug, Display, From, Error)]
#[display_from(Debug)]
pub enum Error {
    ParseFailed,

    #[derive_from(std::option::NoneError)]
    WidgetNotFound,
}
