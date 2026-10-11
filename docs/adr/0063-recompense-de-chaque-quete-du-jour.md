# ADR 0063 — Chaque quête du jour a sa récompense

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md), pour la seule récompense des quêtes du jour, qui ne donnaient que le tampon ; la carte de lecteur, son tampon, sa récompense au 7ᵉ tampon et les quêtes au long cours restent en vigueur
- **Modifie** : [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md), pour la récompense de la carte de lecteur pleine : son pli de lecteur à fascicule choisi devient un pli en attente de plus, comme celui de la quête d’adresse
- **Complète** : [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) (récompense à récupérer) ; [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md) (les sabliers et l’encre gagnés)

## Contexte

Avec l’[ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md), faire ses trois quêtes du jour ne rapporte qu’un tampon. La première récompense arrive au 7ᵉ tampon, au bout d’au moins sept jours. Le joueur qui termine ses quêtes n’a rien à récupérer ce jour-là, et le tester le constate : l’écran n’offre aucun bouton.

Le joueur doit sentir qu’une quête faite lui rapporte quelque chose le jour même, sans série à entretenir ni pression ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md), la boussole).

## Décision

### Une récompense par sorte de quête

Chaque quête du jour a une récompense annoncée d’avance, qui dépend de sa sorte. Valeurs de départ, à simuler :

| Sorte | Récompense |
|---|---|
| Table | 5 gouttes d’encre |
| Mémoire | 1 sablier |
| Adresse | 1 pli en attente de plus |

- La récompense suit la **sorte** de la quête tirée. Quand la quête de table cède sa place à une seconde quête de mémoire ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)), la récompense est celle de la mémoire.
- Le **pli en attente de plus** ajoute une charge à l’énergie ([ADR 0020](0020-deux-plis-en-attente-et-sabliers.md)) : un pli de plus à ouvrir comme les autres, dans le fascicule que le joueur choisit alors. Il n’y a rien à choisir au moment de la récupération. La recharge en cours continue.
- Quand l’énergie est **pleine**, la récupération est refusée (`403`) et la récompense attend ; le joueur ouvre un pli, puis la récupère. Rien n’est perdu.

### Une quête faite attend d’être récupérée

- Faire une quête **n’accorde rien** à ce moment-là, comme un jalon ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)) : le joueur récupère la récompense par un bouton « Récupérer » sur la quête.
- C’est une commande idempotente et transactionnelle, avec sa clé et la `version_etat` connue du client. Une quête non faite refuse la récupération (`403`) ; une quête déjà récupérée aussi (`409`).
- Une récompense qui attend **n’expire jamais**, même après minuit GMT : la quête disparaît de la page au jour suivant, pas sa récompense, qui reste récupérable tant que le joueur ne l’a pas prise.
- Le compteur Corail de la destination Quêtes compte aussi ces récompenses ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)).
- L’encre et les sabliers sont historisés avec le motif `quete_jour` ; ils s’ajoutent aux mouvements de la carte de lecteur sans les remplacer.

### La carte de lecteur suit la même règle

La récompense de la carte pleine au 7ᵉ tampon passe de « un pli de lecteur » à **un pli en attente de plus** : 20 gouttes d’encre, 1 sablier et un pli en attente de plus. Plus de choix de fascicule, plus de chances à afficher ; énergie pleine, la récupération est refusée et la carte attend.

### Ce qui ne change pas

- Le tampon du jour et l’arrivée de la récompense de la carte au 7ᵉ tampon.
- Le seuil de deux quêtes tirées pour poser un tampon, que cet ADR ne modifie pas.
- Aucune série, aucune flamme, aucun rappel.

## Options envisagées

### Une seule récompense, après la troisième quête

Écartée : elle ne donne rien aux jours où le joueur n’en fait qu’une ou deux, et repose le problème de l’ADR 0045 à plus petite échelle.

### Encre seule pour chaque quête

Plus simple pour l’économie, mais trois quêtes ne se distinguent plus, et le joueur n’a aucune raison d’y préférer l’une d’elles. Écartée par décision de l’équipe : une ressource par sorte.

### Un pli de lecteur dont le joueur choisit le fascicule

C’était la première rédaction : le pli de la carte de lecteur ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)), avec un sélecteur de fascicule et un tirage immédiat. Écartée par décision de l’équipe : pour une récompense quotidienne, un choix à chaque fois est une corvée ; une charge d’énergie se dépense quand le joueur veut, dans le fascicule qu’il veut.

### Accorder d’office, sans bouton

Écartée : l’[ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) veut que le joueur prenne ses récompenses, et pose la règle « toutes les récompenses sont affichées d’avance et attendent d’être récupérées ».

## Conséquences

### Positives

- Chaque quête rapporte le jour même, avec une récompense lisible d’avance.
- Le joueur qui ne fait qu’une quête reçoit quand même quelque chose.

### Négatives

- Le **pli en attente quotidien** s’ajoute à la recharge naturelle : jusqu’à un pli de plus par jour, donc environ 30 par mois, contre 4 par mois pour la carte de lecteur. L’énergie plafonne le stock, mais le rythme des fascicules ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)) et le plan financier sont à resimuler avant d’implémenter ; si besoin, la récompense d’adresse sera bornée ou remplacée.
- Un sablier par jour s’ajoute aux sabliers échangés ([ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md)) et gagnés par la carte.
- Une migration, un type de commande, un motif de mouvement et une origine de pli de plus.
- La page Quêtes, avec le bouton sur chaque quête du jour, reste à dessiner.

## Critères de réévaluation

- Les plis de lecteur quotidiens font avancer les fascicules nettement plus vite que prévu : réduire ou remplacer la récompense d’adresse.
- Les sabliers s’accumulent sans être utilisés : changer la récompense de mémoire.
- Les joueurs font les quêtes seulement pour la récompense et cessent de s’y intéresser : revoir leur contenu.
