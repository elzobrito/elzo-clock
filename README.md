# elzo-clock (`eclock`)

Dashboard de relógio no terminal, **seu** — inspirado na ideia do [clock-tui / tclock](https://github.com/akitaonrails/clock-tui) (relógio grande + painéis de status), escrito do zero em Rust com [ratatui](https://github.com/ratatui/ratatui).

Não é fork do código do Akita; é um projeto pessoal enxuto.

## Funcionalidades (v0.1)

- Relógio digital grande (escala 1–3)
- Data opcional
- Idioma da data configurável (`pt-BR` padrão ou `en`)
- Até 3 **widgets** assíncronos (comandos shell finitos sem bloquear a TUI)
- **Widget ESAA** (`eclock-esaa`): `verify`, `eligible` e acompanhamento da máquina de estados
- Config TOML em `~/.config/elzo-clock/config.toml` (criada na 1ª execução)
- Tema **akita** (cores estilo tclock: dígitos sage, títulos laranja, acentos roxo/azul)
- Atalhos: `q` sair · `r` refresh · `s` segundos · `d` data · `+/-` tamanho

## Build

```bash
cd ~/path/to/elzo-clock
cargo build --release
# binário:
./target/release/eclock
```

Instalar no PATH do usuário:

```bash
cargo install --path .
# ou
cp target/release/eclock ~/.local/bin/
```

## Uso

```bash
eclock
```

### Widget Agenda Google

```bash
# já: gcalcli via pipx + eclock-gcal
eclock-gcal          # agenda ou instruções de OAuth
# OAuth (uma vez, no browser):
#   1) Enable Calendar API + OAuth Desktop client no Google Cloud
#   2) gcalcli init
# ver docs/spec/ECLOCK-050-google-calendar.md
```

No `config.toml`:

```toml
[[widgets]]
title = "Agenda Google"
command_argv = ["eclock-gcal"]
refresh_secs = 300
```

### Widget ESAA

Helper instalável:

```bash
cp scripts/eclock-esaa scripts/eclock_esaa_state.py ~/.local/bin/
chmod +x ~/.local/bin/eclock-esaa ~/.local/bin/eclock_esaa_state.py
eclock-esaa                              # auto-detecta .roadmap
eclock-esaa ~/path/to/esaa-workspace # root fixo
export ECLOCK_ESAA_ROOT=~/path/to/esaa-core
```

No `config.toml`:

```toml
[[widgets]]
title = "ESAA"
command_argv = ["eclock-esaa"]
# ou: command = "eclock-esaa ~/path/to/esaa-workspace"
refresh_secs = 10
timeout_secs = 20
```

Mostra: root, `verify_status`, `last_event_seq`, `project_profile`, elegíveis/suprimidas e a máquina `todo → in_progress → review → done`. Tarefas ativas exibem responsável, tempo no estado e última transição do ledger.

Refresh padrão do painel ESAA: **10 segundos** (`refresh_secs = 10`).

### Config padrão (widgets)

Na primeira execução o app grava um `config.toml` com exemplos:

| Widget | Ideia |
|--------|--------|
| **ESAA** | `eclock-esaa` → verify + eligible |
| Agenda | `gcalcli` / `khal` se existirem; senão instruções |
| Sistema | hostname, uptime, load |

Para imitar o print do Akita, configure widgets assim (precisa das ferramentas instaladas e autenticadas):

```toml
[clock]
show_date = true
show_seconds = true
size = 2
color = "akita"   # ou: tclock, cyan, green, magenta, yellow, blue, white
language = "pt-BR" # ou: en

[[widgets]]
title = "GitHub"
command = "gh search prs --state=open --owner=@me --limit 10 2>/dev/null || echo 'instale gh e autentique'"
refresh_secs = 900

[[widgets]]
title = "Google Calendar"
command = "gcalcli agenda --nostarted --nocolor 2>/dev/null | head -n 25 || echo 'instale gcalcli'"
refresh_secs = 3600

[[widgets]]
title = "Sistema"
command = "printf 'uptime: %s\\n' \"$(uptime -p)\"; free -h | head -n 2"
refresh_secs = 60
```

Widgets devem ser **comandos finitos** (snapshot e exit). TUIs em tela cheia não funcionam bem como widget.

## Atalhos

| Tecla | Ação |
|-------|------|
| `q` / `Esc` | Sair |
| `r` | Forçar refresh dos widgets |
| `s` | Mostrar/ocultar segundos |
| `d` | Mostrar/ocultar data |
| `+` / `-` | Aumentar/diminuir dígitos |

## Roadmap possível

- [ ] Mais de 3 widgets / layout bottom strip
- [ ] Temas e milissegundos
- [ ] Timer / stopwatch
- [ ] Scroll no widget com mouse
- [x] Integração ESAA (verify, eligible e acompanhamento ao vivo da máquina de estados)


# elzo-clock (`eclock`)

Dashboard de relógio no terminal, **seu** — inspirado na ideia do [clock-tui / tclock](https://github.com/akitaonrails/clock-tui) (relógio grande + painéis de status), escrito do zero em Rust com [ratatui](https://github.com/ratatui/ratatui).

Não é fork do código do Akita; é um projeto pessoal enxuto.

## Funcionalidades (v0.1)

- Relógio digital grande (escala 1–3)
- Data opcional
- Idioma da data configurável (`pt-BR` padrão ou `en`)
- Até 3 **widgets** assíncronos (comandos shell finitos sem bloquear a TUI)
- **Widget ESAA** (`eclock-esaa`): `verify`, `eligible` e acompanhamento da máquina de estados
- Config TOML em `~/.config/elzo-clock/config.toml` (criada na 1ª execução)
- Tema **akita** (cores estilo tclock: dígitos sage, títulos laranja, acentos roxo/azul)
- Atalhos: `q` sair · `r` refresh · `s` segundos · `d` data · `+/-` tamanho

## Build

```bash
cd ~/path/to/elzo-clock
cargo build --release
# binário:
./target/release/eclock
```

Instalar no PATH do usuário:

```bash
cargo install --path .
# ou
cp target/release/eclock ~/.local/bin/
```

## Uso

```bash
eclock
```

### Widget Agenda Google

```bash
# já: gcalcli via pipx + eclock-gcal
eclock-gcal          # agenda ou instruções de OAuth
# OAuth (uma vez, no browser):
#   1) Enable Calendar API + OAuth Desktop client no Google Cloud
#   2) gcalcli init
# ver docs/spec/ECLOCK-050-google-calendar.md
```

No `config.toml`:

```toml
[[widgets]]
title = "Agenda Google"
command_argv = ["eclock-gcal"]
refresh_secs = 300
```

### Widget ESAA

Helper instalável:

```bash
cp scripts/eclock-esaa scripts/eclock_esaa_state.py ~/.local/bin/
chmod +x ~/.local/bin/eclock-esaa ~/.local/bin/eclock_esaa_state.py
eclock-esaa                              # auto-detecta .roadmap
eclock-esaa ~/path/to/esaa-workspace # root fixo
export ECLOCK_ESAA_ROOT=~/path/to/esaa-core
```

No `config.toml`:

```toml
[[widgets]]
title = "ESAA"
command_argv = ["eclock-esaa"]
# ou: command = "eclock-esaa ~/path/to/esaa-workspace"
refresh_secs = 10
timeout_secs = 20
```

Mostra: root, `verify_status`, `last_event_seq`, `project_profile`, elegíveis/suprimidas e a máquina `todo → in_progress → review → done`. Tarefas ativas exibem responsável, tempo no estado e última transição do ledger.

Refresh padrão do painel ESAA: **10 segundos** (`refresh_secs = 10`).

### Config padrão (widgets)

Na primeira execução o app grava um `config.toml` com exemplos:

| Widget | Ideia |
|--------|--------|
| **ESAA** | `eclock-esaa` → verify + eligible |
| Agenda | `gcalcli` / `khal` se existirem; senão instruções |
| Sistema | hostname, uptime, load |

Para imitar o print do Akita, configure widgets assim (precisa das ferramentas instaladas e autenticadas):

```toml
[clock]
show_date = true
show_seconds = true
size = 2
color = "akita"   # ou: tclock, cyan, green, magenta, yellow, blue, white
language = "pt-BR" # ou: en

[[widgets]]
title = "GitHub"
command = "gh search prs --state=open --owner=@me --limit 10 2>/dev/null || echo 'instale gh e autentique'"
refresh_secs = 900

[[widgets]]
title = "Google Calendar"
command = "gcalcli agenda --nostarted --nocolor 2>/dev/null | head -n 25 || echo 'instale gcalcli'"
refresh_secs = 3600

[[widgets]]
title = "Sistema"
command = "printf 'uptime: %s\\n' \"$(uptime -p)\"; free -h | head -n 2"
refresh_secs = 60
```

Widgets devem ser **comandos finitos** (snapshot e exit). TUIs em tela cheia não funcionam bem como widget.

## Atalhos

| Tecla | Ação |
|-------|------|
| `q` / `Esc` | Sair |
| `r` | Forçar refresh dos widgets |
| `s` | Mostrar/ocultar segundos |
| `d` | Mostrar/ocultar data |
| `+` / `-` | Aumentar/diminuir dígitos |

## Roadmap possível

- [ ] Mais de 3 widgets / layout bottom strip
- [ ] Temas e milissegundos
- [ ] Timer / stopwatch
- [ ] Scroll no widget com mouse
- [x] Integração ESAA (verify, eligible e acompanhamento ao vivo da máquina de estados)


## Segurança e secrets

### Modelo de segurança dos widgets

```toml
[security]
allow_shell_command = true   # shell legado via `command`
# widget_allowlist = ["eclock-esaa", "eclock-gcal"]
redact_widget_output = true
privacy_mode = false         # true mascara títulos da agenda

[[widgets]]
title = "ESAA"
command_argv = ["eclock-esaa"]  # preferido (sem sh -c)
```

Riscos residuais: `docs/security/residual-risks.md`. Spec: `docs/spec/ECLOCK-SEC-REMEDIATION.md`.


- Nunca commite `.env`, `.env.*`, client secrets OAuth ou tokens.
- Tokens do **gcalcli** ficam em `$HOME` (tipicamente sob `~/.gcalcli*` / cache do Google); use `chmod 600` nos arquivos sensíveis.
- Widgets rodam comandos locais; prefira binários confiáveis (`eclock-esaa`, `eclock-gcal`). Ver `docs/spec/ECLOCK-SEC-REMEDIATION.md`.

## Licença

MIT — use e adapte como quiser.


- Nunca commite `.env`, `.env.*`, client secrets OAuth ou tokens.
- Tokens do **gcalcli** ficam em `$HOME` (tipicamente sob `~/.gcalcli*` / cache do Google); use `chmod 600` nos arquivos sensíveis.
- Widgets rodam comandos locais; prefira binários confiáveis (`eclock-esaa`, `eclock-gcal`). Ver `docs/spec/ECLOCK-SEC-REMEDIATION.md`.

## Licença

MIT — use e adapte como quiser.
