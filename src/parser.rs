pub mod object {
    use anyhow::{anyhow, Context, Result};

    pub fn remove_double_quotes(code: &str) -> Result<&str> {
        Ok(code
            .strip_prefix('"')
            .context("テキストの始端にダブルクオーテーションがありません")?
            .strip_suffix('"')
            .context("テキストの始端にダブルクオーテーションがありません")?)
    }

    pub fn parse_kv(kv_text: &str, quoted: bool) -> Result<(&str, &str)> {
        let kvs = kv_text.split('=').collect::<Vec<&str>>();
        if kvs.len() != 2 {
            return Err(anyhow!("不正なパラメータ形式です: {}", kv_text));
        }
        return Ok(if quoted {
            (
                kvs[0],
                remove_double_quotes(kvs[1]).context(format!("{}", kvs[1]))?,
            )
        } else {
            (kvs[0], kvs[1])
        });
    }

    pub fn choose_kv_line<'a>(lines: &'a Vec<&str>, key: &'a str) -> Result<&'a str> {
        Ok(lines
            .into_iter()
            .find(|line| line.trim().starts_with(&format!("{}=", key)))
            .context(format!(
                "オブジェクト中にキーが発見できませんでした Key: {}",
                key
            ))?
            .trim())
    }

    pub fn choose_and_parse_kv<'a>(
        lines: &'a Vec<&str>,
        key: &'a str,
        quoted: bool,
    ) -> Result<(&'a str, &'a str)> {
        parse_kv(
            choose_kv_line(lines, key).context("key が示す行の取得に失敗しました")?,
            quoted,
        )
        .context("key / value の取得に失敗しました")
    }
}

pub mod meta {
    use uuid::Uuid;

    pub enum MetaInfo {
        String(String),
        Uuid(Uuid),
        Bool(bool),
    }

    pub fn parse() {}
}
