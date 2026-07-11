# Contrato operacional — elzo-clock

## Identidade
| Campo | Valor |
|-------|--------|
| Projeto | elzo-clock |
| Binary | eclock |
| Helper ESAA | eclock-esaa |
| Operador | Elzo |
| Domínio | terminal-tools |

## Fontes / outputs / protegidos
Ver `docs/spec/GOV-PROFILE-001.md` e `.roadmap/project_profile.json`.

## Comandos mínimos
```bash
cd ~/path/to/elzo-clock
python -m esaa --root . verify
python -m esaa --root . eligible
python -m esaa --root . profile show
eclock-esaa .
cargo build --release
eclock   # requer terminal interativo
```

## Widget ESAA no relógio
`~/.config/elzo-clock/config.toml` deve chamar `eclock-esaa` (ou path absoluto do projeto).
