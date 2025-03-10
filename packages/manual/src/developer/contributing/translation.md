# Translations

```admonish info
When making a PR for translation, please also note if you are willing
be a long term maintainer for the language or not. If yes, please
also join my [Discord](/index.md#discord).

Whenever new texts are added, I will translate them with AI first
and send screenshots for the long term maintainers to review if the translation
makes sense in the context.
```

The translation files are located in `packages/localization/src/ui/*.yaml`.

Here's what you need to know when modifying the translation files:
- Each translation entry is in the format of `<key>: "<value>"`
- To escape `"` inside the value, put `\"`.
- Tokens like `{{data}}` are placeholders that will be replaced
  with the actual text in the app. Keep this in mind if you can't find the entry
  you are looking for.

To add new translations, follow these steps:
- Add the new key and English value in `en-US.yaml`
- Prepare a translation file like this:
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
- Run `task exec -- localization:edit < path/to/your/file`. This will apply
  the edits to the translation files and make sure everything is formatted.
  Note that existing values will be overriden and values not found in English
  will be deleted.


   
