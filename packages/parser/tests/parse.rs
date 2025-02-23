use anyhow::bail;
use indoc::indoc;
use teleparse::Root;

use skybook_parser::search::{QuotedItemResolver, ResolvedItem};

#[tokio::test]
async fn parse_simple() -> anyhow::Result<()> {
    let script = indoc! {r#"
        eat axe in slot 3;
        get 3 apple[life = true, time = false]
        get 4 arrow;
        unequip arrow;
        get "古代箭"
        eat axe in slot 3;
        eat inf in slot 5
        pick-up 1 weapon;

        equip ice_arrow
        use bow; freeze meat

        use food 3 times
    "#};

    test_parser_snapshot("simple", script).await
}

#[tokio::test]
async fn parse_notes() -> anyhow::Result<()> {
    let script = indoc! {r#"
        '''note
        '''
        '''note
        a
        '''
        sync
        '''some-tag
        '''
        sync
        '''note
        text
        '''
        '''note
        text

        get  1 apple
        '''
        sync
        sync
        '''note
        empty line in between
        '''
        sync

        sync
        sync
        '''note
        test afterwards
        two lines
        '''
    "#};

    test_parser_snapshot("notes", script).await
}

struct StubQuotedItemResolver;
impl QuotedItemResolver for StubQuotedItemResolver {
    type Future = std::future::Ready<Option<ResolvedItem>>;

    fn resolve_quoted(&self, word: &str) -> Self::Future {
        std::future::ready(Some(ResolvedItem::new(word.to_string())))
    }
}

async fn test_parser_snapshot(path: &str, script: &str) -> anyhow::Result<()> {
    if !std::fs::exists("tests/parse")? {
        std::fs::create_dir_all("tests/parse")?;
    }
    let update = std::env::var("UPDATE_PARSER_SNAPSHOTS")
        .map(|s| s.trim() == "1")
        .unwrap_or(false);

    let script = normalize_newlines(script);

    let mock_suffix = if cfg!(feature = "mock-data") {
        "_mock"
    } else {
        ""
    };
    let lex_path = format!("tests/parse/{}{}.lex", path, mock_suffix);
    let syn_path = format!("tests/parse/{}{}.syn", path, mock_suffix);
    let cir_path = format!("tests/parse/{}{}.cir", path, mock_suffix);
    let sem_path = format!("tests/parse/{}{}.sem", path, mock_suffix);

    let lex_out = format!("{:#?}", skybook_parser::parse_tokens(&script));
    let syn_out = format!(
        "{:#?}",
        skybook_parser::syn::Script::parse(&script).unwrap()
    );
    let resolver = StubQuotedItemResolver;
    let cir_out = format!("{:#?}", skybook_parser::parse(&resolver, &script).await);
    let sem_out = format!(
        "{:#?}",
        skybook_parser::parse_semantic(&script, 0, script.len())
    );

    let mut errors = Vec::new();
    if let Err(e) = process_snapshot_file(&lex_path, &lex_out, update) {
        errors.push(e);
    }
    if let Err(e) = process_snapshot_file(&syn_path, &syn_out, update) {
        errors.push(e);
    }
    if let Err(e) = process_snapshot_file(&cir_path, &cir_out, update) {
        errors.push(e);
    }
    if let Err(e) = process_snapshot_file(&sem_path, &sem_out, update) {
        errors.push(e);
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("{}", e);
        }
        bail!("{} snapshot errors", errors.len());
    }

    Ok(())
}

fn process_snapshot_file(path: &str, content: &str, mut update: bool) -> anyhow::Result<()> {
    if !std::fs::exists(path)? {
        update = true;
    }
    if update {
        std::fs::write(path, content)?;
        return Ok(());
    }
    let current_content = normalize_newlines(std::fs::read_to_string(&path)?);
    let expected_content = normalize_newlines(content);
    if current_content != expected_content {
        let wip_path = path.replace("tests/parse/", "tests/parse/wip/");
        if !std::fs::exists("tests/parse/wip")? {
            std::fs::create_dir_all("tests/parse/wip")?;
        }
        std::fs::write(wip_path, content)?;
        bail!("Snapshot mismatch: {}", path);
    }

    Ok(())
}

fn normalize_newlines(s: impl AsRef<str>) -> String {
    s.as_ref().lines().collect::<Vec<_>>().join("\n")
}
