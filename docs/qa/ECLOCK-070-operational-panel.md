# ECLOCK-070 — QA do painel operacional

## Testes

```bash
python3 -m unittest scripts/test_eclock_esaa_state.py
scripts/eclock-esaa .
```

A suíte valida contadores, agrupamento de `in_progress` e `review`, responsável, tempo injetável, mensagens de seção vazia e ausência de `event_id`, `EV-*` e “último”.
