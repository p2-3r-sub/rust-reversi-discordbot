# rust-reversi-discordbot

**discordでリバーシが遊べるBOT**

別に高速化の必要はないけどRustでコードを書いてみたかったので

練習のついでに作ってみました。実行ファイルの配布はしていないので

~~いないとは思いますが~~実行してみたい方は自分でビルドしてください。

<small>多分コードは結構汚いです(´・ω・｀)</small>

## 使用方法

`config.json` の `TOKEN` にBOTのトークンを書いて起動してください。

IntentsはすべてONにしておいてください。 

## BOTの使用方法

`/ping`

Pong!

`/match_start`

指定したユーザーと試合を開始します。

コマンドを使用した人が先行 (黒) になります。

`/match_end`

試合を終了します。

1つのチャンネルで同時に一つの試合しか行えません。