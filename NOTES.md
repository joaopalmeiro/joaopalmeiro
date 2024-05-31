# Notes

- https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-github-profile/customizing-your-profile/managing-your-profile-readme
- https://hyperskill.org/learn/step/29730
- https://jqlang.github.io/jq/manual/#sort-sort_by

## Commands

```bash
jq --help
```

```bash
jq 'sort_by(.category, .subcategory, .name)' data.json > sorted.json && mv sorted.json data.json
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
