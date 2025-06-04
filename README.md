# 🌐 Language Mappings for [anticheat-test.com](https://anticheat-test.com)

> ⚠️ **Warning**  
> This project is a work in progress. New strings will be added regularly.

## 📄 Overview

This repository contains language mappings for the [anticheat-test.com](https://anticheat-test.com) server. Contributors can add or update translations by making a pull request.

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

   Use `English.json` as the template for new translations.

4. **Create your translation file:**

   Copy `English.json` and rename it to your target language name, e.g., `German.json` for German.

## 🛠️ Contributing

We welcome contributions! Please adhere to the following guidelines:

- **Use `English.json` as your base.** To have the all the language keys.
- **Avoid using translators.** Refrain from using tools like Google Translate. If uncertain about a translation, it's better to leave it unchanged.
- **Preserve placeholders.** Do not modify placeholders like `%s`; they are essential for string formatting.
- **Validate your JSON.** Use a [JSON parser](https://jsonformatter.org/json-parser) to ensure your file is correctly formatted.
- **Update `loader_mappings.json`.** Add your new language to this file to ensure it's deployed to servers.

## 🎨 Texture IDs

For texture IDs, we utilize assets from [minecraft-heads.com](https://minecraft-heads.com). If you find a head that represents your language, feel free to include it.

## ✅ Checklist before making a pull request

- JSON file is valid and properly formatted.
- Placeholders like `%s` are intact.
- `loader_mappings.json` is updated with the new language.
- No translators are used.
- The translation file is named correctly (e.g., `French.json` for French).
