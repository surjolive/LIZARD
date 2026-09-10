# LIZARD Language Support

Native VS Code support for `.lz` files.

This package registers the LIZARD language, grammar, snippets, run/check commands, and document formatting through the real `lizard fmt` CLI.

Configure the executable when it is not on `PATH`:

```json
{
  "lizard.path": "C:/path/to/lizard.exe",
  "lizard.formatOnSave": true
}
```

Build the extension with `npm install` followed by `npm run package`.
