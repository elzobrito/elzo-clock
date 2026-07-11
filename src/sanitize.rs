//! Sanitização de saída de widgets: ANSI/C0 e redaction de segredos.

/// Remove sequências ANSI (CSI/OSC) e caracteres de controle C0 (exceto tab/newline).
pub fn strip_control_chars(value: &str) -> String {
    let mut clean = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            match chars.peek().copied() {
                Some('[') => {
                    // CSI: ESC [ ... letter
                    chars.next();
                    for code in chars.by_ref() {
                        if code.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC: ESC ] ... BEL or ESC \
                    chars.next();
                    while let Some(code) = chars.next() {
                        if code == '\u{7}' {
                            break;
                        }
                        if code == '\u{1b}' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    // drop lone ESC
                }
            }
            continue;
        }
        // Allow tab/newline; drop other C0 and DEL
        if ch == '\n' || ch == '\t' || ch == '\r' {
            clean.push(ch);
            continue;
        }
        if ch.is_control() {
            continue;
        }
        clean.push(ch);
    }
    clean
}

/// Redige padrões óbvios de segredos na saída de widgets.
pub fn redact_secrets(value: &str) -> String {
    let mut out = value.to_string();

    // Google OAuth access tokens
    out = replace_regexish(&out, r"ya29\.[A-Za-z0-9_\-]+", "[REDACTED]");
    // GitHub PATs
    out = replace_regexish(&out, r"ghp_[A-Za-z0-9]{20,}", "[REDACTED]");
    out = replace_regexish(&out, r"github_pat_[A-Za-z0-9_]{20,}", "[REDACTED]");
    // Google API keys
    out = replace_regexish(&out, r"AIza[0-9A-Za-z\-_]{20,}", "[REDACTED]");
    // OpenAI-like
    out = replace_regexish(&out, r"sk-[A-Za-z0-9]{20,}", "[REDACTED]");
    // Bearer tokens
    out = replace_regexish(
        &out,
        r"(?i)bearer\s+[A-Za-z0-9\-\._~\+\/]+=*",
        "Bearer [REDACTED]",
    );
    // JWT-ish (three base64 segments)
    out = replace_regexish(
        &out,
        r"\beyJ[A-Za-z0-9_\-]{10,}\.[A-Za-z0-9_\-]{10,}\.[A-Za-z0-9_\-]{10,}\b",
        "[REDACTED_JWT]",
    );
    // password= / client_secret=
    out = replace_regexish(
        &out,
        r"(?i)(password|client_secret|api_key|secret)\s*[:=]\s*\S+",
        "$1=[REDACTED]",
    );
    // PEM private keys (single-line collapse of begin..end)
    if out.contains("PRIVATE KEY-----") {
        out = redact_pem_blocks(&out);
    }
    out
}

