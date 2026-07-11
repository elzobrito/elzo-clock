# ECLOCK-069 — Destaque do dia atual na Agenda

O widget reconhece títulos contendo `Agenda`, `Calendar` ou `Calendário`. Somente nesses painéis, a linha cuja data corresponde ao dia local atual recebe vermelho e negrito.

Formatos reconhecidos:

- `Sat Jul 11`, formato emitido pelo `gcalcli`;
- `2026-07-11`, formato ISO usado por integrações alternativas.

O helper `eclock-gcal` remove códigos ANSI antes de entregar o texto. A aplicação também normaliza esses códigos defensivamente para configurações antigas ou comandos personalizados.

A comparação usa `chrono::Local`, preservando o fuso horário da máquina. Datas anteriores e posteriores mantêm a cor genérica de agenda.
