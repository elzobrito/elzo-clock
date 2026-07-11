# ECLOCK-SEC-REMEDIATION — Spec de remediação da auditoria de segurança

**Workspace:** elzo-clock  
**Fonte:** security-audit `elzo-clock-security-20260711`  
**Veredito audit:** aprovado com recomendações (9 partial, 0 fail)  
**Abordagem:** hardening pragmático (código + CI + residual documentado)

## 1. Objetivo

Endereçar os 9 findings partial da auditoria sem quebrar widgets oficiais (`eclock-esaa`, `eclock-gcal`) nem o modelo single-user desktop.

## 2. Mapa finding → mudança → aceite

| Finding | Sev | Classe | Mudança | Critério de aceite | Task |
|---------|-----|--------|---------|--------------------|------|
| IV-002 Shell widget | HIGH | A mitigado | `command_argv` sem shell; `allow_shell_command`; `widget_allowlist` opcional; defaults builtins em argv | Testes: argv sem `sh -c`; allowlist bloqueia; shell legado se permitido | ECLOCK-SEC-015 |
| SC-004 gitignore .env | MED | A fixed | `.env`, `.env.*` no `.gitignore` + nota README | Patterns presentes; doc OAuth em `$HOME` | ECLOCK-SEC-010 |
| LG-002 Widget stdout | MED | A mitigado | `redact_widget_output` default true; `sanitize::redact_secrets` pós-stdout | Tokens sintéticos mascarados; flag off funciona | ECLOCK-SEC-020 |
| FE-002 ANSI | MED | A mitigado | `strip_control_chars` (CSI/OSC/C0) em pipeline + render | Testes payload malformado; agenda colorize ok | ECLOCK-SEC-024 |
| DS-001 CI | MED | A fixed | `.github/workflows/ci.yml` (test, clippy, audit) | Workflow versionado; `cargo test` no job | ECLOCK-SEC-022 |
| DT-001 At-rest | MED | A mitigado + residual | `chmod` 700/600 ao criar/gravar config Unix | Arquivo novo mode 600; residual FDE em residual-risks | ECLOCK-SEC-023 + 090 |
| DT-002 PII UI | LOW | A mitigado + residual | `privacy_mode` mascara títulos de agenda | Flag documentada; default false | ECLOCK-SEC-023B + 090 |
| AZ-002 OS user trust | MED | B accepted | Docs residual + perms | Explicitado em residual-risks | ECLOCK-SEC-090 |
| IF-002 Host hardening | LOW | B accepted | Docs residual | Explicitado em residual-risks | ECLOCK-SEC-090 |

## 3. Residual aceito (classe B)

### AZ-002 — OS user trust
Quem controla a conta Unix do usuário controla config, tokens gcalcli e execução de widgets. Não há RBAC in-app. Mitigações: permissões 600/700; allowlist opcional; não rodar eclock com privilégios elevados.

### IF-002 — Host hardening
Atualizações do OS, full-disk encryption, firewall e permissões de `$HOME` são responsabilidade do operador. O app documenta recomendações; não impõe política de host.

### Residual DT-001 / DT-002
Sem criptografia de `config.toml` com senha mestre. Privacy mode não desliga a agenda por default (UX).

## 4. API de produto — SecurityConfig

```toml
[security]
allow_shell_command = true   # default: compat com `command` string
widget_allowlist = []        # vazio = desligado
redact_widget_output = true
privacy_mode = false
```

### WidgetConfig

- `command_argv: Option<Vec<String>>` — preferido; execução sem shell  
- `command: Option<String>` — legado `sh -c` se `allow_shell_command`  
- Pelo menos um de `command_argv` ou `command` deve existir (validação no load/run)

### Regras de execução

1. `command_argv` presente e não vazio → `Command::new(argv[0]).args(&argv[1..])` (+ timeout se disponível)  
2. Senão `command` + `allow_shell_command` → `sh -c`  
3. Senão → erro no painel  
4. Se `widget_allowlist` non-empty → validar basename do executável  
5. Pós-stdout: `strip_control_chars`; se redact → `redact_secrets`

### Defaults

| Widget | Preferência |
|--------|-------------|
| ESAA | `command_argv = ["eclock-esaa"]` |
| Agenda | `command_argv = ["eclock-gcal"]` |
| Sistema | script/argv se possível; senão shell legado documentado |

## 5. Módulos

| Módulo | Responsabilidade |
|--------|------------------|
| `src/sanitize.rs` | strip + redaction + testes |
| `src/config.rs` | SecurityConfig, parse, run_*, chmod |
| `src/widgets.rs` | repassar config de execução |
| `src/app.rs` | privacy_mode display; strip na render |

## 6. Ordem de implementação (ESAA)

001 → 010 → 015 → 020 → 024 → 023 → 022 → 023B → 090 → 091

`max_parallel=1`: serializar claims.

## 7. Fora de escopo

- seccomp/bubblewrap/Flatpak  
- crypto de config com senha mestre  
- re-auditoria completa do plugin (além de nota 091)  
- release crates.io  

## 8. Verificação

- Unit/integration em `cargo test`  
- `cargo audit` / clippy no CI  
- Checklist QA com os 9 IDs em `docs/qa/ECLOCK-SEC-REMEDIATION-QA.md`  
- `python -m esaa --root . verify` após cada complete  
