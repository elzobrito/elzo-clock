# QA — remediação auditoria segurança elzo-clock

Data: 2026-07-11  
Base: security-audit elzo-clock-security-20260711  

## Checklist 9 findings

| ID | Status | Evidência |
|----|--------|-----------|
| IV-002 | **mitigated** | `command_argv` + allowlist + shell opt-in; testes `argv_runs_*`, `allowlist_*`, `shell_blocked_*` |
| SC-004 | **fixed** | `.gitignore` com `.env` / `.env.*` |
| LG-002 | **mitigated** | `redact_widget_output` default true; `sanitize::redact_secrets` |
| FE-002 | **mitigated** | `strip_control_chars` em pipeline e render |
| DS-001 | **fixed** | `.github/workflows/ci.yml` |
| DT-001 | **mitigated** | perms 700/600 + residual FDE em residual-risks |
| DT-002 | **mitigated** | `privacy_mode` + residual UX |
| AZ-002 | **accepted-risk** | residual-risks.md |
| IF-002 | **accepted-risk** | residual-risks.md |

## Smoke

```bash
cargo test
# opcional: cargo audit; cargo clippy
```

Resultado esperado: testes unitários verdes (incl. sanitize, allowlist, perms).
