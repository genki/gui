# CLI

## コマンド

```sh
gui check examples/demo.gui
gui check examples/demo.gui other.gui
gui check

gui page examples/demo.gui
gui page
gui drill
gui inherit
gui node
gui nav

gui scan page1.html page2.html
gui scan --stage summary state.snapshot.yaml
gui compare left.snapshot.yaml right.snapshot.yaml
```

## 入力解決

ファイル引数を省略した場合は、カレントディレクトリ配下を再帰走査して対象
ファイルを集めます。

複数の `.gui` ファイルを渡した場合は、1 つの論理 document として merge して
からコマンドを実行します。

ディレクトリを渡した場合も、配下を再帰走査して対象ファイルを集めます。

- `check`, `page`, `drill`, `inherit`, `node`, `nav`: `*.gui`
- `scan`: `*.html`, `*.htm`, `*.yaml`, `*.yml`
- `compare`: `*.html`, `*.htm`, `*.yaml`, `*.yml`

## コマンド概要

- `check`: `.gui` を parse / validate
- `page`: 現在の page 規則に合致する node 一覧
- `drill`: `drill` 木をインデント付き表示
- `inherit`: `inherit` 木をインデント付き表示
- `node`: node id 一覧
- `nav`: nav id 一覧
- `scan`: rendered HTML 群から `.gui` を推定して stdout へ出力
- `scan --stage summary`: `.gui` ではなく YAML summary を出力
- `compare`: 2 つの HTML / snapshot manifest を summary 化して差分レポートを出力

## よくある使い方

```sh
gui check examples/demo.gui
gui page
gui scan saved/home.html saved/pricing.html > site.gui
gui scan --stage summary saved/wizard.snapshot.yaml
gui compare saved/origin.snapshot.yaml saved/clone.snapshot.yaml
```

## 比較ワークフロー

app 固有 config を使う例:

```sh
gui compare --config app-scan-config.yaml \
  saved/origin.snapshot.yaml \
  saved/clone.snapshot.yaml
```

現在の compare は主に次の finding を出します。

- `missing-dialog`
- `missing-control`
- `unexpected-control`
- `state-hint-mismatch`
- `stepper-mismatch`
- `nav-mismatch`
- `nav-label-mismatch`

主に使う config:

- `stepper`: wizard / stepper 抽出用 selector と active 判定ヒント
- `snapshot`: `stateHints` のうち flow/step として扱う key 名
- `compare.dynamic_selectors`: 可変リストや avatar のような領域
- `compare.dynamic_text_patterns`: 可変 text の regex
- `compare.normalize_patterns`: 比較前に text を正規化する regex 置換

## 注意

- `gui scan` 自体はページ取得や JavaScript 実行を行いません。
- HTML の取得は別ツールの責務です。
