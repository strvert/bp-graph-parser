# ue-serialized-text-parser

UnrealEngineから得られるオブジェクトのテキスト表現データのパーサー。開発中。

- [x] オブジェクトプロパティののパース
  - [x] 文字列リテラル
    - "ABCD"
  - [x] 整数リテラル
    - 1234
  - [x] 浮動小数点数リテラル
    - 14.67
  - [x] Uuidリテラル
    - F6D0DA4A4AA531533341018A20422309
  - [x] NSLOCTEXTリテラル
    - NSLOCTEXT("K2Node", "Target", "Target")
  - [x] オブジェクトリテラル
    - Class'"/Script/Engine.Actor"'
  - [x] 接続先リストリテラル
    - (K2Node_DynamicCast_46 5EE02C3B480C2249B48954B390C035D6,K2Node_CallFunction_1093 0710E8C14EFFED0DD9E024BCB29F23C3,)
  - [x] プロパティリストリテラル
    - 他のプロパティによって構成されるリストのリテラル
- [x] オブジェクトのパース
- [x] CRLFへの対応
- [x] 複数オブジェクトのパース
- [ ] 入れ子オブジェクトのパース
- [ ] プログラムでの利便性が高い一般的なフォーマット(json, yamlなど)への再構築
- [ ] WASMを用いたJSライブラリ化
