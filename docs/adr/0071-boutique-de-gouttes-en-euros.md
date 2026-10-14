# ADR 0071 — Boutique en euros : seules les gouttes s’achètent

- **Statut** : Proposé
- **Portée** : après le MVP. Le MVP reste sans paiement tant que le présent ADR n’est pas accepté.
- **Remplacerait partiellement**, une fois accepté : [ADR 0018](0018-sabliers-et-boutique.md), pour ce qui se vend (des lots de sabliers en euros), l’encre « non achetable et non convertible en argent », la monnaie premium écartée, le remboursement d’un lot sans sablier utilisé et la renonciation avant le premier usage ; [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md), pour la section « Achats » de la boutique, qui renvoie aux lots de sabliers de l’ADR 0018. Le reste de ces ADR resterait en vigueur, dont les garde-fous de l’ADR 0018 (aucune offre limitée, aucune relance, boutique désactivable par territoire, autorité du serveur) et l’échange de l’encre contre des sabliers de l’ADR 0039.
- **Complète** : [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (le compte est requis pour acheter), [ADR 0072](0072-prestataire-et-parcours-de-paiement.md) (prestataire et parcours de paiement), [ADR 0009](0009-contenu-et-equilibrage.md) (les prix en gouttes restent des paramètres d’équilibrage)

## Contexte

L’ADR 0018 prévoit une boutique qui vend des lots de sabliers en euros, sans monnaie intermédiaire. Depuis, l’ADR 0039 a fait des sabliers un objet qui s’échange contre 10 gouttes, et l’ADR 0051 a mis les indices à 4, 8 et 16 gouttes. L’encre est devenue la ressource unique avec laquelle le joueur paie tout ce qui l’aide : sabliers et indices. L’écran Boutique de la PWA annonce déjà, verrouillée, une section « Des gouttes, en euros ».

Le 14 octobre 2026, le produit a décidé que **seules les gouttes s’achèteraient en euros**. Une seule chose à vendre, une seule ressource à expliquer, et aucun objet du jeu qui existerait en deux versions, gagnée et achetée.

Ce choix fait des gouttes une **monnaie de jeu achetable**. Le réseau européen des autorités de protection des consommateurs (CPC) a publié le 21 mars 2025 sept principes sur ces monnaies ([PDF de la Commission européenne](https://commission.europa.eu/document/download/8af13e88-6540-436c-b137-9853e7fe866a_en?filename=Key+principles+on+in-game+virtual+currencies.pdf)). Ils ne lient pas les autorités, mais disent comment elles lisent la directive sur les pratiques commerciales déloyales, la directive sur les droits des consommateurs et la directive sur les clauses abusives. Tant que l’encre ne se gagnait qu’en jouant, ils ne s’appliquaient pas ; dès qu’elle s’achète, ils s’appliquent. Les risques qu’ils visent sont ceux que l’ADR 0018 opposait à la monnaie premium : prix masqué et soldes inutilisables. Le présent ADR les traite un par un.

## Décision

### Ce qui se vend

- La boutique vend des **gouttes d’encre en euros**, et rien d’autre. Les sabliers, les indices et toute autre ressource continuent de s’obtenir en gouttes, par les gestes existants.
- Les gouttes achetées et les gouttes gagnées sont **la même encre**, au même solde. Elles achètent les mêmes choses aux mêmes prix.
- La boutique ne vend **jamais un pli, une brique, une chance ni un tirage**, directement ou par l’encre. L’encre n’achète aujourd’hui que des sabliers (du temps) et des indices (de l’aide). Toute nouvelle dépense en gouttes demandera un ADR qui vérifie ce point.
- Tout ce que l’encre achète s’obtient aussi en jouant : le jeu reste complet sans payer, et la vitrine « gratuit, sans publicité » reste vraie.
- Dans l’interface, on parle toujours de **gouttes d’encre**, jamais de « monnaie » ni de « gemmes » (glossaire du design system). « Monnaie de jeu » n’apparaît que dans les textes juridiques.

### Prix et quantités

| Paramètre | Valeur initiale |
|---|---|
| Prix de la goutte | **1 € les 60 gouttes**, sans remise de quantité |
| Quantité achetée | au choix du joueur, par pas de **60 gouttes** |
| Achat minimum | **120 gouttes, soit 2 €** : sous ce montant, les frais fixes de paiement prennent plus de 10 % |
| Achat maximum | le reste du plafond mensuel |
| Plafond mensuel par compte | **20 €** par mois civil, abaissable à tout moment, relevable après un délai de 7 jours |
| Achats en euros d’un compte neuf | **désactivés** |

- Le prix est **unique** : 60 gouttes valent toujours 1 €, quelle que soit la quantité. Il n’y a donc qu’un prix de référence à afficher, et aucun lot « plus avantageux » ne pousse à acheter plus.
- À ce prix, un sablier vaut environ 0,17 €, comme dans le lot de 12 sabliers à 1,99 € de l’étude de marché, et les indices valent environ 0,07 €, 0,13 € et 0,27 €.
- Le joueur choisit la quantité : il n’est jamais forcé d’acheter un lot qui ne correspond à rien. La feuille d’achat propose d’emblée la quantité qui couvre **ce qu’il cherchait à obtenir**, arrondie au pas supérieur, quand l’achat part d’un manque (sablier ou indice).
- Ces valeurs sont des paramètres d’équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)). Les prix sont TTC et affichés comme tels.

### Le prix en euros, partout où l’on paie en gouttes

- Partout où un prix est affiché en gouttes (sablier, indice, feuilles d’échange), son **équivalent en euros** est affiché à côté, au prix unique ci-dessus. Par exemple : « 10 gouttes · 0,17 € ».
- Cet équivalent s’affiche **même si le joueur paie avec des gouttes gagnées**, et même s’il n’a jamais rien acheté. C’est une référence objective, pas une facture.
- L’équivalent s’affiche tant que la boutique en euros est ouverte dans le territoire du joueur. Boutique fermée, il disparaît.

### Le joueur choisit d’acheter, et le titulaire de la carte garde la main

- Les achats en euros demandent un **compte** ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)).
- Ils sont **désactivés par défaut** sur chaque compte. Le joueur les active dans les Réglages, par un geste explicite qui rappelle le plafond mensuel. Le jeu ne promet aucun contrôle d’âge, qu’une PWA ne peut pas tenir.
- Le plafond mensuel s’applique au **compte**, toutes cartes confondues. Le baisser prend effet tout de suite ; le relever ne prend effet qu’après 7 jours.
- Aucune proposition d’achat n’apparaît au moment d’une révélation, d’un échec ou d’un pli ([ADR 0018](0018-sabliers-et-boutique.md)). L’entrée « Acheter des gouttes » n’apparaît qu’à deux endroits : la boutique, et la feuille qui constate un manque de gouttes, en lien secondaire après l’échange et le temps d’attente.

