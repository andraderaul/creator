# Creator - Documentação Completa do Projeto

## 📋 Resumo Executivo

**Creator** é uma ferramenta CLI (Command-Line Interface) desenvolvida em Rust, especificamente projetada para manter estruturas de pastas consistentes em projetos React Native. A ferramenta automatiza a criação de arquivos e diretórios seguindo padrões pré-definidos através de configuração JSON e templates Handlebars.

## 🏗️ Arquitetura do Projeto

### Visão Geral da Estrutura

```
creator/
├── src/
│   ├── main.rs         # Entry point da aplicação
│   ├── lib.rs          # Módulo raiz da biblioteca
│   ├── config.rs       # Gerenciamento de configuração
│   ├── opts.rs         # Parser de argumentos CLI (clap)
│   ├── creator.rs      # Lógica principal de criação
│   ├── generator.rs    # Sistema de templates Handlebars
│   └── file_utils.rs   # Utilitários para operações de arquivo
├── templates/
│   ├── components.hbs  # Template para componentes React
│   ├── hooks.hbs       # Template para hooks customizados
│   └── default.hbs     # Template genérico
├── config.json         # Configuração da estrutura de pastas
├── Cargo.toml          # Configuração do projeto Rust
└── README.md           # Documentação do usuário
```

## 🔧 Análise Técnica Detalhada

### 1. Entry Point (`main.rs`)

O fluxo principal da aplicação segue um padrão simples e eficiente:

```rust
fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let creator = Creator::from_config(config.config, config.source_dir);

    match config.commands {
        Commands::NewFeature { feature_name } => { /* ... */ }
        Commands::NewCore {} => { /* ... */ }
        Commands::NewApplication {} => { /* ... */ }
        Commands::NewComponent { feature, name } => { /* ... */ }
    }

    Ok(())
}
```

**Pontos-chave:**

- Utiliza `anyhow` para tratamento de erros unificado
- Implementa pattern matching para diferentes comandos
- Separação clara de responsabilidades

### 2. Sistema de Configuração (`config.rs`)

**Arquitetura de Configuração Híbrida:**

- **CLI-first**: Aceita parâmetros via linha de comando
- **Interactive fallback**: Quando parâmetros não são fornecidos, utiliza `inquire` para interface interativa
- **Validation**: Implementa validação de entrada com feedback personalizado

**Design Pattern Implementado:**

```rust
impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let config = get_config(value.config)?;
        let source_dir = get_source_dir(value.source_dir)?;
        let commands = get_commands(value.commands)?;
        // ...
    }
}
```

### 3. Sistema de CLI (`opts.rs`)

**Framework**: Utiliza `clap` v4 com derive macros para parsing de argumentos.

**Comandos Disponíveis:**

- `new-feature <feature_name>`: Cria estrutura completa de feature
- `new-core`: Gera módulos do core da aplicação
- `new-application`: Cria estrutura de aplicação
- `new-component <feature> <name>`: Adiciona componente a uma feature existente

**Características Técnicas:**

- Parser declarativo com macros
- Suporte a subcomandos aninhados
- Validação automática de argumentos

### 4. Engine de Criação (`creator.rs`)

**Design Patterns Utilizados:**

- **Builder Pattern**: Construção do Creator via `from_config`
- **Strategy Pattern**: Diferentes estratégias para cada tipo de criação
- **Template Method**: Método `create()` genérico com implementações específicas

**Estrutura de Dados:**

```rust
pub type SubStructure = HashMap<String, FileStructure>;
pub type MainStructure = HashMap<String, SubStructure>;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    creator: MainStructure,
}
```

**Funcionalidades Core:**

- `create_feature()`: Criação de features com estrutura completa
- `create_core()`: Geração de módulos core
- `create_application()`: Estruturação de aplicação
- `create_component_module()`: Adição de componentes a features

### 5. Sistema de Templates (`generator.rs`)

**Template Engine**: Handlebars para Rust

- **Simplicidade**: Templates minimalistas focados em funcionalidade
- **Flexibilidade**: Suporte a variáveis dinâmicas (`{{templateName}}`)
- **Fallback**: Template padrão quando arquivo não encontrado

**Implementação:**

```rust
pub fn generate(path: &PathBuf, name: String) -> Result<String> {
    let source = fs::read_to_string(&path).unwrap_or_else(|_| {
        String::from("export function {{templateName}}(){}")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", &source)?;

    let mut data = BTreeMap::new();
    data.insert("templateName".to_string(), name);

    handlebars.render("template", &data)
}
```

### 6. Utilitários de Arquivo (`file_utils.rs`)

**Funções Core:**

- `create_folder()`: Criação recursiva de diretórios
- `create_file()`: Criação de arquivos com conteúdo
- `to_kebab_case()`: Conversão para formato kebab-case
- `to_pascal_case()`: Conversão para formato PascalCase

**Características Técnicas:**

- Error handling robusto com `anyhow`
- Funções puras e testáveis
- Cobertura de testes unitários

## 📁 Sistema de Configuração (config.json)

### Estrutura da Configuração

