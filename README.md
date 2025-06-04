# Template Axum SQLx API

Un template pour construire des APIs REST avec Axum et SQLx.

## Fonctionnalités

- 🚀 Framework web Axum
- 🗄️ Base de données PostgreSQL avec SQLx
- 🔒 Validation des données
- 📝 Logging structuré
- 🔄 Gestion des erreurs
- 🌐 CORS configurable
- 📦 Configuration via fichier TOML
- 📊 Endpoints de diagnostic et monitoring
- 🔍 Validation des données avec validator

## Prérequis

- Rust (dernière version stable)
- PostgreSQL
- Docker (optionnel)

## Installation

1. Clonez le repository :
```bash
git clone http://localhost:3000/osef/template-axum-sqlx-api.git
cd template-axum-sqlx-api
```

2. Créez un fichier `config.toml` à la racine du projet :
```toml
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "postgres://postgres:postgres@localhost:5432/template_db"

[logging]
level = "info"
```

3. Créez la base de données :
```bash
sqlx database create
```

4. Exécutez les migrations (à implémenter) :
```bash
sqlx migrate run
```

## Démarrage

```bash
cargo run
```

Le serveur sera accessible à l'adresse `http://localhost:3000`.

## Endpoints d'Aide et Diagnostic

L'API fournit plusieurs endpoints pour le monitoring et le diagnostic :

- `GET /help/health` : Vérification complète de l'état de santé du système
  - État de la base de données
  - Métriques système (CPU, mémoire, disque)
  - Temps de réponse
- `GET /help/health-light` : Vérification rapide (DB + performance)
- `GET /help/info` : Informations sur l'API
- `GET /help/ping` : Test de connectivité simple

## Structure du Projet

```
.
├── src/
│   ├── config.rs     # Configuration de l'application
│   ├── db.rs         # Gestion de la base de données
│   ├── handlers/     # Gestionnaires de routes
│   │   ├── common.rs # Handlers communs
│   │   └── help.rs   # Handlers d'aide et diagnostic
│   ├── models/       # Modèles de données
│   │   ├── common.rs # Modèles communs
│   │   └── help.rs   # Modèles d'aide et diagnostic
│   ├── routes/       # Définition des routes
│   │   ├── common.rs # Routes communes
│   │   └── help.rs   # Routes d'aide et diagnostic
│   └── main.rs       # Point d'entrée
├── config.toml       # Configuration de l'application
├── .gitignore
├── Cargo.toml
└── README.md
```

## Configuration

La configuration se fait via le fichier `config.toml` :

```toml
[server]
host = "127.0.0.1"  # Adresse du serveur
port = 3000         # Port du serveur

[database]
url = "postgres://postgres:postgres@localhost:5432/template_db"  # URL de connexion

[logging]
level = "info"      # Niveau de logging (debug, info, warn, error)
```

## Développement

### Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatage

```bash
cargo fmt
```

## Docker

Pour construire et exécuter avec Docker :

```bash
docker build -t template-axum-sqlx-api .
docker run -p 3000:3000 template-axum-sqlx-api
```

## Licence

MIT 