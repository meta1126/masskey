## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
## CodeGraph と Graphify の使い分け(上の graphify のルールより優先する)
- 関数の呼び出し関係、シンボルの定義・参照、特定のコードの中身を知りたいときは、まず codegraph_explore を使う。
- リポジトリ全体の構造、モジュール間の関係、どこに何があるかの俯瞰は、graphify query / path / explain を使う。結果の INFERRED / AMBIGUOUS は推測を含むので、重要な箇所は元のソースで確認する。
- 迷ったら codegraph_explore を先に使う。
- コードを編集した後の graphify update . は、ユーザーが頼んだときだけ実行する(git の post-commit フックで更新するため)。
