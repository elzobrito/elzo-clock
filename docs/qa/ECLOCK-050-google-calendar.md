# ECLOCK-050 — QA agenda Google

## Ambiente
- gcalcli: instalado via pipx (`gcalcli 4.5.1`)
- PATH: `~/.local/bin/gcalcli`, `~/.local/bin/eclock-gcal`
- GOA: `your@email.example`, CalendarEnabled=true
- GOA token scopes: email/profile/openid **sem calendar** (fallback não utilizado)
- gcalcli OAuth: autenticado e operacional

## Testes
```bash
eclock-gcal
# imprime agenda real via gcalcli
```

## Resultado
| Check | Status |
|-------|--------|
| gcalcli no PATH | PASS |
| eclock-gcal instalado | PASS |
| Widget config aponta eclock-gcal | PASS |
| Eventos reais no widget | PASS |

## Conclusão
**PASS** — em 2026-07-11, `eclock-gcal` retornou eventos reais da agenda por `gcalcli`.

O diagnóstico anterior de bloqueio OAuth era um snapshot histórico anterior à autenticação e não representa mais o estado operacional atual.
