# ECLOCK-070 — Painel operacional no widget ESAA

## Objetivo

Apresentar no terminal o mesmo recorte de acompanhamento do painel operacional do `esaa-esd`, adequado ao espaço compacto do `eclock`.

## Conteúdo

- contadores de `todo`, `in_progress`, `review`, `done` e bloqueadas;
- linhas `I:` (`in_progress`) com ID, tempo no status, responsável e título;
- linhas `R:` (`review`) com ID, tempo no status, responsável e título;
- mensagem explícita quando uma seção ativa está vazia.

O tempo no status é calculado desde a transição que colocou a tarefa no estado atual. As tarefas `todo` e `done` aparecem nos contadores, como nos cards do ESD; os detalhes ficam concentrados nos estados que exigem acompanhamento operacional.

O bloco operacional é emitido antes de `root`, `verify` e `eligible`, garantindo que permaneça visível mesmo em terminais baixos.
As siglas `I` e `R` mantêm as tarefas ativas em uma linha compacta; o cabeçalho associa essas siglas aos contadores `IN_PROGRESS` e `REVIEW`.

## Fora do widget

O feed `Ledger — últimos eventos`, IDs de evento e a última ação não são exibidos. O ledger continua sendo a fonte de verdade usada internamente para derivar estado e tempo, sem virar conteúdo visual.
