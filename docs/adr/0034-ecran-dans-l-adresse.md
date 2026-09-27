# ADR 0034 — L'écran de la PWA se lit dans son adresse

- **Statut** : Accepté
- **Complète** : [ADR 0001](0001-pwa-responsive.md) (PWA) et la règle « changer d'onglet conserve l'état de chaque écran » des [interfaces](../interfaces.html)

## Contexte

Un rechargement de la PWA ramenait toujours à la table : l'onglet, les filtres du codex et la carte agrandie étaient perdus. Le bouton Précédent du navigateur ne ramenait pas non plus à l'écran d'avant. Or la PWA est servie en fichiers statiques ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)), sans réécriture d'adresse côté serveur.

## Décision

- L'écran courant est tenu dans le **fragment** de l'adresse : `#/table`, `#/codex`, `#/plis`, `#/reglages`.
- Dans le codex, le fragment porte aussi la **carte agrandie**, par son identifiant opaque, et la vue : filtre par type, tri et fascicule (`#/codex/unt_…?type=mot&tri=az&fascicule=0`).
- **Changer d'onglet ou ouvrir une carte** ajoute une étape à l'historique ; changer un filtre ou le tri la remplace.
- Une adresse inconnue ou forgée retombe sur la table ; un identifiant de carte invalide est ignoré. L'adresse ne porte jamais de texte du catalogue.
- La table en cours, la réserve confirmée et les réglages restent gardés sur l'appareil ([ADR 0007](0007-etat-et-economie-autoritaires.md)) : l'adresse ne décrit que ce qu'on regarde.

## Options envisagées

### Chemins d'adresse (`/codex/unt_…`)

Plus lisibles, mais chaque chemin devrait renvoyer `index.html` : l'Object Storage et le CDN n'ont pas de réécriture simple. Écarté pour le MVP.

### État d'écran dans le stockage local

Il survivrait au rechargement, mais ne se partagerait pas et ignorerait le bouton Précédent. Écarté.

## Conséquences

### Positives

- un rechargement ou le bouton Précédent ramène au même écran ;
- une carte du codex se désigne par une adresse, sans rien dévoiler de son texte.

### Négatives

- adresses avec un `#`, moins lisibles ;
- un identifiant de carte figure dans l'adresse : il reste opaque, mais il circule avec elle.

## Critères de réévaluation

- Le CDN permet de servir `index.html` pour tout chemin de la PWA.
- Des adresses de cartes partagées révèlent une carte avant sa découverte.
