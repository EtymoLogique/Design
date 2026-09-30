# ADR 0042 — Page Quêtes et récompenses de jalon à récupérer

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md), pour le moment où le serveur accorde un jalon et pour l’endroit où ses récompenses s’affichent ; le format des jalons et le prix des indices restent en vigueur
- **Complète** : [ADR 0007](0007-etat-et-economie-autoritaires.md), pour la commande de récupération ; [ADR 0034](0034-ecran-dans-l-adresse.md), pour l’adresse de la nouvelle destination

## Contexte

L’ADR 0038 accorde un jalon dans la transaction de la découverte qui l’atteint : le pli de jalon et les sabliers arrivent dans la réserve sans geste du joueur. À l’usage, le cadeau passe inaperçu. Le joueur voit un compteur de sabliers bouger ou une brique nouvelle apparaître sans savoir d’où elle vient, et il ignore que des jalons existent tant qu’il n’ouvre pas l’onglet « Objectifs » du codex.

Les jalons n’ont rien à voir avec la collection : ils récompensent une progression. Les ranger dans le codex les cache, et le game-design prévoit d’autres objectifs (défi quotidien, familles, langues) qui auront besoin du même endroit.

## Décision

### Une destination « Quêtes »

- La PWA gagne une destination **Quêtes**, après le codex, dans la navigation principale (`#/quetes`).
- Elle a deux onglets :
  - **Journalières** (`#/quetes/journalieres`) : les quêtes journalières, annoncées comme « bientôt disponibles ». Leur contenu fera l’objet d’un ADR ; aucune n’existe dans le MVP.
  - **Fascicules** (`#/quetes/fascicules`) : les jalons de chaque fascicule paru, leur progression et leur récompense.
- L’onglet Fascicules se filtre par fascicule avec **le même sélecteur que le codex** (jaquette, nom, complétude) ; le filtre est dans l’adresse (`?fascicule=0`).
- Sans onglet dans l’adresse, la page ouvre Fascicules si une récompense attend le joueur, Journalières sinon.
- Les jalons **quittent** l’onglet « Objectifs » du codex, qui garde les familles et les langues et renvoie aux quêtes.

### Un jalon atteint attend d’être récupéré

- Le serveur **marque** un jalon atteint dans la transaction de la découverte qui l’atteint, ou à la lecture de l’état si le jalon est publié après coup. Il n’accorde **rien** à ce moment-là.
- Le joueur récupère la récompense par un bouton « Récupérer » dans les quêtes. C’est une commande idempotente et transactionnelle (`POST /jalon`), avec sa clé et la `version_etat` connue du client : elle crédite le pli de jalon et les sabliers en une fois.
- Le pli de jalon suit toujours l’ADR 0038 : un exemplaire de chaque brique, de l’encre si la réserve est pleine, sans énergie, hors garantie et hors filet. Il est historisé comme un pli d’origine `jalon`, à la date de la récupération.
- Une récompense se récupère **une seule fois** : `403` si le jalon n’est pas atteint, `409` s’il est déjà récupéré.
- Une récompense qui attend **n’expire jamais** et ne se perd pas : pas de minuterie, pas de rappel pressant.
- Les jalons accordés d’office avant cette décision sont considérés comme récupérés.

### Faire savoir que les récompenses existent

- Un **compteur Corail** signale les récompenses qui attendent : sur la destination Quêtes, sur l’onglet Fascicules et sur la carte du fascicule concerné. Le Corail garde son sens : quelque chose de nouveau attend le joueur.
- La découverte qui atteint un jalon le dit sur la table et propose « Récupérer la récompense ».
- La récompense de chaque jalon, atteint ou non, reste **affichée d’avance** : jamais un tirage.

## Options envisagées

### Garder l’octroi automatique et ajouter une annonce

Écartée : une annonce se ferme sans être lue, et le joueur ne relie toujours pas le cadeau à son origine. Le geste de récupérer rend la récompense visible et lisible.

### Laisser les jalons dans le codex

Écartée : le codex est la collection, pas une liste d’objectifs. Les quêtes journalières et les autres objectifs à venir auraient encore besoin d’une autre place.

### Récompenses qui expirent

Écartée : la boussole refuse la pression et la peur de manquer. Une récompense gagnée reste acquise.

## Conséquences

### Positives

- Le joueur sait d’où viennent ses briques et ses sabliers, et que les jalons existent.
- Une place est prête pour les quêtes journalières et les autres objectifs.
- La récupération reste une commande du serveur, idempotente, comme les autres ([ADR 0007](0007-etat-et-economie-autoritaires.md)).

### Négatives

- Un geste de plus pour obtenir une récompense ; un joueur qui ne va jamais dans les quêtes n’en reçoit aucune, même si le compteur le lui rappelle.
- Une sixième destination dans la navigation : sur mobile, elle passe dans le menu.
- Un onglet « bientôt disponible » tant que les quêtes journalières ne sont pas conçues.
- Les maquettes I-18 montrent encore les jalons dans le codex ; la page Quêtes reste à dessiner dans l’inventaire.

## Critères de réévaluation

- Des récompenses atteintes et jamais récupérées chez une part notable des joueurs : rendre l’accès plus direct.
- Des quêtes journalières conçues : un ADR fixe leur contenu, leurs récompenses et leur renouvellement.
- Une navigation mobile trop chargée : regrouper des destinations.
