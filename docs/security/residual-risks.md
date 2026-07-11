# Riscos residuais — elzo-clock

Produto: TUI desktop **single-user**. Não há servidor HTTP nem multi-tenant.

## Accepted risk

### AZ-002 — OS user trust
Quem controla a conta Unix controla `~/.config/elzo-clock`, tokens gcalcli e a execução dos widgets.
**Mitigações:** `chmod 600/700` na config; preferir `command_argv`; `widget_allowlist` opcional; não executar `eclock` com privilégios elevados.

### IF-002 — Host hardening
Atualizações do SO, full-disk encryption e permissões de `$HOME` são responsabilidade do operador.
**Recomendações:** disk encryption (LUKS), updates regulares, não compartilhar sessão gráfica com contas não confiáveis.

## Residual após mitigação

| Finding | Residual |
|---------|----------|
| IV-002 | Shell legado ainda disponível se `allow_shell_command=true` e config adversária |
| LG-002 | Redaction heurística — não é DLP completo |
| DT-001 | Sem encriptação de config com senha mestre (usa perms OS) |
| DT-002 | `privacy_mode=false` por default (UX); PII visível se flag off |

## Fora de escopo
seccomp, Flatpak, crypto de config, multi-user RBAC.
