# ECLOCK-067 — Máquina de estados no widget ESAA

## Objetivo

O widget deve acompanhar a execução real das tarefas, tomando o ledger como fonte de verdade e mantendo `verify` e `eligible` como sinais complementares.

## Derivação

| Evento | Estado resultante |
|---|---|
| `task.create` | `todo` |
| `claim` | `in_progress` |
| `complete` | `review` |
| `review` aprovado | `done` |
| `review` com `request_changes` | `in_progress` |

O acompanhamento destaca tarefas em `in_progress` e `review`, com responsável, tempo no estado e última transição. Dependências ainda não concluídas são indicadas como bloqueio. Toda leitura é feita sobre `.roadmap/roadmap.json` e `.roadmap/activity.jsonl`, sem escrita direta.

## Referência

A semântica e o painel ao vivo seguem o comportamento observado no `esaa-esd`, adaptado ao espaço limitado da TUI.
