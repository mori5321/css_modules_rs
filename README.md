
# MileStone
## Phase1
- [ ] .cssファイルをbundleする
  - [ ] 該当CSSファイルの取得
  - [ ] CSSの検証
  - [ ] CSSの変換(uuid or timestampを用いたclass名に)
  - [ ] bundle.cssファイルに結合

- [ ] 対応表作成
  - [ ] "旧class名: 変換後Class名 + filepath" という対応表(HashMap?)を作成する
  - [ ] 対応表を何かしらの形で保持する。(onMemory? schemaファイル?)

- [ ] マクロでRust側を書き換える
  - [ ] css_module!(./style.css) という記法
  - [ ] filepathに該当するHashMapを対応表から取得する


こんな感じが良さそう
``` sample.rs
  let style = css_module!("./sample.css");
  
  html! {
    <div styleName={style.hello}>Hello World</div>
  }
```

# Phase2
- [ ] Refactor
  - [ ] Core機能とYew(or Wasm)とのbind部分を分割する

# Phase3
- [ ] CSSの主要機能への対応をチェック
- [ ] CSSの主要機能へ対応
- [ ] PostCSSの主要機能へ対応(composeだったかimportだったか)
