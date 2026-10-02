# ADR 0046 — Démarrage généreux : cadeau de bienvenue et quêtes initiales

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md), pour la seule source d’encre : le cadeau de bienvenue et les quêtes initiales en donnent aussi, une seule fois
- **Complète** : [ADR 0030](0030-reserve-de-depart-par-fascicule.md) (réserve de départ) ; [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md) (quêtes du jour) ; [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) (récompenses à récupérer)

## Contexte

Les premiers jours décident de la fidélité : dans les jeux de réflexion, environ un joueur sur trois revient le lendemain ([étude de marché](../etude-de-marche.md)). Or, après le tutoriel, ses graines ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)) et ses deux premiers plis, le nouveau joueur attend la prochaine charge, 12 h plus tard.

Nous voulons une première semaine riche, avec plusieurs découvertes par jour, puis un ralentissement qui se fasse de lui-même, sans changer la durée de la recharge ni créer de frustration calculée.

## Décision

### La recharge ne change pas

Une charge toutes les 12 h, deux au plus, pour tous les joueurs et à tout moment ([ADR 0020](0020-deux-plis-en-attente-et-sabliers.md)).

### Un cadeau de bienvenue

- Chaque profil reçoit une fois un **cadeau de bienvenue**, à récupérer dans la page Quêtes (« Récupérer », [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)) dès la fin du tutoriel.
- Valeurs de départ, à simuler :
  - 3 **plis de bienvenue** au contenu fixé, comme un pli de jalon, dont les briques sont choisies pour ouvrir des recettes, jamais une légendaire ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)) ;
  - 12 sabliers, soit un pli de plus ;
  - 40 gouttes d’encre, de quoi payer les trois premiers niveaux d’un indice (5 + 10 + 20).
- Les plis de bienvenue ne consomment pas d’énergie et ne comptent pour aucune garantie.
- Le catalogue déclare le cadeau. Ses briques appartiennent à des fascicules parus, et le validateur le vérifie.
- Dans l’interface, il s’appelle « Cadeau de bienvenue ».
- Il ne se présente pas dans la boutique : un cadeau affiché comme un achat à 0 € servirait surtout à habituer au geste d’achat.

### Des quêtes initiales

- Pendant la première semaine, des **quêtes initiales**, faites une seule fois, généreuses puis de moins en moins. Elles suivent les règles des quêtes du jour : un acte de réflexion, jamais un simple geste, et toujours faisables ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)).
- Elles s’affichent en tête de l’onglet « Journalières » jusqu’à ce qu’elles soient toutes faites, et ne comptent pas pour le tampon du jour.
- Exemples, avec des valeurs de départ à simuler :

| Quête initiale | Récompense |
|---|---|
| Découvrez 3 mots | 2 plis offerts |
| Découvrez un mot avec une transformation | 1 pli offert et 10 gouttes |
| Réussissez vos premiers ricochets sans faute | 3 sabliers |
| Remplissez votre première carte de lecteur | 1 pli de lecteur |

- Un **pli offert** est un pli ordinaire du fascicule que choisit le joueur, chances affichées, sans énergie et hors garanties. Le pli de lecteur est celui de la carte de lecteur ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)).

### Un ralentissement naturel, jamais un levier d’achat

- Les règles sont les mêmes pour tous et affichées dès le début. Seuls le cadeau et les quêtes initiales s’épuisent : le ralentissement est naturel et prévisible.
- La générosité du départ se règle sur la rétention mesurée ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)), jamais sur des achats. Garde-fou : jamais plus de deux jours sans découverte possible pour un joueur actif ([ADR 0012](0012-briques-rationnees.md)).
- Le ralentissement n’est pas personnalisé selon le comportement du joueur, aucune offre n’apparaît à un moment de frustration ([ADR 0018](0018-sabliers-et-boutique.md)), et aucune frustration n’est dosée pour pousser à l’achat : le game design refuse de « préparer des mécanismes trompeurs sous prétexte d’une monétisation future ».
- Avec plusieurs fascicules parus, les graines de chacun s’ajoutent au cadeau : la générosité du départ se simule avec la taille du catalogue à l’ouverture ([ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md)).

## Options envisagées

### Accélérer la recharge la première semaine

Écartée : le compte à rebours garde toujours la même durée. Une recharge plus rapide, puis plus lente, ferait deux règles à expliquer et une perte ressentie au 8ᵉ jour.

### Présenter le cadeau comme un achat gratuit dans la boutique

Écartée : il habituerait au geste d’achat, alors que la page Quêtes donne le même cadeau sans ambiguïté.

### Doser la frustration pour pousser à l’achat

Écartée, pour trois raisons : les principes du jeu l’excluent ([game design](../game-design.md), [ADR 0018](0018-sabliers-et-boutique.md)) ; il n’y a rien à acheter pendant la bêta, ni avant l’ouverture de la boutique, et la frustration ne mènerait qu’au départ ; la boutique rapporte environ 0,05 € par joueur actif et par mois ([plan financier](../plan-financier.md)), quand la rétention fait l’audience et le bouche-à-oreille.

### Des quêtes de bienvenue faites de gestes

Écartée : retourner une carte ou suivre un lien n’apprend rien qu’un joueur ne découvre seul. Le tutoriel couvre la table de fusion.

## Conséquences

### Positives

- Une première semaine riche, sans changer aucune règle du jeu.
- Un ralentissement honnête : rien n’est retiré, le cadeau s’épuise simplement.
- Les quêtes initiales font découvrir les ricochets et la carte de lecteur en les vivant.

### Négatives

- Plus de plis la première semaine : le rythme du premier fascicule d’un joueur est à resimuler.
- Le contenu du cadeau se choisit à chaque version du catalogue : ses briques doivent ouvrir des recettes.
- Un joueur qui récupère son cadeau tard en profite moins : la page Quêtes doit le signaler par son compteur Corail ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)).

## Critères de réévaluation

- La rétention au 1ᵉʳ et au 7ᵉ jour ne progresse pas : revoir le contenu du cadeau et des quêtes initiales.
- Plus de 10 % des nouveaux joueurs restent plus de deux jours sans découverte possible pendant leur première semaine.
- Une forte chute d’activité à la fin du cadeau.
