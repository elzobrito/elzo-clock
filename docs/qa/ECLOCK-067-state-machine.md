# ECLOCK-067 — QA da máquina de estados

## Verificações

```bash
python3 -m unittest scripts/test_eclock_esaa_state.py
scripts/eclock-esaa .
```

Os testes cobrem transições diretas, retorno por `request_changes`, tarefa sem eventos, bloqueio por dependência e cálculo de tempo no estado.

O teste integrado deve preservar a saída de `verify`, `eligible` e `suppressed`, acrescentando o resumo da máquina e as tarefas em `in_progress` ou `review`.
