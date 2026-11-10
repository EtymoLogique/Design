# ADR 0084 — Ricochets : découper un mot de toutes ses coupes, parties composées comprises

- **Statut** : Proposé
- **Complète** : [ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md) (la lame, « Couper ici » et « Reprendre » ne changent pas ; seul le nombre de coupes attendues change) ; [ADR 0068](0068-ricochets-moins-de-touchers-pour-decouper-et-assembler.md) (la lame repart après chaque coupe, sur un leurre) ; [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (la maîtrise ne change pas) ; [ADR 0007](0007-etat-et-economie-autoritaires.md) (le serveur juge)

## Contexte

Le format *Découper* ([ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md)) attendait les coupes de la composition préférée de la carte, sur un seul niveau. Mais une partie d’une composition est une unité, et cette unité peut être elle-même un mot composé : *étymologiste* est *étymologie · iste*, et *étymologie* est *étymo- · -logie*. Couper *étymologiste* en *étymo · logie · iste* est juste, et le joueur qui connaît le mot le sait.

Une première version de cet ADR faisait trouver chaque niveau comme une découpe distincte (`[10]`, puis `[5, 10]`), l’une après l’autre. Le joueur devait reposer les coupes communes, et il ne coupait jamais le mot « d’un coup » : le parcours était pénible.

## Décision

### Ce que le joueur fait

- Il coupe le mot **une seule fois**, du geste de l’ADR 0053, et pose **toutes** les coupes : *étymo · logie · iste*. La consigne dit combien de morceaux : « Coupez ce mot en 3 morceaux. »
- **Seule la coupe complète est juste.** *Étymologie · iste* seul est faux : on demande tous les morceaux. Une coupe en trop est fausse aussi.
- La commande de réponse reste unique, avec la liste des coupes. Le serveur fait foi. L’ordre des coupes et les doublons ne comptent pas.
- Au verdict, tous les morceaux élémentaires apparaissent, aux couleurs de leurs briques ; la leçon dit le mot en briques.

### Les coupes d’un mot

- Les coupes d’un mot sont les frontières de sa composition préférée, **plus celles de chaque partie qui est elle-même un mot composé** (sa propre composition préférée), de proche en proche. *Étymologiste* : `[5, 10]`.
- Une partie **élidée** dans le mot (*biologie* dans *biologiste* : segment `[0, 6)` pour un mot de huit lettres) est bornée par son segment : une coupe qui tomberait au-delà n’existe pas dans le mot. *Biologiste* : `[3, 6]`.
- Une partie ne se coupe pas si elle n’a pas de composition préférée aux segments continus, ou si elle reviendrait sur un mot déjà en cours (aucune boucle).
- Plafond de l’API : huit coupes par réponse. Au-delà, le contenu ne publie que les frontières du premier niveau : mieux vaut une coupe juste que des coupes que le joueur ne pourrait pas poser.

### Les données

- Le compilateur de contenu publie ces coupes dans `coupes`, dans le bloc `ricochets` du manifeste : des positions seulement, aucun texte copié. Le contrat et l’application ne changent pas.
- Les autres découpes justes d’un même mot (`autres_coupes` : *écologiste* en *écologie · iste* ou *éco · logiste*) et les coupes équivalentes à lettre partagée (`coupes_alternatives`) restent des champs du manifeste que l’application sait juger ; le contenu ne les publie pas encore et cet ADR n’y touche pas.

## Options envisagées

### Faire trouver chaque niveau comme une découpe distincte

Retenue d’abord, écartée : le joueur repose les coupes communes (`10`, puis `5` et `10`), valide plusieurs fois et ne coupe jamais le mot en une fois.

### Accepter toute coupe qui reste dans une décomposition valide

Écartée : *étymologie · iste* serait juste, et on ne mesurerait plus que le joueur a tout trouvé.

### Un arbre de coupes jugé coupe par coupe

Écartée : un contrat de réponse à état, une commande qui n’écrit rien tant que la question reste ouverte et un jugement dupliqué, pour un parcours que la coupe complète donne sans rien changer au contrat.

## Conséquences

### Positives

- Le joueur coupe le mot en une fois et le format mesure toute la chaîne.
- Aucun changement de contrat, de base ni de code applicatif côté serveur.
- Une donnée absente laisse le jeu comme avant.

### Négatives

- Un mot dont une partie se coupe devient plus difficile : seule la coupe complète compte, et le joueur qui connaît *étymologie · iste* sans connaître *étymo · logie* se trompe.
- La justesse dépend de la saisie des compositions : une partie sans composition préférée ne se coupe pas.
- Le verdict colore les morceaux par recalcul côté client (même règle que le compilateur) : une divergence se verrait aux couleurs, pas au jugement.

## Critères de réévaluation

- Si la part de réponses fausses des mots à plusieurs niveaux dépasse de plus de 15 points celle des autres mots sur un mois, ouvrir la question d’un crédit pour le premier niveau.
- Si des mots réels dépassent huit coupes, relever le plafond de l’API plutôt que de les taire.
