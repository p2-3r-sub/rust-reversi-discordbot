# rust-reversi-discordbot

**discordでリバーシと量子五目並べが遊べるBOT**

~~別に高速化の必要はないけどRustでコードを書いてみたかったので~~

(五目並べのために画像を描画しているので意味出てきたかも)

練習のついでに作ってみました。実行ファイルの配布はしていないので

~~いないとは思いますが~~実行してみたい方は自分でビルドしてください。

<small>多分コードは結構汚いです(´・ω・｀)</small>

量子五目並べも追加しました！ (リポジトリ名はリバーシボットですが...)

実装した量子五目並べは QuizKnock さんのこの動画が元ネタです。

QuizKnock : [【理解不能】何色になるか分からない量子で五目並べやってみた【でも楽しそう】](https://www.youtube.com/watch?v=mitAxA3f4U4)

## 使用方法

ビルドする際にfontフォルダに五目並べの文字描画用のフォント `./font/font.ttf` を何か入れてください。

`config.json` の `TOKEN` にBOTのトークンを書いて起動してください。

IntentsはすべてONにしておいてください。 

## BOTの使用方法

`/ping`

Pong!

`/reversi_start` `/q_gomoku_end`

指定したユーザーと試合を開始します。

コマンドを使用した人が先行 (黒) になります。

`/reversi_end` `/q_gomoku_start`

試合を終了します。

1つのチャンネルで同時に一つの試合しか行えません。