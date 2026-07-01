# FileForge v0.3

Rust製の巨大ファイル解析ツールです。

## 主な構成

- fileforge-cli
- fileforge-core
- fileforge-engine
- fileforge-storage
- fileforge-analyzer
- fileforge-encoding
- fileforge-text
- fileforge-csv
- fileforge-jsonl
- fileforge-query
- fileforge-index

## 主な機能

- txt/log検索
- 複数キーワード検索
- regex検索
- JSONL key/value検索
- .ffidxインデックス作成
- 行ジャンプ
- AnalyzerManager
- Text/CSV/JSONL Analyzer
- ReaderFactory
- LineReader / ChunkReader / MemoryMapReader
- UTF-8/BOM/UTF-16簡易判定
- Query: contains(text,"ERROR") / AND / OR / NOT

## 実行例

```bash
cargo run -p fileforge-cli -- analyze testdata/sample.log
cargo run -p fileforge-cli -- analyze testdata/sample.csv
cargo run -p fileforge-cli -- analyze testdata/sample.jsonl
cargo run -p fileforge-cli -- query testdata/sample.log "ERROR OR WARN"
cargo run -p fileforge-cli -- query testdata/sample.log "contains(text,\"ERROR\")"
```