### Rétractation des gouttes achetées

- Une monnaie de jeu n’est pas un contenu numérique au sens des principes CPC (principe 5) : on ne peut pas faire renoncer le joueur à son droit de rétractation sur les gouttes elles-mêmes.
- Pendant **14 jours** après un achat, le joueur peut se rétracter. Il récupère alors le prix des **gouttes de cet achat qui restent inutilisées**, au prix unique, et ces gouttes quittent son solde.
- **Ordre de dépense** : quand le joueur dépense de l’encre, les **gouttes gagnées partent d’abord**, puis les gouttes achetées, de la plus ancienne à la plus récente. C’est l’ordre qui laisse le plus de gouttes remboursables au joueur.
- La rétractation se fait par un **geste en ligne** dans les Réglages du compte, sans écrire à personne. Elle déclenche le remboursement par le prestataire ([ADR 0072](0072-prestataire-et-parcours-de-paiement.md)).
- Au-delà de 14 jours, les gouttes achetées restent au joueur, sans date d’expiration.

### Ce que l’encre achète : un contenu numérique

- Un sablier ou un indice payé en gouttes est un **contenu numérique** fourni tout de suite. Pour ces achats, l’exception à la rétractation s’applique : la première fois que le joueur dépense des **gouttes achetées**, il donne son accord pour une exécution immédiate et reconnaît perdre son droit de rétractation sur ce qu’il obtient. Ce consentement tient en une case, distincte du bouton qui achète, et reste valable pour les dépenses suivantes jusqu’à ce que le joueur le retire.
- Tant que le joueur ne dépense que des gouttes gagnées, aucun consentement n’est demandé.

### Clarté des conditions

- Les CGV disent en clair : ce qu’est une goutte, son prix, qu’elle ne se convertit jamais en argent et ne se cède pas, la rétractation et l’ordre de dépense ci-dessus.
- **Changer le prix en gouttes** d’un sablier ou d’un indice (paramètre d’équilibrage) ne retire jamais au joueur ce qu’il a déjà obtenu. Une hausse est annoncée au moins 14 jours à l’avance dans le jeu ; une baisse s’applique tout de suite.
- Le jeu ne se réserve pas le droit de retirer des gouttes à un joueur, sauf en cas de rétractation ou d’impayé sur l’achat qui les a fournies.

