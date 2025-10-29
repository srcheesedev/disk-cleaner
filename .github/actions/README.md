# GitHub Actions

Este directorio contiene acciones personalizadas reutilizables para CI/CD.

## 🦀 setup-rust

Configura el entorno de Rust con cache inteligente.

```yaml
- uses: ./.github/actions/setup-rust
  with:
    components: 'rustfmt,clippy'
    targets: 'x86_64-unknown-linux-gnu'
    install-cargo-audit: 'true'
```

## 🔧 setup-git

Configura Git para commits automatizados.

```yaml
- uses: ./.github/actions/setup-git
  with:
    user-name: 'Release Bot'
```

## 🔄 auto-version

Versionado semántico automático basado en conventional commits.

```yaml
- uses: ./.github/actions/auto-version
  with:
    version-file: 'Cargo.toml'
    token: ${{ secrets.GITHUB_TOKEN }}
```

## 🏗️ build-artifacts

Construcción y empaquetado multi-plataforma.

```yaml
- uses: ./.github/actions/build-artifacts
  with:
    version: '1.0.0'
    target: 'x86_64-unknown-linux-gnu'
    platform: 'linux'
```

## � github-release

Creación de releases con changelog automático.

```yaml
- uses: ./.github/actions/github-release
  with:
    version: '1.0.0'
    token: ${{ secrets.GITHUB_TOKEN }}
```

## 🎯 Beneficios

- **🔄 Reutilizable**: Actions genéricos para cualquier proyecto
- **📦 Modular**: Cada action tiene una responsabilidad específica
- **🧹 DRY**: Elimina duplicación de código
- **⚡ Eficiente**: Pipeline de 95 líneas vs 369 original
- **🛠️ Mantenible**: Cambios centralizados en actions