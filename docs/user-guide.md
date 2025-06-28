# Creator CLI v2.0 - Guia Completo de Uso

## Índice

- [Introdução](#introdução)
- [Conceitos Fundamentais](#conceitos-fundamentais)
- [Configuração](#configuração)
- [Comandos Disponíveis](#comandos-disponíveis)
- [Casos de Uso Práticos](#casos-de-uso-práticos)
- [Presets Disponíveis](#presets-disponíveis)
- [Templates e Personalização](#templates-e-personalização)
- [Fluxos de Trabalho Recomendados](#fluxos-de-trabalho-recomendados)
- [Troubleshooting](#troubleshooting)

## Introdução

A Creator CLI v2.0 é uma ferramenta completamente dinâmica para gerenciamento de estruturas de projetos. Diferentemente da v1 que tinha comandos hardcoded, a v2.0 é 100% baseada em configuração JSON, oferecendo flexibilidade total para definir qualquer estrutura de projeto.

### Características Principais

- **Sistema Dinâmico**: Nenhum comando hardcoded, tudo definido via configuração
- **Categorias Flexíveis**: Suporte a itens estáticos, dinâmicos ou mistos
- **Auto-Discovery**: Detecção automática de configurações e diretórios
- **Interface Interativa**: CLI intuitiva com navegação hierárquica
- **Sistema de Presets**: Configurações prontas para diferentes arquiteturas

## Conceitos Fundamentais

### Estrutura de Configuração

A Creator CLI trabalha com três conceitos principais:

1. **Projeto**: Informações gerais e estrutura de categorias
2. **Categorias**: Agrupamentos lógicos de itens (ex: features, core, infra)
3. **Itens**: Templates específicos para criação de arquivos/estruturas

### Tipos de Categorias

#### 1. **Estática (Static)**

Itens pré-definidos e fixos.

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

#### 2. **Dinâmica (Dynamic)**

Permite criação de novos itens em runtime.

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
      },
      "hooks": {
        "template": "templates/hooks.hbs",
        "file_extension": "ts"
      }
    }
  }
}
```

#### 3. **Mista (Mixed)**

Combina itens estáticos com suporte a criação dinâmica.

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

### Auto-Discovery

A Creator CLI automaticamente procura por:

1. **Arquivos de configuração** (na ordem):

   - `config.json`
   - `config-clean-architecture.json`
   - `config-module-based.json`

2. **Diretórios source** (na ordem):
   - `src/`
   - `app/`
   - `lib/`

### Especificando Configuração Manual

```bash
# Usar configuração específica
creator -c config-clean-architecture.json list

# Usar diretório source específico
creator -s app/ create
```

### Estrutura Completa de Configuração

```json
{
  "project": {
    "name": "my-project-name",
    "version": "2.0",
    "structure": {
      "category-name": {
        "description": "Opcional: descrição da categoria",
        "children": {
          "item-name": {
            "template": "path/to/template.hbs",
            "file_extension": "ts|tsx|js|jsx"
          }
        },
        "allow_dynamic_children": true,
        "default_structure": {
          "dynamic-item": {
            "template": "path/to/template.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

## Comandos Disponíveis

### 1. Modo Interativo

```bash
creator
```

Inicia o modo interativo onde você pode navegar pelas opções:

- Selecionar categoria
- Escolher tipo de item
- Definir nome
- Criar estrutura automaticamente

### 2. Listar Estrutura

```bash
# Listar todas as categorias
creator list

# Listar itens de uma categoria específica
creator list -c features

# Com configuração específica
creator -c config-clean-architecture.json list
```

**Saída esperada:**

```
📋 Available categories in 'my-project':

📁 infra
   External configurations and integrations
   Static items: clients, providers, config

📁 features
   Business features with dynamic creation support
   Dynamic types: modules, services, hooks

📁 pages
   Application pages/screens
   Static items: dashboard, login, profile
```

### 3. Criar Itens

#### Criação Direta (Non-Interactive)

```bash
# Criar item estático
creator create -c infra -i providers -n ApiProvider

# Criar item dinâmico
creator create -c features -i modules -n UserManagement

# Com configuração específica
creator -c config-module-based.json create -c modules -i containers -n UserProfile
```

#### Resultado da Criação

Para o comando `creator create -c features -i modules -n UserManagement`:

```
src/
└── features/
    └── user-management/
        └── modules/
            └── index.tsx
```

**Conteúdo do arquivo gerado:**

```tsx
import { useState, useEffect } from "react";

export function UserManagement() {}
```

### 4. Inicializar Projeto

```bash
# Inicializar com preset
creator init -p clean-architecture
creator init -p module-based

# Lista presets disponíveis
creator init
```

## Casos de Uso Práticos

### Caso 1: Desenvolvendo uma Feature de E-commerce

**Objetivo**: Criar uma feature completa de carrinho de compras

```bash
# 1. Criar a feature principal
creator create -c features -i modules -n ShoppingCart

# 2. Adicionar serviços
creator create -c features -i services -n CartService

# 3. Adicionar hooks personalizados
creator create -c features -i hooks -n useCartState

# 4. Adicionar páginas relacionadas
creator create -c pages -i dashboard -n CartSummary
```

**Estrutura resultante:**

```
src/
├── features/
│   ├── shopping-cart/
│   │   └── modules/
│   │       └── index.tsx
│   ├── cart-service/
│   │   └── services/
│   │       └── index.ts
│   └── use-cart-state/
│       └── hooks/
│           └── index.ts
└── pages/
    └── cart-summary/
        └── dashboard/
            └── index.tsx
```

### Caso 2: Configurando Infraestrutura de API

**Objetivo**: Criar clients e providers para diferentes serviços

```bash
# Clients para diferentes APIs
creator create -c infra -i clients -n UserApiClient
creator create -c infra -i clients -n PaymentApiClient
creator create -c infra -i clients -n NotificationClient

# Providers de configuração
creator create -c infra -i providers -n DatabaseProvider
creator create -c infra -i config -n ApiEndpoints
```

### Caso 3: Desenvolvimento com Arquitetura Modular

**Usando preset module-based:**

```bash
# 1. Inicializar projeto com preset
creator init -p module-based

# 2. Criar módulo de autenticação
creator create -c modules -i containers -n Authentication
creator create -c modules -i components -n LoginForm
creator create -c modules -i services -n AuthService

# 3. Adicionar utilitários compartilhados
creator create -c shared -i utils -n ValidationHelpers
creator create -c shared -i hooks -n useFormValidation
```

## Presets Disponíveis

### Clean Architecture Preset

**Arquivo**: `config-clean-architecture.json`

**Categorias:**

- **infra**: Configurações externas e integrações
- **features**: Features de negócio (dinâmica)
- **pages**: Páginas/telas da aplicação
- **core**: Utilitários e código compartilhado

**Ideal para**: Projetos que seguem Clean Architecture e DDD

### Module-Based Preset

**Arquivo**: `config-module-based.json`

**Categorias:**

- **application**: Camada principal da aplicação
- **modules**: Módulos de negócio (dinâmica)
- **shared**: Componentes e utilitários compartilhados
- **external**: Integrações externas (mista)

**Ideal para**: Projetos modulares com separação clara de responsabilidades

## Templates e Personalização

### Templates Disponíveis

#### 1. Default Template (`templates/default.hbs`)

```typescript
export function {{templateName}}(){}
```

#### 2. Components Template (`templates/components.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function {{templateName}}(){}
```

#### 3. Hooks Template (`templates/hooks.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function use{{templateName}}(){}
```

### Criando Templates Personalizados

1. **Criar novo template:**

```handlebars
// templates/service.hbs
export class {{templateName}}Service {
  constructor() {
    // Initialize service
  }

  async execute(): Promise<void> {
    // Implementation here
  }
}
```

2. **Usar no config:**

```json
{
  "api-service": {
    "template": "templates/service.hbs",
    "file_extension": "ts"
  }
}
```

### Variáveis Disponíveis nos Templates

- `{{templateName}}`: Nome em PascalCase (ex: "UserService")
- Nome do arquivo: sempre "index" + extensão configurada
- Estrutura de pastas: `category/kebab-case-name/item-type/`

## Fluxos de Trabalho Recomendados

### Para Projetos Novos

1. **Escolher arquitetura** e inicializar com preset apropriado
2. **Customizar configuração** se necessário
3. **Criar estrutura base** usando categorias estáticas
4. **Desenvolver features** usando categorias dinâmicas

### Para Projetos Existentes

1. **Analisar estrutura atual** do projeto
2. **Criar configuração personalizada** que espelhe a estrutura
3. **Testar com categoria de teste** antes de usar em produção
4. **Migrar gradualmente** usando a Creator CLI

### Trabalhando em Equipe

1. **Versionar configuração** no repositório (`config.json`)
2. **Documentar convenções** específicas do projeto
3. **Usar presets** para novos membros da equipe
4. **Validar estrutura** com `creator list` regularmente

## Troubleshooting

### Problemas Comuns

#### 1. Config não encontrada

```
Error: Config file not found
```

**Solução:**

- Verificar se existe `config.json` no diretório
- Usar `-c` para especificar arquivo específico
- Executar `creator init` para criar configuração inicial

#### 2. Categoria não encontrada

```
Error: Category 'feature' not found
```

**Solução:**

- Verificar nome da categoria com `creator list`
- Conferir configuração JSON
- Verificar se a categoria está definida na estrutura

#### 3. Template não encontrado

```
Error: Template file not found: templates/custom.hbs
```

**Solução:**

- Verificar se o arquivo template existe
- Usar caminho relativo correto
- Conferir permissões de arquivo

#### 4. Nome inválido

```
Error: Name can only contain alphanumeric characters, underscore, and dash
```

**Solução:**

- Usar apenas caracteres alfanuméricos, `_` e `-`
- Evitar espaços e caracteres especiais
- Exemplo válido: `user-management`, `user_service`, `UserComponent`

### Dicas de Performance

1. **Configurações pequenas**: Evitar estruturas muito complexas
2. **Templates simples**: Templates muito complexos podem impactar performance
3. **Cache de configuração**: A CLI faz cache automático durante execução

### Debug e Logs

```bash
# Usar modo verbose (se disponível)
creator -v create -c features -i modules -n TestFeature

# Verificar configuração carregada
creator list
```

### Validação de Configuração

A CLI automaticamente valida:

- ✅ Sintaxe JSON válida
- ✅ Campos obrigatórios presentes
- ✅ Templates existem
- ✅ Extensões de arquivo válidas
- ✅ Estrutura de categorias consistente

---

## Conclusão

A Creator CLI v2.0 oferece flexibilidade total para gerenciar estruturas de projeto através de configuração JSON. Com suporte a categorias estáticas, dinâmicas e mistas, templates personalizáveis e sistema de presets, ela se adapta a qualquer arquitetura de projeto.

**Próximos passos recomendados:**

1. Experimentar com modo interativo: `creator`
2. Explorar presets disponíveis: `creator init`
3. Customizar configuração para seu projeto
4. Criar templates personalizados conforme necessário

Para mais informações, consulte o [README.md](../README.md) ou abra uma [issue no GitHub](https://github.com/andraderaul/creator/issues).
