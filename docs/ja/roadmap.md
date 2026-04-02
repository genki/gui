# ロードマップ

## 現在できること

- `.gui` の parse / import / merge / validate
- CLI の一覧・検査コマンド
- HTML scan による page / section / layout / action / index / dialog 抽出
- nav 抽出と過検出抑止
- dialog trigger 抽出と layout への昇格

## 現在の弱点

- 主 nav / 補助 nav の順位付け
- 同一 page を表す複数 URL の統合
- docs taxonomy と site-wide nav の分離
- semantic attribute がない JS modal trigger の検出
- page / layout を超えた dialog 所属推定
- path heuristic だけでは弱い共通 layout 抽出

## 推奨する解決策

- scan と最終抽象化の間に alias 正規化段階を追加する。
  - `canonical`, `og:url`, path 正規化, title 類似, breadcrumb を使い、
    複数 URL を 1 つの論理 page id へ collapse する。
- nav を出力前に ranking する。
  - 位置, 複数 page での再出現率, target 集合の安定性, active state,
    aria-label などを使って `primary`, `secondary`, `footer`, `local`
    に分類する。
- 大規模 docs 構造は捨てずに taxonomy として保持する。
  - 大きな index を単に suppress するのではなく、専用 kind として
    保持する。
- DOM fingerprint による layout 推定段階を追加する。
  - first path segment や shared nav だけでなく、複数 page に共通する
    非 root subtree を比較して layout を推定する。
- dialog trigger 推定を confidence 付きで拡張する。
  - `onclick`, `data-*`, 近傍文言, id 類似を使うが、弱い推定は確信度を
    持たせて区別できるようにする。

## 優先順

1. alias 正規化
2. nav ranking
3. docs 系 taxonomy 分類
4. DOM fingerprint による layout 推定
5. confidence 付き dialog trigger 推定

## 実装タスク

- scan pipeline に明示的な `normalize` 段階を導入する。
  - `scan -> normalize -> classify -> abstract`
- 中間データ構造として次を定義する。
  - page alias
  - nav score と nav role
  - taxonomy candidate
  - subtree fingerprint
  - dialog trigger confidence
- 少なくとも次の fixture 群を用意する。
  - marketing site
  - docs site
  - commerce site
  - app/dashboard site
- source を直接 patch せずに heuristic を確認できるよう、中間段階の
  debug 出力を追加する。

## 次の改善候補

- nav ranking の強化
- page alias 正規化
- dialog trigger heuristic の拡張
- docs/site taxonomy 推定の強化
- scan 中間段階の debug 出力
