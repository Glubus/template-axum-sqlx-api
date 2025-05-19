# Template Axum SQLx API

Un template moderne pour créer des APIs REST avec Rust, Axum et SQLx.

## 🚀 Fonctionnalités

- Framework web Axum
- Base de données PostgreSQL avec SQLx
- Structure modulaire (models, handlers, routes)
- Gestion des erreurs avec thiserror
- Logging avec tracing
- Configuration via variables d'environnement
- Documentation API (Swagger/OpenAPI) - optionnel
- Docker Compose pour PostgreSQL et pgAdmin

## 📋 Prérequis

- Rust (dernière version stable)
- Docker et Docker Compose
- PostgreSQL (si vous n'utilisez pas Docker)
- Un compte GitHub

## 🛠 Installation

1. Fork ce repository sur GitHub :
   - Cliquez sur le bouton "Fork" en haut à droite de cette page
   - Cela créera une copie du projet dans votre compte GitHub

2. Clonez votre fork :
```bash
git clone https://github.com/VOTRE-USERNAME/template-axum-sqlx-api.git
cd template-axum-sqlx-api
```

3. Configurez les variables d'environnement :
   - Copiez le fichier `.env.template` en `.env` et éditer le fichier en question pour correspondre a la config postgres ou docker :
   ```bash
   cp .env.template .env
   ```
   - Ou créez manuellement un fichier `.env` à la racine du projet avec le contenu suivant :
   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/template_db
   HOST=127.0.0.1
   PORT=3001
   ```

4. Démarrez la base de données PostgreSQL avec Docker :
```bash
docker compose -f assets/compose.yml up -d
```

## 🏃‍♂️ Démarrage

1. Compilez et lancez le serveur :
```bash
cargo run
```

Le serveur sera accessible sur `http://localhost:3001`

## 📁 Structure du Projet

```
src/
├── models/          # Modèles de données
│   ├── mod.rs      # Module principal des modèles
│   └── common.rs   # Structures communes
├── handlers/        # Gestionnaires de routes
│   ├── mod.rs      # Module principal des handlers
│   └── common.rs   # Utilitaires communs
├── routes/         # Définition des routes
│   └── mod.rs      # Configuration du routeur
├── db.rs           # Gestion de la base de données
├── config.rs       # Configuration de l'application
└── main.rs         # Point d'entrée
```

## 🛠 Développement

### Ajouter un nouveau module

1. Créez les fichiers nécessaires :
```bash
touch src/models/votre_module.rs
touch src/handlers/votre_module.rs
touch src/routes/votre_module.rs
```

2. Exportez les modules dans leurs respectifs `mod.rs`

3. Ajoutez les routes dans `routes/mod.rs`

### Base de données

- Accédez à pgAdmin : http://localhost:5050
  - Email : admin@admin.com
  - Mot de passe : admin

- Pour arrêter la base de données :
```bash
docker compose -f assets/compose.yml down
```

## 🔧 Configuration

### Variables d'environnement

Le fichier `.env.template` contient toutes les variables d'environnement disponibles :

```env
# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/template_db

# Server Configuration
HOST=127.0.0.1
PORT=3001

# Logging Configuration
RUST_LOG=info

# Security (Optional)
# JWT_SECRET=your-secret-key
# JWT_EXPIRATION=3600

# CORS (Optional)
# CORS_ORIGIN=http://localhost:3000

# Rate Limiting (Optional)
# RATE_LIMIT_REQUESTS=100
# RATE_LIMIT_DURATION=60
```

Pour configurer votre environnement :
1. Copiez le fichier `.env.template` en `.env` :
```bash
cp .env.template .env
```
2. Modifiez les valeurs selon vos besoins
3. Les variables marquées comme "Optional" peuvent être décommentées si nécessaire

### Docker Compose

Le fichier `assets/compose.yml` configure :
- PostgreSQL 16
- pgAdmin 4
- Volumes persistants
- Healthchecks

## 📚 Documentation

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Rust Documentation](https://doc.rust-lang.org/book/)

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
1. Fork le projet
2. Créer une branche (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push sur la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

## 📝 License

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 👥 Auteurs

- Votre Nom - *Travail initial* - [Votre GitHub](https://github.com/votre-username) 