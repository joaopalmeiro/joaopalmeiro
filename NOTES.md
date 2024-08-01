# Notes

- https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-github-profile/customizing-your-profile/managing-your-profile-readme
- https://hyperskill.org/learn/step/29730
- https://jqlang.github.io/jq/manual/#sort-sort_by
- https://stackoverflow.com/questions/72277908/how-to-sort-case-insensitive-using-jq
- https://xojoc.pw/blog/jq-sort-json
- "It uses `tostring` to convert each value to a string, which handles non-string values [(e.g., `null`)]."

## Commands

```bash
jq --help
```

```bash
jq 'sort_by(.category, .subcategory, .name)' data.json > sorted.json && mv sorted.json data.json
```

```bash
jq 'sort_by((.category | ascii_downcase), (.subcategory | tostring | ascii_downcase), (.name | ascii_downcase))' data.json > sorted.json && mv sorted.json data.json
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
  "category": "",
  "subcategory": null
}
```

```markdown
- [](): .
```
