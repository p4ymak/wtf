# wtf
Web Text Finder - CLI tool to start web browser with search engine depending on language of query.

If the first alphabetic character of query is cyrillic it sends it to Yandex, otherwise to Google.

https://www.instagram.com/p/CEZ-gAIjIIB/
I personally use wtf in NeoVim as a shortcut.

To compile it to your OS you will need Rust installed. Then:

cd zenclock

cargo build --release


To use it as CLI command, add it to PATH.

In Linux add line to ***~/.bashrc***

export PATH="/wherever_it_is/wtf/target/release/:$PATH"
