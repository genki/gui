# HTML解析

`gui scan` は保存済みの rendered HTML を読み、抽象 `.gui` を出力します。
`--stage summary` を付けると、比較や後段処理に向いた YAML summary を出力します。

## 現在の抽出方針

### page 構造

- canonical URL があれば file path 推定より優先
- canonical がなければ file 名から path を推定
- `drill` の親推定では path prefix より breadcrumb を優先

### nav

- `nav`, `tablist`, `header`, `footer` など高信頼 container を対象にする
- 繰り返し出る link 集合を `nav` cluster としてまとめる
- 近似 duplicate な cluster は統合する

### 抑止 heuristic

ノイズになりやすい次の構造は意図的に抑止します。

- `login`, `cart`, `checkout` など action 寄り導線
- page host が不明なときの absolute 外部 URL
- locale switcher
- 巨大 footer directory
- 巨大 docs index nav

### dialog

- `dialog`, `role=dialog`, `role=alertdialog`, `aria-modal=true` を `kind: dialog` として抽出
- trigger は `opens` 関係で表現
- `aria-controls`, `href=#id`, `data-dialog*`, `data-modal-target` を使って結び付ける
- 複数 page で共通に開かれる dialog は layout 側へ昇格する場合がある

### stepper

- wizard / stepper 構造は indicator selector、tablist 風 container、semantic class hint から推定する
- summary には `labels` と `active_label` を記録する
- 抽象 `.gui` では最初に見つかった stepper を page state 配下の `flow-step` 木として出力できる

### snapshot manifest

`gui scan` は YAML の snapshot manifest も入力にできます。各 snapshot には次を持てます。

- `id`
- `url`
- `html`
- `actions`
- `stateHints`

これらは summary に保持され、page 階層推定にも使われます。

### dynamic 領域と text 正規化

config を渡した場合は、比較向けに可変領域と text 正規化を扱えます。

- `compare.dynamic_selectors`: 該当する control / nav / list / image を dynamic 扱いにする
- `compare.dynamic_text_patterns`: 該当する text 値を dynamic 扱いにする
- `compare.normalize_patterns`: 比較前に regex で text を正規化する

### kind

現在の `node.kind`:

- `page`
- `section`
- `layout`
- `action`
- `index`
- `dialog`

dialog には追加で `dialog-kind` が付きます。

- `generic`
- `form`
- `confirm`
- `alert`
- `consent`
- `sheet`
- `picker`
- `promo`

## まだ弱い点

- 主 nav / 補助 nav の順位付け
- 同一実体 page の alias 統合
- semantic attribute がない JS 専用 modal trigger
- 大規模 docs 群での taxonomy 推定
- diff cluster と具体的な DOM box の対応付け

設計の背景は [`spec/scan.md`](../../spec/scan.md) を参照してください。
