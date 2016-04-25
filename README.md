alias-auto-add
==============

Adds unique aliases to a Mutt alias file.

When viewing an email in Mutt, this program is set as a display filter that
stores an alias for the from address. Aliases will thus be generated
automatically when reading email.

The program reads an email from STDIN and tries to add an alias for the from
address listed. If the given alias already exists, a new unique alias is
generated and used instead. This allows us to always capture an alias even if a
person has multiple email addresses.

Inspired by W. Caleb McDaniel's aliasing script published at:
http://wcm1.web.rice.edu/mutt-tips.html. This adds the advantage of creating a
new unique alias for all email addresses regardless of whether the sender's name
is duplicated.


## Install
A binary built for Mac OS X is available on the [releases][1] page. Download the
binary, put it in your `PATH`, and skip to step 2.

For other platforms, you'll need the [Rust][2] compiler.

1. Install with `cargo`:

		$ cargo install --git https://github.com/teddywing/alias-auto-add.git --root /usr/local

2. Add the program as a Mutt display filter. Add the following line to your
   `.muttrc`, specifying the location of your Mutt aliases file as an argument
   to `alias-auto-add`:

		set display_filter = /usr/local/alias-auto-add ~/.mutt/aliases


## Uninstall

	$ cargo uninstall --root /usr/local alias-auto-add


## License
Copyright © 2016 Teddy Wing. Licensed under the GNU GPL (see the included
COPYING file).


[1]: https://github.com/teddywing/alias-auto-add/releases
[2]: https://www.rust-lang.org/
