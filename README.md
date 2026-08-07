# 🌐 Language Mappings for [anticheat-test.com](https://anticheat-test.com)

> ⚠️ **Warning**  
> This project is a work in progress. New strings will be added regularly.

## 📄 Overview

This repository contains language mappings for the [anticheat-test.com](https://anticheat-test.com) server. Contributors can add or update translations by making a pull request.

Translation files use UTF-8 `.properties` files. `English.properties` is the source for the available keys.

## 🚀 Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/anticheat-test/anticheat-test.com-translations.git
   ```

2. **Navigate to the project directory:**

   ```bash
   cd anticheat-test.com-translations
   ```

3. **Choose the base file:**

   Use `English.properties` as the template for new translations.

4. **Create or update your translation file:**

   Copy `English.properties` and rename it to your target language, for example `German.properties`.

Properties use the following format:

```properties
scoreboard.ping=Ping
scoreboard.anticheat=Anticheat
```

Keep the key on the left unchanged and translate the value on the right.

## 🛠️ Contributing

We welcome contributions! Please adhere to the following guidelines:

- **Use `English.properties` as your base** so the translation contains all current keys.
- **Keep keys synchronized** with `English.properties`; do not remove or rename keys from the English file.
- **Preserve placeholders** such as `%s`; they are essential for formatting.
- **Preserve comments** beginning with `#` or `!`.
- **Avoid using translators.** If uncertain about a translation, it is better to leave it unchanged.
- **Update `index.json`** when adding a language or changing its metadata.

## 🎨 Texture IDs

Texture IDs and translation credits are stored in `index.json`. For texture IDs, we use assets from [minecraft-heads.com](https://minecraft-heads.com).

## ✅ Checklist before making a pull request

- The translation file is valid `.properties` syntax.
- Keys match `English.properties`.
- Placeholders such as `%s` are intact.
- Comments and intended whitespace are preserved.
- `index.json` is updated when needed.
- The translation file is named correctly, for example `French.properties`.