```json
{
  "creator": {
    "features": {
      "hooks": {
        "template": "templates/hooks.hbs",
        "file": "index.ts"
      },
      "containers": {
        "template": "templates/components.hbs",
        "file": "index.tsx"
      },
      "components": {
        "template": "templates/components.hbs",
        "file": "index.tsx"
      },
      "services": {
        "template": "templates/components.hbs",
        "file": "index.ts"
      }
    },
    "core": {
      "notification": {
        "template": "templates/default.hbs",
        "file": "index.ts"
      },
      "localization": {
        "template": "templates/default.hbs",
        "file": "index.ts"
      },
      "logger": { "template": "templates/default.hbs", "file": "index.ts" }
    },
    "application": {
      "home": { "template": "templates/default.hbs", "file": "index.ts" },
      "product": { "template": "templates/default.hbs", "file": "index.ts" }
    }
  }
}
```

### Hierarquia de Configuração

1. **Nível 1 (creator)**: Container raiz
2. **Nível 2 (features/core/application)**: Categorias principais
3. **Nível 3 (hooks/components/etc)**: Subcategorias específicas
4. **Nível 4 (template/file)**: Configuração por item

## 🎯 Templates Handlebars

### Template para Componentes (`components.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function {{templateName}}(){}
```

### Template para Hooks (`hooks.hbs`)

```typescript
import { useState, useEffect } from 'react';

export function use{{templateName}}(){}
```

### Template Padrão (`default.hbs`)

```typescript
export function {{templateName}}(){}
```

## 🔄 Fluxo de Execução

### 1. Inicialização

```
CLI Args → Opts::parse() → Config::try_from() → Creator::from_config()
```

### 2. Processamento de Comandos

```
Commands::match → creator.create_*() → Generator::generate() → file_utils::create_*()
```

### 3. Geração de Arquivos

```
Template Loading → Handlebars Processing → File System Operations
```

## 🧪 Estratégia de Testes

### Cobertura Atual

- **file_utils.rs**: Testes unitários para conversões de string
- **config.rs**: Testes para funções de configuração
- **Ausente**: Testes de integração e end-to-end

### Gaps Identificados

- Testes para o fluxo completo de criação
- Mock do sistema de arquivos
- Testes de templates Handlebars
- Validação de estruturas geradas

## 📦 Dependências e Tecnologias

### Dependências Principais

- **clap (4.5.2)**: CLI parsing com derive macros
- **inquire (0.7.0)**: Interface interativa de terminal
- **handlebars (5.1.0)**: Template engine
- **serde (1.0)**: Serialização/deserialização JSON
- **anyhow (1.0)**: Error handling unificado

### Decisões Arquiteturais

- **Rust**: Performance, safety, ecosystem maduro para CLI
- **Handlebars**: Simplicidade vs. complexidade (vs. Tera, Liquid)
- **Clap**: Ecosystem standard para CLI em Rust
- **Inquire**: UX superior para interação de terminal

## 🎯 Pontos Fortes

### 1. **Arquitetura Limpa**

- Separação clara de responsabilidades
- Modules bem definidos
- Error handling consistente

### 2. **Experiência do Desenvolvedor**

- CLI intuitiva com help integrado
- Modo interativo como fallback
- Feedback claro de erros

### 3. **Flexibilidade**

- Templates customizáveis
- Configuração externa (JSON)
- Estrutura extensível

### 4. **Performance**

- Rust garantindo performance nativa
- Operações de I/O eficientes
- Binary standalone

## ⚠️ Pontos de Melhoria Identificados

### 1. **Cobertura de Testes**

- Falta de testes de integração
- Ausência de testes end-to-end
- Mock do file system necessário

### 2. **Validação de Configuração**

- Sem validação de schema JSON
- Falta validação de templates Handlebars
- Ausência de verificação de paths

### 3. **Funcionalidades Pendentes**

- Subdirectories baseados em config (roadmap)
- Autocompletar para paths
- Templates mais sofisticados

### 4. **Documentação**

- Falta de documentação de API
- Ausência de exemplos de uso
- Necessidade de guias de migração

## 🚀 Casos de Uso

### Caso de Uso 1: Nova Feature

```bash
creator new-feature authentication
```

**Resultado**: Cria estrutura completa com hooks, components, containers, services

### Caso de Uso 2: Componente Específico

```bash
creator new-component authentication LoginForm
```

**Resultado**: Adiciona LoginForm.tsx na feature authentication

### Caso de Uso 3: Modo Interativo

```bash
creator
```

**Resultado**: Interface interativa para seleção de comandos

## 📊 Métricas do Projeto

- **Linhas de Código**: ~350 LOC
- **Módulos**: 6 módulos principais
- **Templates**: 3 templates Handlebars
- **Comandos CLI**: 4 comandos principais
- **Dependências**: 5 dependências principais
- **Cobertura de Testes**: ~30% (estimado)

## 🏆 Conclusão

O Creator é uma ferramenta CLI bem estruturada que implementa boas práticas de desenvolvimento em Rust. Apresenta arquitetura limpa, separação de responsabilidades clara e experiência de usuário sólida.

**Pontos Destacados:**

- Implementação robusta com error handling adequado
- Flexibilidade através de configuração externa
- Performance garantida pela escolha do Rust
- CLI intuitiva com fallbacks interativos

**Oportunidades de Evolução:**

- Expansão da cobertura de testes
- Implementação de funcionalidades do roadmap
- Aprimoramento da validação de configuração
- Documentação mais abrangente

O projeto demonstra competência técnica sólida e pode servir como base para expansões futuras na automação de estruturas de projeto.
