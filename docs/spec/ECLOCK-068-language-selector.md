# ECLOCK-068 — Seletor de idioma

## Configuração

O campo `clock.language` controla o idioma da data por extenso:

```toml
[clock]
language = "pt-BR" # ou "en"
```

`pt-BR` é o padrão, inclusive para configurações antigas que não possuem o campo. Valores desconhecidos também usam `pt-BR` como fallback determinístico.

## Formatos

- `pt-BR`: `sábado, 11 de julho de 2026`
- `en`: `Saturday, 11 July 2026`

A hora permanece no formato de 24 horas e não depende do idioma escolhido.
