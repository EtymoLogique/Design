# ADR 0037 — Une nouvelle version de la PWA s’annonce par un écran de mise à jour

- **Statut** : Accepté
- **Portée** : dépôt App (PWA)
- **Complète** : [ADR 0031](0031-contrat-genere-et-outillage-de-la-pwa.md) (service worker produit par `vite-plugin-pwa`) et [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (PWA statique, API serverless)

## Contexte

Le service worker sert la coque de la PWA depuis son cache. Avec une mise à jour automatique en arrière-plan, un navigateur garde l’ancienne version jusqu’à un second chargement : après les déploiements de J2 et J3 en hors-prod, la nouvelle version n’était pas visible tout de suite, et on ne pouvait pas la valider directement.

La vérification doit tourner en continu. Or l’API est serverless et redescend à zéro instance sans trafic ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)) : une vérification qui l’appelle la tiendrait éveillée pour rien.

## Décision

### Détecter

- Le navigateur relit le service worker (`sw.js`) sur le CDN, là où la PWA est déposée en fichiers statiques et servie sans cache HTTP. L’API n’est jamais appelée pour cette vérification.
- La vérification a lieu **à l’ouverture de la page**, puis **toutes les 60 secondes** tant que l’onglet est visible et en ligne, au retour sur l’onglet et au retour du réseau.
- Chaque build change `sw.js`, qui porte l’empreinte de chaque fichier de la coque : toute nouvelle version déployée est détectée.

### Annoncer

- La nouvelle version s’installe en attente. Un **écran plein et opaque** passe alors devant la PWA et bloque le jeu, avec une seule action : « Mettre à jour ».
- Le bouton active la nouvelle version et recharge tous les onglets ouverts. Recharger la page sans cliquer ramène le même écran.
- La table en cours est sauvegardée dans IndexedDB et survit au rechargement.

### Présenter les nouveautés

- Cet écran servira aussi à afficher le **changelog** de la nouvelle version : ce qu’elle apporte au joueur, avant qu’il clique sur « Mettre à jour ».
- Le changelog sera livré avec la version, en fichier statique, et lu sur le CDN comme le reste de la vérification.

## Options envisagées

### Mise à jour automatique, sans écran

C’était le comportement initial (`registerType: 'autoUpdate'`). Écartée : l’ancienne version reste affichée jusqu’à un second chargement, et le joueur ne sait pas qu’il a changé de version.

### Rechargement forcé et silencieux

Il aligne la version tout de suite, mais recharge la page sous les yeux du joueur, sans explication, et ne laisse aucune place au changelog. Écarté.

### Interroger l’API sur sa version

Écarté : chaque vérification réveillerait l’API, qui doit pouvoir redescendre à zéro.

### Bandeau non bloquant

Écarté : le joueur continuerait à jouer sur une coque qui n’est plus déployée, contre un contrat d’API qui peut avoir changé.

## Conséquences

### Positives

- Une version déployée est visible dès l’ouverture de la page, ou dans la minute pour un onglet déjà ouvert : un déploiement en hors-prod se valide directement.
- La vérification ne coûte qu’une requête statique au CDN et ne réveille jamais l’API.
- Le joueur voit pourquoi la page change, et, avec le changelog, ce qu’apporte la nouvelle version.

### Négatives

- Le jeu est interrompu à chaque déploiement, même mineur, jusqu’au clic.
- Un onglet ouvert relit `sw.js` chaque minute : une requête de plus au CDN par joueur actif.
- Le délai de détection dépend du respect de `no-cache` par le CDN pour `sw.js`.

## Critères de réévaluation

- Des déploiements assez fréquents pour que l’interruption gêne les joueurs.
- Un coût de requêtes au CDN qui devient notable.
- Un CDN qui met `sw.js` en cache malgré les en-têtes.
