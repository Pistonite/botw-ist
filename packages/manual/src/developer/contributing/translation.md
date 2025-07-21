# Translations

The translation files are located in `packages/localization/src/ui/*.yaml`.

Here's what you need to know when modifying the translation files:
- Each translation entry is in the format of `<key>: "<value>"`
- To escape `"` inside the value, put `\"`.
- Tokens like `{{data}}` are placeholders that will be replaced
  with the actual text in the app. Keep this in mind if you can't find the entry
  you are looking for.

To add new translations, follow these steps:
1. Add the new key and English value in `en-US.yaml`
2. Prepare a translation file like this:
   ```yaml
    # en-US is optional and will be ignored, 
    en-US: 
      button.ok: "OK"
    # order of languages/keys below does not matter
    # you can also include multiple keys
    de-DE:
      button.ok: "OK"
    es-ES:
      button.ok: "Aceptar"
    ko-KR:
      button.ok: "확인"
    zh-CN:
      button.ok: "好的"
    ```
3. Go to the `localization` package (`cd packages/localization`)
4. Run the `edit` task and pass in your file:
   ```
   task edit -- path/to/your/file
   ```
  This will apply
  the edits to the translation files and make sure everything is formatted.
  Note that existing values will be overridden and values not found in English
  will be deleted.

To change the translation for one language, simply modify the language files.
