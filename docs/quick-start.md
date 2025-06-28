# Creator CLI v2.0 - Quick Start Guide 🚀

## 5 Minutos para Produtividade

Este guia te leva do zero à criação de estruturas em **menos de 5 minutos**.

## 📥 1. Instalação

```bash
# Download do binário (veja releases no GitHub)
chmod +x creator
sudo mv creator /usr/local/bin/  # Linux/macOS
```

## 🎯 2. Primeiro Uso

### Modo Interativo (Recomendado)

```bash
creator
```

Navegue pelas opções:

- ✅ **Create new item** → Escolha categoria → Tipo → Nome
- ✅ **List structure** → Veja configuração atual
- ✅ **Exit** → Sair

### Listar Estrutura Disponível

```bash
creator list
```

**Saída esperada:**

```
📋 Available categories in 'my-react-native-clean-app':

📁 infra
   External configurations and integrations
   Static items: clients, providers, config

📁 features
   Business features with dynamic creation support
   Dynamic types: modules, services, hooks
```

## 🏗️ 3. Criar Primeiro Item

```bash
# Comando direto
creator create -c features -i modules -n UserProfile

# Resultado: src/features/user-profile/modules/index.tsx
```

**Arquivo gerado:**

```tsx
import { useState, useEffect } from "react";

export function UserProfile() {}
```

## 🎨 4. Explorar Presets

```bash
# Inicializar com Clean Architecture
creator init -p clean-architecture

# Inicializar com Module-based
creator init -p module-based

# Ver presets disponíveis
creator init
```

## ⚡ 5. Comandos Essenciais

| Comando                                 | Função                    | Exemplo                                           |
| --------------------------------------- | ------------------------- | ------------------------------------------------- |
| `creator`                               | Modo interativo           | `creator`                                         |
| `creator list`                          | Listar categorias         | `creator list`                                    |
| `creator list -c features`              | Listar items de categoria | `creator list -c features`                        |
| `creator create -c CAT -i ITEM -n NAME` | Criar item direto         | `creator create -c infra -i clients -n ApiClient` |
| `creator init -p PRESET`                | Inicializar preset        | `creator init -p clean-architecture`              |
| `creator -c CONFIG.json list`           | Usar config específica    | `creator -c config-module-based.json list`        |

## 🧪 6. Teste Rápido

```bash
# 1. Listar estrutura
creator list

# 2. Criar teste
creator create -c features -i services -n TestService

# 3. Verificar resultado
ls -la src/features/test-service/services/

# 4. Ver conteúdo
cat src/features/test-service/services/index.ts
```

## 🔧 7. Troubleshooting Rápido

### ❌ "Config file not found"

```bash
# Verificar configs disponíveis
ls -la *.json

# Usar config específica
creator -c config-clean-architecture.json list
```

### ❌ "Category not found"

```bash
# Ver categorias disponíveis
creator list

# Verificar nome correto (case-sensitive)
creator list | grep -i categoria
```

### ❌ "Template not found"

```bash
# Verificar templates
ls -la templates/

# Usar config com templates válidas
creator -c config-clean-architecture.json create -c infra -i clients -n Test
```

## 📚 8. Próximos Passos

### Para uso básico:

- 📖 [Guia Completo](./cli-usage-guide.md) - Documentação detalhada
- 🔧 [Exemplos de Configuração](./configuration-examples.md) - Configs para diferentes arquiteturas

### Para customização:

- Editar `config.json` para seu projeto
- Criar templates personalizados em `templates/`
- Combinar elementos de diferentes presets

### Para times:

- Versionar `config.json` no repositório
- Documentar convenções específicas do projeto
- Treinar equipe com modo interativo

## 🎯 Casos de Uso Comuns

### Criar Feature Completa

```bash
creator create -c features -i modules -n UserAuth
creator create -c features -i services -n UserAuthService
creator create -c features -i hooks -n useUserAuth
```

### Setup de Infraestrutura

```bash
creator create -c infra -i clients -n ApiClient
creator create -c infra -i providers -n DatabaseProvider
creator create -c infra -i config -n AppConfig
```

### Páginas da Aplicação

```bash
creator create -c pages -i dashboard -n UserDashboard
creator create -c pages -i login -n LoginScreen
creator create -c pages -i profile -n UserProfile
```

---

## ✅ Checklist de Sucesso

Após seguir este guia, você deve conseguir:

- [ ] Executar `creator` e navegar no modo interativo
- [ ] Listar categorias com `creator list`
- [ ] Criar items com `creator create -c CATEGORIA -i TIPO -n NOME`
- [ ] Entender a estrutura de pastas gerada
- [ ] Usar presets com `creator init -p PRESET`
- [ ] Resolver problemas básicos de configuração

**🎉 Parabéns! Você está pronto para usar a Creator CLI v2.0 produtivamente!**

---

_Próximo passo: [Guia Completo](./cli-usage-guide.md) para funcionalidades avançadas_ 📖
