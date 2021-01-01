# メモ
## グラフ + 最短距離
### 1点からの距離のみで良い
#### ダイクストラ
- 注意事項
  - 負の辺があると使えない
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

### 複数の点から
#### ワーシャルフロイド
- 注意事項
  - 負の辺があっても使える
  - 負の閉路があると使えない
  - O(V^3)になる。遅い
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

## 点更新、区間取得
### Fenwick tree (Binary indexed tree, BIT)
- できること
  - 要素の追加
  - 特定区間の値の更新
  - 特定区間の値の和の取得
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/range_xor_query.rs
