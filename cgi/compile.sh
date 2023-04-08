rm -rf ./target
cargo build --release
me="$(whoami)"
if (me="stephane")
  then mv ./target/release/numerologie_core /Applications/MAMP/cgi-bin/numerologie_core.cgi
else
  mv ./target/release/numerologie_core /usr/lib/cgi-bin/numerologie_core.cgi
fi
