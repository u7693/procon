# Setup

[cargo-atcoder](https://github.com/tanakh/cargo-atcoder)のmacOS環境でのセットアップ

```bash
# GNU版のstripを入れる
$ brew install binutils

# rust-embedded/crossを入れる
$ cargo install cross

# 設定ファイルをリンク
$ stow -v -t ~/Library/Preferences config

# cargo-atcoderを入れる
$ cargo install --git https://github.com/tanakh/cargo-atcoder.git
```
