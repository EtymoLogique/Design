# ADR 0031 — Contrat de l'API tiré des types Rust, PWA outillée par Vite

- **Statut** : Proposé
- **Portée** : dépôt App (PWA et API)
- **Précise** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (« un schéma unique, dont on génère les types TypeScript du front ») et [ADR 0001](0001-pwa-responsive.md) (PWA)

## Contexte

L'[ADR 0024](0024-architecture-logicielle-et-hebergement.md) exige que les types des commandes et de l'artefact soient décrits par un **schéma unique**, dont on génère les types TypeScript de la PWA. Il ne dit ni où vit ce schéma, ni comment il est produit. Il choisit React et un service worker Workbox sans fixer l'outillage.

Deux voies existent : écrire le schéma à la main, puis en générer le Rust et le TypeScript ; ou tirer le schéma des types Rust, qui portent déjà la validation (`deny_unknown_fields`, formats, bornes).

## Décision

### Les types Rust sont la source de vérité

- Les requêtes et réponses de l'API, et les formats de l'artefact lus par la PWA, sont des types Rust annotés (`serde`, `utoipa`).
- La commande `etymo-api openapi` en tire `contrat/openapi.json` (OpenAPI 3.1), versionné dans le dépôt.
- La PWA en génère ses types TypeScript (`openapi-typescript`), versionnés eux aussi.
- La CI régénère les deux fichiers et **échoue s'ils diffèrent** de ceux du dépôt : aucun contrat ne dérive en silence.

### La PWA est compilée par Vite

- Vite, React et TypeScript strict ; `vite-plugin-pwa` produit le service worker Workbox et le manifeste d'application.
- Le service worker précache la coque seulement. Les fichiers hachés du catalogue sont mis en cache à la première lecture ; l'index est revalidé à chaque lancement ([ADR 0025](0025-medias-statiques-et-publication-programmee.md), [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)).
- Tant que le CDN ne pose pas d'en-têtes, la CSP stricte est posée dans la page à la compilation, avec les seules origines de l'API et du catalogue.

## Options envisagées

### Schéma écrit à la main, code généré des deux côtés

Un seul fichier neutre, mais la génération de Rust depuis OpenAPI produit des types moins précis que ceux écrits à la main, et la validation se répartirait entre le schéma et le code. Écarté.

### Types TypeScript écrits à la main

Écarté : c'est précisément la dérive que l'[ADR 0024](0024-architecture-logicielle-et-hebergement.md) veut éviter.

### Create React App, Next.js

Create React App n'est plus maintenu. Next.js suppose un serveur pour ce qui fait sa valeur, alors que la PWA est servie en fichiers statiques. Écartés.

## Conséquences

### Positives

- Un seul endroit pour changer une commande : le type Rust, qui la valide aussi.
- Une dérive entre l'API et la PWA casse la CI, pas la production.

### Négatives

- Deux fichiers générés à régénérer et à versionner à chaque changement du contrat.
- Le schéma dépend des capacités d'`utoipa` pour décrire les types Rust.

## Critères de réévaluation

- Un second client (application native, outil éditorial) a besoin du contrat sans la chaîne Rust.
- `utoipa` ne sait plus décrire fidèlement un type du contrat.
