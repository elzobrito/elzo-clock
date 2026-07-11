# ECLOCK-050 — Configurar agenda Google no elzo-clock

## Objetivo
Alimentar o widget **Agenda Google** do `eclock` com eventos reais de `your@email.example`.

## Decisão técnica
| Backend | Status | Notas |
|---------|--------|--------|
| **gcalcli** (pipx) | Instalado e autenticado | Backend operacional confirmado em 2026-07-11 |
| **GNOME Online Accounts** | Conta presente; token **sem** scope calendar | CalendarEnabled=true, mas `GetAccessToken` só email/profile |
| MCP Google Calendar | Disponível no Grok, **não** no widget TUI | Só útil para o agente, não para `eclock` |

## Helper
`scripts/eclock-gcal` → `~/.local/bin/eclock-gcal`

Ordem:
1. `gcalcli list` ok → `gcalcli agenda`
2. senão GOA token + Calendar API (se scope permitir)
3. senão mensagem de setup (exit 0)

## Setup OAuth gcalcli

Os passos abaixo registram o procedimento histórico para reinstalação. Nesta máquina, a autenticação já foi concluída e `eclock-gcal` retorna eventos reais.

1. https://console.cloud.google.com/apis/library/calendar-json.googleapis.com → **Enable**
2. APIs & Services → **Credentials** → Create credentials → **OAuth client ID**
   - Application type: **Desktop app**
   - Copie Client ID e Client Secret
3. Se pedir tela de consentimento: adicione seu e-mail como test user
4. No terminal:

```bash
export PATH="$HOME/.local/bin:$PATH"
gcalcli init
# cole client_id e client_secret
# autorize sua conta Google no browser
gcalcli list
gcalcli agenda --nostarted
eclock-gcal
```

5. Reinicie `eclock` — widget **Agenda Google** usa `eclock-gcal`.

## Widget config

```toml
[[widgets]]
title = "Agenda Google"
command = "eclock-gcal"
refresh_secs = 300
timeout_secs = 25
```

## Critérios
- [x] gcalcli instalado (pipx)
- [x] helper eclock-gcal + diagnóstico GOA
- [x] docs e config.example
- [x] OAuth gcalcli concluído pelo operador e agenda real validada
