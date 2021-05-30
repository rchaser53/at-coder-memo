# 貪欲法の証明の基本
## 背理法
- 貪欲法に従わなかった解が貪欲解より大きくなると仮定
- 解が悪化しない入れ替えを行うことで、貪欲回に直せるので矛盾

小さい箱から順に処理していく

https://atcoder.jp/contests/abc195/tasks/abc195_d のケースだと以下の2パターン
- 入れられるのに入れなかった
- 最大でないものを入れた

1. 上のパターンのものに対して最良の結果になるものを代わりに入れて、入れ替える
2. 他の物が上のパターンになってしまうケースもあるが気にせずに続ける
3. どこかの段階で貪欲法の回答と一緒になる

貪欲方に従わなかった解が貪欲解より大きくなるという仮定が矛盾、
価値が大きいものから順に入れていけば良いということが証明できる