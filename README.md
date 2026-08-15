# Multi-Project Repository

Este repositório contém implementações equivalentes de um projeto Hello World em diferentes tecnologias, organizadas em branches separadas.

## 📋 Estrutura do Repositório

### Branches Disponíveis

- **`main`** - Branch principal do repositório
- **`feature/corrigir-bugs-quarkus`** - Projeto Hello World em Quarkus/Java
- **`feature/migrar-para-rust`** - Projeto Hello World em Rust/Actix-web

## 🚀 Projetos

### 1. Hello Quarkus (Java)

**Branch**: `feature/corrigir-bugs-quarkus`

**Tecnologias**:
- Java 17
- Quarkus 3.38.2
- Maven 3.9.x
- Jakarta REST

**Como Acessar**:
```bash
git checkout feature/corrigir-bugs-quarkus
cd hello-quarkus
```

**Como Executar**:
```bash
# Modo desenvolvimento
./mvnw quarkus:dev

# Executar testes
./mvnw test

# Compilar
./mvnw clean install
```

**Endpoint**: `GET http://localhost:8080/hello`  
**Resposta**: `Hello from Quarkus REST`

**Documentação**: Veja `hello-quarkus/BUG_REPORT_ATUALIZADO.md` para detalhes técnicos e correções de bugs.

---

### 2. Hello Rust (Actix-web)

**Branch**: `feature/migrar-para-rust`

**Tecnologias**:
- Rust 1.97.1
- Actix-web 4.9
- Tokio async runtime

**Como Acessar**:
```bash
git checkout feature/migrar-para-rust
cd hello-rust
```

**Como Executar**:
```bash
# Modo desenvolvimento
cargo run

# Executar testes
cargo test

# Compilar
cargo build
```

**Endpoint**: `GET http://localhost:8080/hello`  
**Resposta**: `Hello from Rust REST`

---

## 🔄 Alternar Entre Projetos

```bash
# Para Quarkus
git checkout feature/corrigir-bugs-quarkus

# Para Rust
git checkout feature/migrar-para-rust

# Voltar ao branch principal
git checkout main
```

## 📊 Comparação

| Característica | Quarkus/Java | Rust/Actix-web |
|----------------|---------------|----------------|
| **Linguagem** | Java 17 | Rust 1.97.1 |
| **Framework** | Quarkus 3.38.2 | Actix-web 4.9 |
| **Build Tool** | Maven | Cargo |
| **Runtime** | JVM | Nativo |
| **Startup** | ~3.6s | ~3.5s |
| **Memória** | ~50MB | ~2MB |
| **Testes** | JUnit 5 | Rust built-in |

## 🎯 Status dos Projetos

### Quarkus
- ✅ Funcional e testado
- ✅ Bugs corrigidos (BUG_REPORT_ATUALIZADO.md)
- ✅ Integração Jira configurada (Ticket KAN-1)
- ✅ Documentação Confluence disponível
- 🔄 Integração Slack/Teams configurada (ver GUIA_SLACK_TEAMS.md)

### Rust
- ✅ Funcional e testado
- ✅ Migração completa de Quarkus
- ✅ Estrutura modular (lib.rs + main.rs)
- ✅ Testes automatizados

## 🤖 Integração com Devin

### Workflows Suportados

**Discussão de Bugs via Slack/Teams:**
```
@devin Encontrei um problema no endpoint /hello do projeto Quarkus.
Ao executar curl http://localhost:8080/hello, recebo timeout.
```

Veja [GUIA_SLACK_TEAMS.md](GUIA_SLACK_TEAMS.md) para configuração completa e exemplos de uso.

**Delegação via Aplicativo Web:**
- Delegar tarefas complexas através da interface web
- Assumir no IDE do Devin quando tiver um bom primeiro rascunho

**Devin para Terminal Local:**
- Executar correções rápidas e exploração de código
- Usar `/handoff` para enviar tarefas longas ao Devin na nuvem

## 📝 Notas

- Ambos projetos implementam o mesmo endpoint `/hello` com funcionalidade equivalente
- Os projetos são mantidos em branches separadas para facilitar comparação e aprendizado
- Estrutura permite fácil adição de novas implementações em outras tecnologias

## 📄 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo LICENSE para detalhes.