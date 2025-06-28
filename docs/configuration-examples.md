# Exemplos de Configuração - Creator CLI v2.0

## 📋 Índice

- [Configurações por Arquitetura](#configurações-por-arquitetura)
- [Configurações por Stack](#configurações-por-stack)
- [Configurações por Domínio](#configurações-por-domínio)
- [Templates Personalizados](#templates-personalizados)
- [Casos Especiais](#casos-especiais)

## Configurações por Arquitetura

### 🏛️ Clean Architecture + DDD

```json
{
  "project": {
    "name": "my-clean-ddd-app",
    "version": "2.0",
    "structure": {
      "domain": {
        "description": "Domain layer - business entities and rules",
        "allow_dynamic_children": true,
        "default_structure": {
          "entities": {
            "template": "templates/entity.hbs",
            "file_extension": "ts"
          },
          "value-objects": {
            "template": "templates/value-object.hbs",
            "file_extension": "ts"
          },
          "repositories": {
            "template": "templates/repository-interface.hbs",
            "file_extension": "ts"
          },
          "services": {
            "template": "templates/domain-service.hbs",
            "file_extension": "ts"
          }
        }
      },
      "application": {
        "description": "Application layer - use cases and DTOs",
        "allow_dynamic_children": true,
        "default_structure": {
          "use-cases": {
            "template": "templates/use-case.hbs",
            "file_extension": "ts"
          },
          "dtos": {
            "template": "templates/dto.hbs",
            "file_extension": "ts"
          },
          "validators": {
            "template": "templates/validator.hbs",
            "file_extension": "ts"
          }
        }
      },
      "infrastructure": {
        "description": "Infrastructure layer - external concerns",
        "children": {
          "repositories": {
            "template": "templates/repository-impl.hbs",
            "file_extension": "ts"
          },
          "database": {
            "template": "templates/database.hbs",
            "file_extension": "ts"
          },
          "external-apis": {
            "template": "templates/api-client.hbs",
            "file_extension": "ts"
          },
          "config": {
            "template": "templates/config.hbs",
            "file_extension": "ts"
          }
        }
      },
      "presentation": {
        "description": "Presentation layer - UI components and controllers",
        "children": {
          "screens": {
            "template": "templates/screen.hbs",
            "file_extension": "tsx"
          },
          "components": {
            "template": "templates/component.hbs",
            "file_extension": "tsx"
          },
          "controllers": {
            "template": "templates/controller.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

**Uso típico:**

```bash
# Criar domínio de usuário
creator create -c domain -i entities -n User
creator create -c domain -i repositories -n UserRepository
creator create -c domain -i services -n UserDomainService

# Casos de uso
creator create -c application -i use-cases -n CreateUser
creator create -c application -i dtos -n CreateUserDto

# Infraestrutura
creator create -c infrastructure -i repositories -n UserRepositoryImpl
creator create -c infrastructure -i external-apis -n AuthApiClient

# Apresentação
creator create -c presentation -i screens -n UserProfileScreen
creator create -c presentation -i components -n UserCard
```

### 🏗️ Hexagonal Architecture

```json
{
  "project": {
    "name": "hexagonal-app",
    "version": "2.0",
    "structure": {
      "core": {
        "description": "Core business logic - ports and domain",
        "allow_dynamic_children": true,
        "default_structure": {
          "domain": {
            "template": "templates/domain-model.hbs",
            "file_extension": "ts"
          },
          "ports": {
            "template": "templates/port.hbs",
            "file_extension": "ts"
          },
          "services": {
            "template": "templates/domain-service.hbs",
            "file_extension": "ts"
          }
        }
      },
      "adapters": {
        "description": "External adapters - driving and driven",
        "children": {
          "driving": {
            "template": "templates/driving-adapter.hbs",
            "file_extension": "ts"
          },
          "driven": {
            "template": "templates/driven-adapter.hbs",
            "file_extension": "ts"
          }
        },
        "allow_dynamic_children": true,
        "default_structure": {
          "rest": {
            "template": "templates/rest-adapter.hbs",
            "file_extension": "ts"
          },
          "persistence": {
            "template": "templates/persistence-adapter.hbs",
            "file_extension": "ts"
          }
        }
      },
      "configuration": {
        "description": "Application configuration and dependency injection",
        "children": {
          "di": {
            "template": "templates/di-container.hbs",
            "file_extension": "ts"
          },
          "config": {
            "template": "templates/app-config.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

### 🧩 Feature-Based Architecture

```json
{
  "project": {
    "name": "feature-based-app",
    "version": "2.0",
    "structure": {
      "features": {
        "description": "Business features with complete isolation",
        "allow_dynamic_children": true,
        "default_structure": {
          "components": {
            "template": "templates/feature-component.hbs",
            "file_extension": "tsx"
          },
          "hooks": {
            "template": "templates/feature-hook.hbs",
            "file_extension": "ts"
          },
          "services": {
            "template": "templates/feature-service.hbs",
            "file_extension": "ts"
          },
          "types": {
            "template": "templates/feature-types.hbs",
            "file_extension": "ts"
          },
          "utils": {
            "template": "templates/feature-utils.hbs",
            "file_extension": "ts"
          },
          "tests": {
            "template": "templates/feature-test.hbs",
            "file_extension": "test.ts"
          }
        }
      },
      "shared": {
        "description": "Shared utilities and components",
        "children": {
          "ui": {
            "template": "templates/shared-component.hbs",
            "file_extension": "tsx"
          },
          "utils": {
            "template": "templates/shared-utils.hbs",
            "file_extension": "ts"
          },
          "hooks": {
            "template": "templates/shared-hook.hbs",
            "file_extension": "ts"
          },
          "constants": {
            "template": "templates/constants.hbs",
            "file_extension": "ts"
          }
        }
      },
      "core": {
        "description": "Core application logic",
        "children": {
          "api": {
            "template": "templates/api-client.hbs",
            "file_extension": "ts"
          },
          "store": {
            "template": "templates/store.hbs",
            "file_extension": "ts"
          },
          "navigation": {
            "template": "templates/navigation.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

## Configurações por Stack

### ⚛️ React Native + TypeScript + Redux

```json
{
  "project": {
    "name": "rn-redux-app",
    "version": "2.0",
    "structure": {
      "screens": {
        "description": "Application screens",
        "allow_dynamic_children": true,
        "default_structure": {
          "container": {
            "template": "templates/rn-screen-container.hbs",
            "file_extension": "tsx"
          },
          "component": {
            "template": "templates/rn-screen-component.hbs",
            "file_extension": "tsx"
          },
          "styles": {
            "template": "templates/rn-styles.hbs",
            "file_extension": "ts"
          }
        }
      },
      "components": {
        "description": "Reusable UI components",
        "allow_dynamic_children": true,
        "default_structure": {
          "component": {
            "template": "templates/rn-component.hbs",
            "file_extension": "tsx"
          },
          "styles": {
            "template": "templates/rn-component-styles.hbs",
            "file_extension": "ts"
          },
          "types": {
            "template": "templates/component-types.hbs",
            "file_extension": "ts"
          }
        }
      },
      "store": {
        "description": "Redux store management",
        "allow_dynamic_children": true,
        "default_structure": {
          "slice": {
            "template": "templates/redux-slice.hbs",
            "file_extension": "ts"
          },
          "thunk": {
            "template": "templates/redux-thunk.hbs",
            "file_extension": "ts"
          },
          "selector": {
            "template": "templates/redux-selector.hbs",
            "file_extension": "ts"
          }
        }
      },
      "services": {
        "description": "API and business services",
        "children": {
          "api": {
            "template": "templates/api-service.hbs",
            "file_extension": "ts"
          },
          "storage": {
            "template": "templates/storage-service.hbs",
            "file_extension": "ts"
          },
          "navigation": {
            "template": "templates/navigation-service.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

### 🌐 Next.js + TypeScript + tRPC

```json
{
  "project": {
    "name": "nextjs-trpc-app",
    "version": "2.0",
    "structure": {
      "pages": {
        "description": "Next.js pages",
        "allow_dynamic_children": true,
        "default_structure": {
          "page": {
            "template": "templates/nextjs-page.hbs",
            "file_extension": "tsx"
          },
          "api": {
            "template": "templates/nextjs-api.hbs",
            "file_extension": "ts"
          }
        }
      },
      "components": {
        "description": "React components",
        "allow_dynamic_children": true,
        "default_structure": {
          "component": {
            "template": "templates/react-component.hbs",
            "file_extension": "tsx"
          },
          "styles": {
            "template": "templates/css-module.hbs",
            "file_extension": "module.css"
          }
        }
      },
      "server": {
        "description": "tRPC server logic",
        "children": {
          "routers": {
            "template": "templates/trpc-router.hbs",
            "file_extension": "ts"
          },
          "procedures": {
            "template": "templates/trpc-procedure.hbs",
            "file_extension": "ts"
          },
          "middleware": {
            "template": "templates/trpc-middleware.hbs",
            "file_extension": "ts"
          }
        }
      },
      "lib": {
        "description": "Utilities and configurations",
        "children": {
          "utils": {
            "template": "templates/utility.hbs",
            "file_extension": "ts"
          },
          "hooks": {
            "template": "templates/react-hook.hbs",
            "file_extension": "ts"
          },
          "types": {
            "template": "templates/types.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

### 📱 Flutter + Dart + Bloc

```json
{
  "project": {
    "name": "flutter-bloc-app",
    "version": "2.0",
    "structure": {
      "features": {
        "description": "Feature-based Flutter modules",
        "allow_dynamic_children": true,
        "default_structure": {
          "bloc": {
            "template": "templates/flutter-bloc.hbs",
            "file_extension": "dart"
          },
          "event": {
            "template": "templates/flutter-event.hbs",
            "file_extension": "dart"
          },
          "state": {
            "template": "templates/flutter-state.hbs",
            "file_extension": "dart"
          },
          "widget": {
            "template": "templates/flutter-widget.hbs",
            "file_extension": "dart"
          },
          "repository": {
            "template": "templates/flutter-repository.hbs",
            "file_extension": "dart"
          }
        }
      },
      "shared": {
        "description": "Shared Flutter components",
        "children": {
          "widgets": {
            "template": "templates/shared-widget.hbs",
            "file_extension": "dart"
          },
          "utils": {
            "template": "templates/dart-utils.hbs",
            "file_extension": "dart"
          },
          "constants": {
            "template": "templates/dart-constants.hbs",
            "file_extension": "dart"
          }
        }
      },
      "core": {
        "description": "Core application logic",
        "children": {
          "network": {
            "template": "templates/network-client.hbs",
            "file_extension": "dart"
          },
          "storage": {
            "template": "templates/storage-service.hbs",
            "file_extension": "dart"
          },
          "theme": {
            "template": "templates/app-theme.hbs",
            "file_extension": "dart"
          }
        }
      }
    }
  }
}
```

## Configurações por Domínio

### 🛒 E-commerce

```json
{
  "project": {
    "name": "ecommerce-platform",
    "version": "2.0",
    "structure": {
      "catalog": {
        "description": "Product catalog management",
        "allow_dynamic_children": true,
        "default_structure": {
          "products": {
            "template": "templates/product-component.hbs",
            "file_extension": "tsx"
          },
          "categories": {
            "template": "templates/category-component.hbs",
            "file_extension": "tsx"
          },
          "search": {
            "template": "templates/search-component.hbs",
            "file_extension": "tsx"
          }
        }
      },
      "cart": {
        "description": "Shopping cart functionality",
        "children": {
          "components": {
            "template": "templates/cart-component.hbs",
            "file_extension": "tsx"
          },
          "services": {
            "template": "templates/cart-service.hbs",
            "file_extension": "ts"
          },
          "hooks": {
            "template": "templates/cart-hook.hbs",
            "file_extension": "ts"
          }
        }
      },
      "checkout": {
        "description": "Checkout and payment flow",
        "children": {
          "steps": {
            "template": "templates/checkout-step.hbs",
            "file_extension": "tsx"
          },
          "payment": {
            "template": "templates/payment-method.hbs",
            "file_extension": "tsx"
          },
          "validation": {
            "template": "templates/checkout-validator.hbs",
            "file_extension": "ts"
          }
        }
      },
      "orders": {
        "description": "Order management",
        "children": {
          "tracking": {
            "template": "templates/order-tracking.hbs",
            "file_extension": "tsx"
          },
          "history": {
            "template": "templates/order-history.hbs",
            "file_extension": "tsx"
          },
          "details": {
            "template": "templates/order-details.hbs",
            "file_extension": "tsx"
          }
        }
      },
      "users": {
        "description": "User management and profiles",
        "children": {
          "auth": {
            "template": "templates/auth-component.hbs",
            "file_extension": "tsx"
          },
          "profile": {
            "template": "templates/profile-component.hbs",
            "file_extension": "tsx"
          },
          "preferences": {
            "template": "templates/user-preferences.hbs",
            "file_extension": "tsx"
          }
        }
      }
    }
  }
}
```

### 🏥 Healthcare

```json
{
  "project": {
    "name": "healthcare-app",
    "version": "2.0",
    "structure": {
      "patients": {
        "description": "Patient management",
        "allow_dynamic_children": true,
        "default_structure": {
          "records": {
            "template": "templates/patient-record.hbs",
            "file_extension": "tsx"
          },
          "appointments": {
            "template": "templates/appointment-component.hbs",
            "file_extension": "tsx"
          },
          "vitals": {
            "template": "templates/vitals-component.hbs",
            "file_extension": "tsx"
          }
        }
      },
      "medical": {
        "description": "Medical information and procedures",
        "children": {
          "diagnoses": {
            "template": "templates/diagnosis-component.hbs",
            "file_extension": "tsx"
          },
          "treatments": {
            "template": "templates/treatment-component.hbs",
            "file_extension": "tsx"
          },
          "medications": {
            "template": "templates/medication-component.hbs",
            "file_extension": "tsx"
          }
        }
      },
      "scheduling": {
        "description": "Appointment and resource scheduling",
        "children": {
          "calendar": {
            "template": "templates/calendar-component.hbs",
            "file_extension": "tsx"
          },
          "resources": {
            "template": "templates/resource-component.hbs",
            "file_extension": "tsx"
          },
          "availability": {
            "template": "templates/availability-component.hbs",
            "file_extension": "tsx"
          }
        }
      },
      "compliance": {
        "description": "Healthcare compliance and security",
        "children": {
          "audit": {
            "template": "templates/audit-component.hbs",
            "file_extension": "tsx"
          },
          "security": {
            "template": "templates/security-service.hbs",
            "file_extension": "ts"
          },
          "reporting": {
            "template": "templates/compliance-report.hbs",
            "file_extension": "tsx"
          }
        }
      }
    }
  }
}
```

## Templates Personalizados

### Entity Template (DDD)

```handlebars
{{! templates/entity.hbs }}
export class
{{templateName}}
{ private constructor( private readonly _id: string, private _props:
{{templateName}}Props ) {} public static create(props:
{{templateName}}Props, id?: string):
{{templateName}}
{ // TODO: Add validation logic return new
{{templateName}}(id || generateId(), props); } public get id(): string { return
this._id; } // TODO: Add domain methods public toSnapshot():
{{templateName}}Snapshot { return { id: this._id, ...this._props }; } } export
interface
{{templateName}}Props { // TODO: Define entity properties } export interface
{{templateName}}Snapshot extends
{{templateName}}Props { id: string; }
```

### Use Case Template

```handlebars
{{!-- templates/use-case.hbs --}}
import { UseCase } from '../../../core/use-case';

export interface {{templateName}}Request {
  // TODO: Define input parameters
}

export interface {{templateName}}Response {
  // TODO: Define response structure
}

export class {{templateName}} implements UseCase<{{templateName}}Request, {{templateName}}Response> {
  constructor(
    // TODO: Inject required repositories and services
  ) {}

  async execute(request: {{templateName}}Request): Promise<{{templateName}}Response> {
    // TODO: Implement use case logic

    // 1. Validate input

    // 2. Execute business logic

    // 3. Return response

    throw new Error('Not implemented');
  }
}
```

### React Component Template

```handlebars
{{!-- templates/feature-component.hbs --}}
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

export interface {{templateName}}Props {
  // TODO: Define component props
}

export const {{templateName}}: React.FC<{{templateName}}Props> = ({
  // TODO: Destructure props
}) => {
  // TODO: Add component logic

  return (
    <View style={styles.container}>
      <Text style={styles.title}>{{templateName}}</Text>
      {/* TODO: Add component JSX */}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 16,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 16,
  },
});
```

### API Service Template

```handlebars
{{!-- templates/api-service.hbs --}}
import { ApiClient } from '../core/api-client';

export interface {{templateName}}Data {
  // TODO: Define data structure
}

export interface {{templateName}}Filters {
  // TODO: Define filter parameters
}

export class {{templateName}}Service {
  constructor(private readonly apiClient: ApiClient) {}

  async getAll(filters?: {{templateName}}Filters): Promise<{{templateName}}Data[]> {
    try {
      const response = await this.apiClient.get('/{{templateName | lowercase}}', {
        params: filters,
      });
      return response.data;
    } catch (error) {
      // TODO: Handle error appropriately
      throw error;
    }
  }

  async getById(id: string): Promise<{{templateName}}Data> {
    try {
      const response = await this.apiClient.get(`/{{templateName | lowercase}}/${id}`);
      return response.data;
    } catch (error) {
      // TODO: Handle error appropriately
      throw error;
    }
  }

  async create(data: Omit<{{templateName}}Data, 'id'>): Promise<{{templateName}}Data> {
    try {
      const response = await this.apiClient.post('/{{templateName | lowercase}}', data);
      return response.data;
    } catch (error) {
      // TODO: Handle error appropriately
      throw error;
    }
  }

  async update(id: string, data: Partial<{{templateName}}Data>): Promise<{{templateName}}Data> {
    try {
      const response = await this.apiClient.put(`/{{templateName | lowercase}}/${id}`, data);
      return response.data;
    } catch (error) {
      // TODO: Handle error appropriately
      throw error;
    }
  }

  async delete(id: string): Promise<void> {
    try {
      await this.apiClient.delete(`/{{templateName | lowercase}}/${id}`);
    } catch (error) {
      // TODO: Handle error appropriately
      throw error;
    }
  }
}
```

## Casos Especiais

### Configuração para Micro-frontends

```json
{
  "project": {
    "name": "micro-frontend-shell",
    "version": "2.0",
    "structure": {
      "shell": {
        "description": "Main shell application",
        "children": {
          "layout": {
            "template": "templates/shell-layout.hbs",
            "file_extension": "tsx"
          },
          "router": {
            "template": "templates/shell-router.hbs",
            "file_extension": "tsx"
          },
          "federation": {
            "template": "templates/module-federation.hbs",
            "file_extension": "ts"
          }
        }
      },
      "microfrontends": {
        "description": "Individual micro-frontend modules",
        "allow_dynamic_children": true,
        "default_structure": {
          "bootstrap": {
            "template": "templates/mf-bootstrap.hbs",
            "file_extension": "tsx"
          },
          "app": {
            "template": "templates/mf-app.hbs",
            "file_extension": "tsx"
          },
          "routes": {
            "template": "templates/mf-routes.hbs",
            "file_extension": "tsx"
          },
          "webpack": {
            "template": "templates/mf-webpack.hbs",
            "file_extension": "js"
          }
        }
      },
      "shared": {
        "description": "Shared libraries and components",
        "children": {
          "design-system": {
            "template": "templates/design-system.hbs",
            "file_extension": "tsx"
          },
          "utils": {
            "template": "templates/shared-utils.hbs",
            "file_extension": "ts"
          },
          "types": {
            "template": "templates/shared-types.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

### Configuração para Monorepo

```json
{
  "project": {
    "name": "monorepo-workspace",
    "version": "2.0",
    "structure": {
      "apps": {
        "description": "Applications in the monorepo",
        "allow_dynamic_children": true,
        "default_structure": {
          "web": {
            "template": "templates/web-app.hbs",
            "file_extension": "tsx"
          },
          "mobile": {
            "template": "templates/mobile-app.hbs",
            "file_extension": "tsx"
          },
          "api": {
            "template": "templates/api-app.hbs",
            "file_extension": "ts"
          }
        }
      },
      "packages": {
        "description": "Shared packages",
        "allow_dynamic_children": true,
        "default_structure": {
          "lib": {
            "template": "templates/package-lib.hbs",
            "file_extension": "ts"
          },
          "ui": {
            "template": "templates/package-ui.hbs",
            "file_extension": "tsx"
          },
          "config": {
            "template": "templates/package-config.hbs",
            "file_extension": "ts"
          }
        }
      },
      "tools": {
        "description": "Development tools and scripts",
        "children": {
          "build": {
            "template": "templates/build-tool.hbs",
            "file_extension": "js"
          },
          "linting": {
            "template": "templates/lint-config.hbs",
            "file_extension": "js"
          },
          "testing": {
            "template": "templates/test-config.hbs",
            "file_extension": "js"
          }
        }
      }
    }
  }
}
```

### Configuração para Testing

```json
{
  "project": {
    "name": "test-driven-app",
    "version": "2.0",
    "structure": {
      "features": {
        "description": "Features with comprehensive testing",
        "allow_dynamic_children": true,
        "default_structure": {
          "component": {
            "template": "templates/tdd-component.hbs",
            "file_extension": "tsx"
          },
          "service": {
            "template": "templates/tdd-service.hbs",
            "file_extension": "ts"
          },
          "hook": {
            "template": "templates/tdd-hook.hbs",
            "file_extension": "ts"
          }
        }
      },
      "tests": {
        "description": "Test utilities and configurations",
        "children": {
          "unit": {
            "template": "templates/unit-test.hbs",
            "file_extension": "test.ts"
          },
          "integration": {
            "template": "templates/integration-test.hbs",
            "file_extension": "test.ts"
          },
          "e2e": {
            "template": "templates/e2e-test.hbs",
            "file_extension": "spec.ts"
          },
          "fixtures": {
            "template": "templates/test-fixture.hbs",
            "file_extension": "ts"
          },
          "mocks": {
            "template": "templates/test-mock.hbs",
            "file_extension": "ts"
          }
        }
      }
    }
  }
}
```

---

## 🎯 Como Usar Estes Exemplos

1. **Escolha a configuração** que melhor se adapta ao seu projeto
2. **Copie o JSON** para seu `config.json`
3. **Customize templates** conforme necessário
4. **Teste a configuração**:
   ```bash
   creator list
   creator create -c categoria -i tipo -n ExemploTeste
   ```
5. **Ajuste conforme necessário** para seu contexto específico

**Dica**: Combine elementos de diferentes configurações para criar uma estrutura única para seu projeto!

---

_Para mais exemplos e configurações, consulte o [Guia de Uso Completo](./cli-usage-guide.md)_ 📚
