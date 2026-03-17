# Notes

- https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-github-profile/customizing-your-profile/managing-your-profile-readme
- https://hyperskill.org/learn/step/29730
- https://jqlang.github.io/jq/manual/#sort-sort_by
- https://stackoverflow.com/questions/72277908/how-to-sort-case-insensitive-using-jq
- https://xojoc.pw/blog/jq-sort-json
- "It uses `tostring` to convert each value to a string, which handles non-string values [(e.g., `null`)]."
- https://github.com/serde-rs/json
  - https://crates.io/crates/serde_json
  - https://serde.rs/derive.html
    - `serde = { version = "1.0.228", features = ["derive"] }`
- https://koenwoortman.com/rust-read-file-to-string/
- https://doc.rust-lang.org/std/string/struct.String.html#examples: `let mut hello = String::from("Hello, ");` + `hello.push_str("orld!");`
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html: "`Ownership` is a set of rules that govern how a Rust program manages memory."
- https://doc.rust-lang.org/beta/std/collections/btree_map/enum.Entry.html#method.or_default

## Development

Install [rustup](https://rust-lang.org/tools/install/) (if necessary):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
jq 'sort_by((.category | ascii_downcase), (.name | ascii_downcase))' data.json > sorted.json && mv sorted.json data.json
```

```bash
cargo run
```

```bash
cargo fmt
```

## Commands

```bash
jq --help
```

```bash
jq 'sort_by((.category | ascii_downcase), (.subcategory | tostring | ascii_downcase), (.name | ascii_downcase))' data.json > sorted.json && mv sorted.json data.json
```

```bash
jq 'sort_by(.category, .subcategory, .name)' data.json > sorted.json && mv sorted.json data.json
```

```bash
jq 'empty' data.json
```

## Snippets

```json
{
  "name": "",
  "url": "",
  "description": "",
  "category": ""
}
```

```json
{
  "name": "",
  "url": "",
  "description": "",
  "category": "",
  "subcategory": null
}
```

```markdown
- [](): .
```
