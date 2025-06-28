# Creator CLI v2.0 - Guia de Uso Completo

## 📚 Índice

- [Introdução](#introdução)
- [Conceitos Principais](#conceitos-principais)
- [Configuração](#configuração)
- [Comandos e Exemplos](#comandos-e-exemplos)
- [Casos de Uso Práticos](#casos-de-uso-práticos)
- [Presets Prontos](#presets-prontos)
- [Personalização](#personalização)
- [Fluxos de Trabalho](#fluxos-de-trabalho)
- [Solução de Problemas](#solução-de-problemas)

## Introdução

A Creator CLI v2.0 representa uma reescrita completa focada em **flexibilidade total**. Diferentemente da v1 com comandos fixos, a v2.0 é **100% configuration-driven**, permitindo criar qualquer estrutura de projeto via JSON.

### 🎯 Características Principais

- **Zero Hardcoding**: Todos os comandos são gerados dinamicamente da configuração
- **Categorias Flexíveis**: Suporte a itens estáticos, dinâmicos ou híbridos
- **Auto-Discovery**: Detecção automática de configs e diretórios
- **Interface Rica**: CLI interativa com navegação hierárquica
- **Sistema de Presets**: Configurações prontas para arquiteturas populares
- **Performance**: <100ms de startup com validação nativa

## Conceitos Principais

### Hierarquia de Organização

```
Projeto
├── Categoria 1 (ex: features)
│   ├── Item Tipo A (ex: modules)
│   ├── Item Tipo B (ex: services)
│   └── Item Tipo C (ex: hooks)
├── Categoria 2 (ex: infra)
│   ├── Item Tipo D (ex: clients)
│   └── Item Tipo E (ex: providers)
└── ...
```

### Tipos de Categoria

#### 🔒 **Static** - Itens Pré-definidos

Estrutura fixa com itens conhecidos.

```json
{
  "infra": {
    "description": "External configurations and integrations",
    "children": {
      "clients": {
        "template": "templates/default.hbs",
        "file_extension": "ts"
      },
      "providers": {
        "template": "templates/default.hbs",
        "file_extension": "ts"
      }
    }
  }
}
```

#### 🔄 **Dynamic** - Criação Runtime

Permite criar novos tipos de item durante execução.

```json
{
  "features": {
    "description": "Business features with dynamic creation support",
    "allow_dynamic_children": true,
    "default_structure": {
      "modules": {
        "template": "templates/components.hbs",
        "file_extension": "tsx"
      },
      "services": {
        "template": "templates/default.hbs",
        "file_extension": "ts"
      }
    }
  }
}
```

#### ⚡ **Mixed** - Híbrido

Combina itens estáticos + capacidade dinâmica.

```json
{
  "external": {
    "description": "External integrations and APIs",
    "children": {
      "apis": {
        "template": "templates/default.hbs",
        "file_extension": "ts"
      }
    },
    "allow_dynamic_children": true,
    "default_structure": {
      "client": {
        "template": "templates/default.hbs",
        "file_extension": "ts"
      }
    }
  }
}
```

## Configuração

### Auto-Discovery System

A CLI procura automaticamente por:

**Configs** (prioridade):

1. `config.json`
2. `config-clean-architecture.json`
3. `config-module-based.json`

**Source Directories** (prioridade):

1. `src/`
2. `app/`
3. `lib/`

### Override Manual

```bash
# Config específica
creator -c config-clean-architecture.json list

# Source dir específico
creator -s app/ create

# Ambos combinados
creator -c config-module-based.json -s lib/ list
```

### Schema Completo

```json
{
  "project": {
    "name": "my-project-name",
    "version": "2.0",
    "structure": {
      "categoria-exemplo": {
        "description": "Descrição opcional da categoria",

        // Para categorias STATIC ou MIXED
        "children": {
          "item-estatico": {
            "template": "caminho/para/template.hbs",
            "file_extension": "ts|tsx|js|jsx"
          }
        },

        // Para categorias DYNAMIC ou MIXED
        "allow_dynamic_children": true,
        "default_structure": {
          "item-dinamico": {
            "template": "caminho/para/template.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

## Comandos e Exemplos

### 🎮 Modo Interativo

```bash
creator
```

**Fluxo interativo:**

1. Escolher ação (Create, List, Exit)
2. Selecionar categoria
3. Escolher tipo de item (estático ou "Create new dynamic")
4. Inserir nome
5. Criação automática

### 📋 Listar Estrutura

```bash
# Todas as categorias
creator list

# Categoria específica
creator list -c features

# Com config customizada
creator -c config-clean-architecture.json list -c infra
```

**Output exemplo:**

```
📋 Available categories in 'my-react-native-app':

📁 infra
   External configurations and integrations
   Static items: clients, providers, config

📁 features
   Business features with dynamic creation support
   Dynamic types: modules, services, hooks

📁 pages
   Application pages/screens
   Static items: dashboard, login, profile

📁 core
   Core utilities and shared code
   Static items: types, utils, hooks
```

### 🏗️ Criar Itens

#### Criação Direta

```bash
# Item estático
creator create -c infra -i providers -n DatabaseProvider

# Item dinâmico
creator create -c features -i modules -n UserAuthentication

# Com config específica
creator -c config-module-based.json create -c modules -i containers -n ProductCatalog
```

#### Estrutura Resultante

Para `creator create -c features -i modules -n UserAuthentication`:

```
src/
└── features/
    └── user-authentication/
        └── modules/
            └── index.tsx
```

**Conteúdo gerado (`index.tsx`):**

```tsx
import { useState, useEffect } from "react";

export function UserAuthentication() {}
```

### 🚀 Inicializar Projeto

```bash
# Com preset específico
creator init -p clean-architecture
creator init -p module-based

# Lista presets disponíveis
creator init
```

## Casos de Uso Práticos

### 💼 Caso 1: Feature E-commerce Completa

**Objetivo:** Criar sistema de carrinho de compras com todas as camadas.

```bash
# 1. Feature principal
creator create -c features -i modules -n ShoppingCart

# 2. Serviços de negócio
creator create -c features -i services -n CartService
creator create -c features -i services -n PaymentService

# 3. Hooks customizados
creator create -c features -i hooks -n useCartState
creator create -c features -i hooks -n usePaymentFlow

# 4. Páginas relacionadas
creator create -c pages -i dashboard -n CartSummary
creator create -c pages -i login -n CheckoutPage
```

**Estrutura final:**

```
src/
├── features/
│   ├── shopping-cart/modules/index.tsx
│   ├── cart-service/services/index.ts
│   ├── payment-service/services/index.ts
│   ├── use-cart-state/hooks/index.ts
│   └── use-payment-flow/hooks/index.ts
└── pages/
    ├── cart-summary/dashboard/index.tsx
    └── checkout-page/login/index.tsx
```

### 🔧 Caso 2: Infraestrutura de APIs

**Objetivo:** Configurar clients e providers para múltiplos serviços.

```bash
# API Clients
creator create -c infra -i clients -n UserApiClient
creator create -c infra -i clients -n PaymentApiClient
creator create -c infra -i clients -n NotificationApiClient
creator create -c infra -i clients -n AnalyticsApiClient

# Configuration Providers
creator create -c infra -i providers -n DatabaseProvider
creator create -c infra -i providers -n CacheProvider
creator create -c infra -i config -n ApiEndpoints
creator create -c infra -i config -n EnvironmentVariables
```

### 🏗️ Caso 3: Arquitetura Modular

**Usando preset `module-based`:**

```bash
# 1. Inicializar com preset
creator init -p module-based

# 2. Módulo de autenticação completo
creator create -c modules -i containers -n Authentication
creator create -c modules -i components -n LoginForm
creator create -c modules -i components -n SignupForm
creator create -c modules -i services -n AuthService
creator create -c modules -i types -n AuthTypes

# 3. Módulo de produtos
creator create -c modules -i containers -n ProductCatalog
creator create -c modules -i components -n ProductCard
creator create -c modules -i services -n ProductService

# 4. Utilitários compartilhados
creator create -c shared -i utils -n ValidationHelpers
creator create -c shared -i hooks -n useFormValidation
creator create -c shared -i components -n LoadingSpinner
```

## Presets Prontos

### 🏛️ Clean Architecture Preset

**Arquivo:** `config-clean-architecture.json`

**Estrutura:**

- **`infra/`** - Configurações externas (static)
- **`features/`** - Features de negócio (dynamic)
- **`pages/`** - Páginas/telas (static)
- **`core/`** - Utilitários compartilhados (static)

**Ideal para:** Projetos seguindo Clean Architecture, DDD, ou separação clara de responsabilidades.

**Uso:**

```bash
creator init -p clean-architecture
creator -c config-clean-architecture.json list
```

### 📦 Module-Based Preset

**Arquivo:** `config-module-based.json`

**Estrutura:**

- **`application/`** - Camada principal (static)
- **`modules/`** - Módulos de negócio (dynamic)
- **`shared/`** - Componentes compartilhados (static)
- **`external/`** - Integrações externas (mixed)

**Ideal para:** Projetos modulares, micro-frontends, ou arquiteturas orientadas a módulos.

**Uso:**

```bash
creator init -p module-based
creator -c config-module-based.json create -c modules -i containers -n UserProfile
```

## Personalização

### 📄 Templates Disponíveis

#### Default (`templates/default.hbs`)

```typescript
export function {{templateName}}(){}
```

#### Components (`templates/components.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function {{templateName}}(){}
```

#### Hooks (`templates/hooks.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function use{{templateName}}(){}
```

### 🎨 Criando Templates Personalizados

**1. Criar template customizado:**

```handlebars
{{!-- templates/service.hbs --}}
import { Injectable } from '@nestjs/common';

@Injectable()
export class {{templateName}}Service {
  constructor() {
    // TODO: Inject dependencies
  }

  async execute(): Promise<void> {
    // TODO: Implement business logic
  }

  async findAll(): Promise<any[]> {
    // TODO: Implement query logic
    return [];
  }
}
```

**2. Usar no config:**

```json
{
  "backend-services": {
    "description": "NestJS backend services",
    "children": {
      "crud-service": {
        "template": "templates/service.hbs",
        "file_extension": "ts"
      }
    }
  }
}
```

**3. Usar:**

```bash
creator create -c backend-services -i crud-service -n UserService
```

### 🔧 Variáveis de Template

- **`{{templateName}}`** - Nome em PascalCase
- **Arquivo sempre:** `index.{extensão}`
- **Estrutura:** `categoria/kebab-case-name/tipo-item/index.ext`

**Exemplo:** `creator create -c features -i services -n UserAuth`

```
src/features/user-auth/services/index.ts
```

## Fluxos de Trabalho

### 🆕 Projetos Novos

```bash
# 1. Escolher arquitetura e inicializar
creator init -p clean-architecture

# 2. Customizar config.json se necessário
# 3. Criar estrutura base
creator create -c core -i types -n GlobalTypes
creator create -c core -i utils -n ApiHelpers

# 4. Desenvolver features
creator create -c features -i modules -n UserManagement
creator create -c features -i services -n UserService
```

### 🔄 Projetos Existentes

```bash
# 1. Analisar estrutura atual
ls -la src/

# 2. Criar config.json personalizada baseada na estrutura
# 3. Testar com categoria simples
creator create -c test -i simple -n TestComponent

# 4. Migrar gradualmente
creator list
creator create -c existing-category -i existing-type -n NewItem
```

### 👥 Trabalho em Equipe

**Setup inicial:**

```bash
# 1. Versionar config no repo
git add config.json config-clean-architecture.json
git commit -m "Add Creator CLI configurations"

# 2. Documentar convenções no README
# 3. Treinar equipe com presets
creator init -p clean-architecture
creator list

# 4. Validação regular
creator list  # Para verificar estrutura atual
```

**Convenções recomendadas:**

- Config versionada no repo
- Usar sempre `-c categoria -i tipo -n Nome` para clareza
- Prefixar nomes com contexto: `UserLoginForm`, `PaymentApiClient`
- Validar com `creator list` antes de commits grandes

## Solução de Problemas

### ❌ Problemas Comuns

#### 1. **Config não encontrada**

```
Error: Config file not found at path: config.json
```

**Soluções:**

```bash
# Verificar arquivos disponíveis
ls -la *.json

# Usar config específica
creator -c config-clean-architecture.json list

# Inicializar nova config
creator init -p clean-architecture
```

#### 2. **Categoria inexistente**

```
Error: Category 'feature' not found
```

**Soluções:**

```bash
# Listar categorias disponíveis
creator list

# Verificar nome exato (case-sensitive)
creator list | grep -i feature

# Checar config atual
cat config.json | grep -A 5 "structure"
```

#### 3. **Template não encontrado**

```
Error: Template file not found: templates/custom.hbs
```

**Soluções:**

```bash
# Verificar templates disponíveis
ls -la templates/

# Usar caminho absoluto ou relativo correto
creator create -c test -i item -n Test
# (verifique se o template está em templates/default.hbs)

# Verificar permissões
chmod 644 templates/*.hbs
```

#### 4. **Nome inválido**

```
Error: Name can only contain alphanumeric characters, underscore, and dash
```

**✅ Nomes válidos:**

- `UserService`
- `user-service`
- `user_service`
- `api-client-v2`

**❌ Nomes inválidos:**

- `User Service` (espaço)
- `user@service` (caracteres especiais)
- `user.service` (ponto)

### 🔍 Debug e Validação

```bash
# Verificar config carregada
creator list

# Testar categoria específica
creator list -c features

# Validar estrutura de projeto
find src/ -type f -name "*.ts*" | head -10

# Verificar templates
ls -la templates/ && cat templates/default.hbs
```

### ⚡ Performance Tips

1. **Configs menores = startup mais rápido**
2. **Templates simples = geração mais rápida**
3. **Cache automático** durante sessão da CLI
4. **Auto-discovery** eficiente para projetos padrão

### ✅ Validação Automática

A CLI valida automaticamente:

- ✅ JSON syntax válida
- ✅ Campos obrigatórios (`project.name`, `project.version`)
- ✅ Templates existem no filesystem
- ✅ Extensões de arquivo válidas
- ✅ Estrutura de categorias consistente
- ✅ Referências circulares

---

## 🎯 Próximos Passos

**Para começar agora:**

1. **Teste o modo interativo:**

   ```bash
   creator
   ```

2. **Explore os presets:**

   ```bash
   creator init
   ```

3. **Customize para seu projeto:**

   - Edite `config.json`
   - Crie templates personalizados
   - Teste com `creator list`

4. **Integre ao workflow da equipe:**
   - Versione configurações
   - Documente convenções
   - Treine outros desenvolvedores

**Recursos adicionais:**

- 📖 [README.md](../README.md) - Documentação técnica
- 🐛 [GitHub Issues](https://github.com/andraderaul/creator/issues) - Reportar problemas
- 💡 [Discussions](https://github.com/andraderaul/creator/discussions) - Ideias e feedback

---

_Creator CLI v2.0 - Flexibilidade total para estruturas de projeto_ 🚀
