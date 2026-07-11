# GOV-PROFILE-001 — Contrato operacional elzo-clock

## Projeto
- **Nome:** elzo-clock (`eclock`)
- **Domínio:** terminal-tools
- **Operador:** Elzo
- **Idioma:** pt-BR

## Fontes de verdade
- `README.md` — visão e uso
- `Cargo.toml` / `Cargo.lock` — deps e versão
- `src/**` — implementação
- `docs/spec/**` — specs governadas
- `.roadmap/activity.jsonl` — histórico ESAA
- `.roadmap/project_profile.json` — perfil

## Paths protegidos
- `.roadmap/**`
- `target/**` (build)
- `Cargo.lock` (não editar à mão sem necessidade)

## Superfícies de saída
- `src/**`, `scripts/**`, `docs/**`, `README.md`

## Guias
- `AGENTS.md`, `.claude/CLAUDE.md` — preservados
- Bootstrap: preferir `--preserve-guides`

## Workflow
todo → claim → complete → review → done
`file_updates` só com complete. Runner: `--runner grok` (ou registrado).

## Comandos mínimos
```bash
cd ~/path/to/elzo-clock
python -m esaa --root . verify
python -m esaa --root . eligible
python -m esaa --root . profile show
eclock-esaa .
cargo build --release
```
