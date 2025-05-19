# Template Axum SQLx API

Un template pour construire des APIs REST avec Axum et SQLx.

## Fonctionnalités

- 🚀 Framework web Axum
- 🗄️ Base de données PostgreSQL avec SQLx
- 🔒 Validation des données
- 📝 Logging structuré
- 🔄 Gestion des erreurs
- 🌐 CORS configurable
- 📦 Configuration via variables d'environnement

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

2. Créez un fichier `.env` à la racine du projet :
```env
# Configuration du serveur
HOST=127.0.0.1
PORT=3000

# Configuration de la base de données
DATABASE_URL=postgres://postgres:postgres@localhost:5432/template_db

# Niveau de logging
RUST_LOG=info
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

## Structure du Projet

```
.
├── src/
│   ├── config.rs     # Configuration de l'application
│   ├── db.rs         # Gestion de la base de données
│   ├── handlers/     # Gestionnaires de routes
│   ├── models/       # Modèles de données
│   ├── routes/       # Définition des routes
│   └── main.rs       # Point d'entrée
├── .env             # Variables d'environnement
├── .gitignore
├── Cargo.toml
└── README.md
```

## Configuration

La configuration se fait via des variables d'environnement dans le fichier `.env` :

- `HOST` : Adresse du serveur (défaut: 127.0.0.1)
- `PORT` : Port du serveur (défaut: 3000)
- `DATABASE_URL` : URL de connexion à la base de données
- `RUST_LOG` : Niveau de logging (défaut: info)

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