### Points soumis à la revue juridique

Ces points restent à confirmer par le conseil prévu en M6 (revue de la boutique) :

- **Support durable.** Le droit de la vente à distance exige une confirmation du contrat sur un support durable, qui mentionne la renonciation à la rétractation. Le jeu ne garde aucune adresse ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)). Pour l’achat de gouttes, le reçu du prestataire ([ADR 0072](0072-prestataire-et-parcours-de-paiement.md)) est la piste retenue. Pour les sabliers et indices payés en gouttes achetées, la piste à soumettre est un **historique des achats dans le jeu**, consultable et exportable, qui consigne chaque consentement.
- **Chaîne d’échanges.** Le principe 2 déconseille plusieurs échanges successifs avant un achat. Gouttes, sabliers, puis temps forment une chaîne de deux échanges. Le sablier est un objet consommable plutôt qu’une monnaie ; à faire confirmer, et à défaut, vendre le temps directement en gouttes.
- **Coffres à butin.** L’encre achète du temps, donc un accès plus rapide à des plis tirés au hasard. La revue par territoire de l’ADR 0018 (Belgique, Pays-Bas) s’applique telle quelle.
- **Bouton de commande** portant « Commander avec obligation de paiement » ou une formule équivalente, et médiateur de la consommation désigné dans les CGV.

## Options envisagées

### Vendre des lots de sabliers (ADR 0018)

Écartée par le produit : elle crée deux circuits pour une même ressource (sabliers gagnés par l’encre et sabliers achetés), et elle ne couvre pas les indices. Elle avait l’avantage d’échapper aux principes sur les monnaies de jeu : ce coût est assumé et traité ci-dessus.

### Lots fixes avec remise de quantité

Écartée : les principes CPC déconseillent les lots qui ne tombent pas sur les prix des objets, et refusent de faire de la remise la base du prix affiché. Un prix unique supprime les deux problèmes et la question « quel lot est le plus avantageux ».

### Gouttes dépensées dans l’ordre d’arrivée

Écartée : plus simple à suivre, mais les gouttes achetées partiraient avant les gouttes gagnées, et le droit de rétractation s’éteindrait plus vite. L’ordre retenu favorise le joueur.

### Achats activés par défaut

Écartée : le principe 7 recommande de désactiver par défaut la dépense en argent réel dans un jeu qui n’est pas réservé aux adultes, et le public scolaire visé compte des mineurs.

### Pas de prix en euros à côté des prix en gouttes

Écartée : c’est le premier principe CPC, et c’est ce qui garde le prix réel visible, comme le voulait l’ADR 0018.

## Conséquences

### Positives

- Une seule chose à vendre, une seule ressource à expliquer. Les gestes du jeu (échange, indices) ne changent pas.
- Le prix réel reste visible partout, et le prix unique écarte toute pression de lot.
- La rétractation et l’ordre de dépense protègent le joueur au-delà de ce que l’ADR 0018 prévoyait.
- Le titulaire de la carte garde la main : achats désactivés par défaut, plafond mensuel modulable.

### Négatives

- Le serveur doit suivre, pour chaque achat, les gouttes encore inutilisées, et appliquer l’ordre de dépense à chaque dépense d’encre.
- Tous les écrans qui affichent un prix en gouttes doivent afficher l’équivalent en euros.
- Les gouttes deviennent une monnaie de jeu achetable : la boutique entre dans le champ des actions coordonnées des autorités européennes, et du futur Digital Fairness Act.
- Un consentement de plus à expliquer, la première fois qu’on dépense des gouttes achetées.
- Le plan financier et l’étude de marché, écrits pour des lots de sabliers, sont à reprendre.

## Critères de réévaluation

- La revue juridique juge la chaîne gouttes, sabliers, temps contraire au principe 2 : vendre le temps directement en gouttes.
- Plus de 5 % des achats sont suivis d’une rétractation : revoir la feuille d’achat et la quantité proposée.
- Des comptes atteignent le plafond mensuel chaque mois : baisser le plafond par défaut et revoir les critères de l’ADR 0018.
- Le Digital Fairness Act, une fois proposé, encadre les monnaies de jeu.
- Les frais de paiement baissent assez pour un achat minimum sous 2 €.