fn redact_pem_blocks(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut in_pem = false;
    for line in value.lines() {
        if line.contains("BEGIN") && line.contains("PRIVATE KEY") {
            in_pem = true;
            result.push_str("[REDACTED_PRIVATE_KEY]\n");
            continue;
        }
        if in_pem {
            if line.contains("END") && line.contains("PRIVATE KEY") {
                in_pem = false;
            }
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    if !value.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }
    result
}

/// Minimal regex-like replacer using the `regex` crate would be ideal; keep zero new
/// deps with hand-rolled scanners for common prefixes + length heuristics.
fn replace_regexish(input: &str, pattern: &str, replacement: &str) -> String {
    // Hand-roll only the patterns we need without the regex crate for supply-chain simplicity.
    match pattern {
        r"ya29\.[A-Za-z0-9_\-]+" => redact_prefix_token(input, "ya29.", is_token_char, replacement),
        r"ghp_[A-Za-z0-9]{20,}" => {
            redact_prefix_token(input, "ghp_", |c| c.is_ascii_alphanumeric(), replacement)
        }
        r"github_pat_[A-Za-z0-9_]{20,}" => redact_prefix_token(
            input,
            "github_pat_",
            |c| c.is_ascii_alphanumeric() || c == '_',
            replacement,
        ),
        r"AIza[0-9A-Za-z\-_]{20,}" => {
            redact_prefix_token(input, "AIza", is_token_char, replacement)
        }
        r"sk-[A-Za-z0-9]{20,}" => {
            redact_prefix_token(input, "sk-", |c| c.is_ascii_alphanumeric(), replacement)
        }
        _ if pattern.starts_with("(?i)bearer") => redact_bearer(input),
        _ if pattern.starts_with(r"\beyJ") => redact_jwt(input),
        _ if pattern.starts_with("(?i)(password") => redact_assignments(input),
        _ => input.to_string(),
    }
}

fn is_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn redact_prefix_token(
    input: &str,
    prefix: &str,
    mut ok: impl FnMut(char) -> bool,
    replacement: &str,
) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() {
        if input[i..].starts_with(prefix) {
            let start = i;
            i += prefix.len();
            let body_start = i;
            while i < input.len() {
                let ch = input[i..].chars().next().unwrap();
                if ok(ch) {
                    i += ch.len_utf8();
                } else {
                    break;
                }
            }
            if i - body_start >= 8 {
                out.push_str(replacement);
            } else {
                out.push_str(&input[start..i]);
            }
        } else {
            let ch = input[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
        let _ = bytes; // silence
    }
    out
}

fn redact_bearer(input: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if lower[i..].starts_with("bearer ") || lower[i..].starts_with("bearer\t") {
            // copy original casing of "Bearer"
            let word_len = 6; // bearer
            out.push_str(&input[i..i + word_len]);
            i += word_len;
            while i < input.len() && input[i..].chars().next().unwrap().is_whitespace() {
                out.push(input[i..].chars().next().unwrap());
                i += 1;
            }
            // skip token
            while i < input.len() {
                let ch = input[i..].chars().next().unwrap();
                if ch.is_ascii_alphanumeric() || "-._~+/=".contains(ch) {
                    i += ch.len_utf8();
                } else {
                    break;
                }
            }
            out.push_str("[REDACTED]");
        } else {
            let ch = input[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn redact_jwt(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with("eyJ") {
            let start = i;
            let mut parts = 0;
            let mut j = i;
            while parts < 3 {
                let seg_start = j;
                while j < input.len() {
                    let ch = input[j..].chars().next().unwrap();
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        j += ch.len_utf8();
                    } else {
                        break;
                    }
                }
                if j == seg_start {
                    break;
                }
                parts += 1;
                if parts < 3 {
                    if j < input.len() && input[j..].starts_with('.') {
                        j += 1;
                    } else {
                        break;
                    }
                }
            }
            if parts == 3 && j - start > 30 {
                out.push_str("[REDACTED_JWT]");
                i = j;
                continue;
            }
        }
        let ch = input[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn redact_assignments(input: &str) -> String {
    let keys = ["password", "client_secret", "api_key", "secret"];
    let mut out = input.to_string();
    for key in keys {
        // case-insensitive simple scan
        let lower = out.to_ascii_lowercase();
        let mut idx = 0;
        let mut rebuilt = String::with_capacity(out.len());
        let original = out.clone();
        while let Some(pos) = lower[idx..].find(key) {
            let abs = idx + pos;
            rebuilt.push_str(&original[idx..abs]);
            rebuilt.push_str(&original[abs..abs + key.len()]);
            let mut j = abs + key.len();
            // skip spaces
            while j < original.len() && original[j..].chars().next().unwrap().is_whitespace() {
                rebuilt.push(original[j..].chars().next().unwrap());
                j += 1;
            }
            if j < original.len() {
                let sep = original[j..].chars().next().unwrap();
                if sep == '=' || sep == ':' {
                    rebuilt.push(sep);
                    j += sep.len_utf8();
                    while j < original.len()
                        && original[j..].chars().next().unwrap().is_whitespace()
                    {
                        j += 1;
                    }
                    // skip value
                    while j < original.len() {
                        let ch = original[j..].chars().next().unwrap();
                        if ch.is_whitespace() {
                            break;
                        }
                        j += ch.len_utf8();
                    }
                    rebuilt.push_str("[REDACTED]");
                    idx = j;
                    continue;
                }
            }
            idx = abs + key.len();
            rebuilt.push_str(&original[abs + key.len()..idx.min(original.len())]);
        }
        rebuilt.push_str(&original[idx..]);
        out = rebuilt;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_ansi_and_c0() {
        let raw = "\u{1b}[31mred\u{1b}[0m\u{7}ok\n";
        let clean = strip_control_chars(raw);
        assert_eq!(clean, "redok\n");
    }

    #[test]
    fn redacts_google_and_github_tokens() {
        let raw = "token ya29.a0ABCDEFGHIJKLMNOP password=supersecret ghp_abcdefghijklmnopqrstuvwxyz012345";
        let red = redact_secrets(raw);
        assert!(!red.contains("ya29.a0ABCDEFGHIJKLMNOP"));
        assert!(red.contains("[REDACTED]"));
        assert!(!red.contains("supersecret"));
        assert!(!red.contains("ghp_abcdefghijklmnopqrstuvwxyz012345"));
    }

    #[test]
    fn redacts_pem() {
        let raw = "-----BEGIN PRIVATE KEY-----\nABC\n-----END PRIVATE KEY-----\n";
        let red = redact_secrets(raw);
        assert!(red.contains("[REDACTED_PRIVATE_KEY]"));
        assert!(!red.contains("ABC"));
    }
}
