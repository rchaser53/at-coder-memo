# メモ
## グラフ + 最短距離
### 1点からの距離のみで良い
#### ダイクストラ
- 負の辺があると使えない
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

### 複数の点から
#### ワーシャルフロイド
- 負の辺があっても使える
- 負の閉路があると使えない
- O(V^3)になる。遅い
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

#### ベルマンフォード < ダイクストラ？
- 正の重みのみならダイクストラの方が早い
- 負の重みの検出に使える？
- 負の重みがあっても処理ができる？

## 点更新、区間取得
### Fenwick tree (Binary indexed tree, BIT)
- できること
  - 要素の追加
  - 特定区間の値の更新
  - 特定区間の値の和の取得
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/range_xor_query.rs

- 備考
  - 転倒数を求めるのに使える
  - その場合1-indexedで実装した方が楽
  - ref: https://github.com/rchaser53/at-corder-memo/commit/d32b7913eddf25605c312fe8bb7d57fad2b88a34
