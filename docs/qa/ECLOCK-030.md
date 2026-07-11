# ECLOCK-030 — QA elzo-clock

## cargo build --release
- exit: **0**
- binary: `target/release/eclock` exists: **True**

## esaa verify (workspace)

> Snapshot histórico da execução original de ECLOCK-030. Para o estado atual, execute `esaa --root . verify`; as sequências avançam a cada evento governado.
```json
{
  "verify_status": "ok",
  "last_event_seq": 85,
  "projection_hash_sha256": "65b11f4589fe7ecc721878811fbca261e3632732ed5b3f5242dcf533c05f651c",
  "project_profile": true
}
```

## eclock-esaa .
```
root: .
cli:  esaa

verify: ok
seq:    85
profile:True

eligible: 0
parallel: 0
suppressed: 0
(nenhuma tarefa elegível)
```

## eclock-esaa esaa-teste
```
root: ~/path/to/esaa-workspace
cli:  esaa

verify: ok
seq:    44
profile:True

eligible: 0
parallel: 0
suppressed: 3
(nenhuma tarefa elegível)
suprimidas:
  · T-1000
  · T-1010
  · T-1020
```

## Instalacao local
- `~/.local/bin/eclock`
- `~/.local/bin/eclock-esaa`
- config: `~/.config/elzo-clock/config.toml` (widget ESAA)

## Conclusao
**PASS** se build 0, verify ok, eclock-esaa imprime verify/eligible.

## Revalidação atual — 2026-07-11

- `esaa --root . verify`: `ok` durante a execução governada desta atualização;
- `scripts/eclock-esaa .`: preserva verify/eligible e acrescenta estados e tarefas ativas;
- `cargo test --all-targets`: 10 testes aprovados;
- `cargo build --release`: aprovado.
