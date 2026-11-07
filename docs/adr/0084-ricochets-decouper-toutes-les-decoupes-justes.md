# ADR 0084 — Ricochets : découper un mot de toutes ses façons, dont les compositions chaînées

- **Statut** : Proposé
- **Complète** : [ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md) (la lame, « Couper ici » et « Reprendre » ne changent pas ; la réponse peut porter plusieurs découpes) ; [ADR 0068](0068-ricochets-moins-de-touchers-pour-decouper-et-assembler.md) (la lame repart après chaque coupe, sur un leurre) ; [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (la maîtrise ne change pas) ; [ADR 0007](0007-etat-et-economie-autoritaires.md) (le serveur juge)

## Contexte

Le format *Découper* ([ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md)) attendait une seule découpe : celle de la composition préférée de la carte. L’application accepte pourtant déjà qu’un mot en ait plusieurs, et le joueur doit alors les trouver toutes (*écologiste* : *écologie · iste* et *éco · logiste*) ; le champ `autres_coupes` du manifeste porte ces découpes. Cette évolution a été livrée sans ADR, et le compilateur de contenu ne publiait jamais `autres_coupes` : sur le vrai catalogue, un mot se découpait toujours d’une seule façon.

Or le catalogue sait dire davantage. Une partie d’une composition est une unité, et cette unité peut être elle-même un mot composé : *étymologiste* est *étymologie · iste*, et *étymologie* est *étymo- · -logie*. Couper *étymologiste* en *étymo · logie · iste* est juste, et le joueur qui connaît le mot le sait.

## Décision

### Ce que le joueur fait

- Rien de nouveau dans le geste : il pose des coupes comme dans l’ADR 0053. Chaque découpe juste est gardée, puis le joueur passe à la suivante tant qu’il en reste ; la dernière découpe juste achève la question.
- Une découpe fausse arrête la question, fausse. Les découpes justes déjà trouvées ne comptent pas : une seule réponse compte par question ([ADR 0065](0065-ricochets-la-maitrise-se-perd-et-se-regagne.md)). Aucun crédit partiel.
- La commande de réponse reste unique, envoyée à la dernière découpe : une découpe seule part à plat, plusieurs ensemble. Le serveur fait foi.
- L’ordre des coupes, celui des découpes et les doublons ne comptent pas.

### Les découpes d’un mot

- La **découpe préférée** est celle de la composition préférée du mot : les frontières entre ses parties (`coupes`).
- Chaque partie qui est elle-même un mot composé (sa propre composition préférée) peut se couper à son tour. Chaque partie compte pour un choix indépendant (coupée ou non, et de chacune de ses façons) : le mot a **toutes les combinaisons** de ces choix pour découpes justes. *Étymologiste* en a deux, `[10]` puis `[5, 10]` ; un mot dont deux parties se coupent en a jusqu’à quatre.
- Une partie **élidée** dans le mot (*biologie* dans *biologiste* : segment `[0, 6)` pour un mot de huit lettres) est bornée par son segment : une coupe qui tomberait au-delà n’existe pas dans le mot.
- Une partie ne se coupe pas si elle n’a pas de composition préférée aux segments continus, ou si elle reviendrait sur un mot déjà en cours (aucune boucle).
- Plafonds de l’API : **quatre découpes** par mot et huit coupes par découpe. Au-delà de quatre découpes, **aucune découpe supplémentaire n’est publiée** : le joueur ne pourrait pas deviner lesquelles poser, mieux vaut la seule découpe préférée.

### Les données

- Le compilateur de contenu publie `autres_coupes` dans le bloc `ricochets` du manifeste, pour chaque mot qui a plus d’une découpe : des listes de positions triées, la préférée exclue. Des positions seulement, aucun texte copié. Le champ est facultatif : un manifeste sans `autres_coupes` se juge comme avant.
- Les coupes équivalentes à lettre partagée (`coupes_alternatives` : *hom · onyme* et *homo · nyme*) restent un autre champ du manifeste, que le contenu ne publie pas encore ; cet ADR n’y touche pas.

## Options envisagées

### Un arbre de coupes jugé coupe par coupe, une commande par coupe

Écartée. Une question qui s’achève au fil des coupes demande un contrat de réponse à état (« continuer », « terminé », « faux »), une commande qui n’écrit rien tant que la question reste ouverte, et un jugement côté client et côté serveur. L’application juge déjà des découpes complètes : réutiliser `autres_coupes` donne le même parcours pour le joueur sans nouveau contrat.

### Une question par niveau (« Recoupez *étymologie* »)

Écartée : la consigne dit alors qu’un morceau se coupe encore.

### Publier un sous-ensemble quand il y a plus de quatre découpes

Écartée : le joueur serait tenu de poser des découpes qu’il ne peut pas deviner.

## Conséquences

### Positives

- Les mots dont une partie est composée se découpent en entier ; le format mesure toute la chaîne.
- Aucun changement de contrat, de base ni de code applicatif : le contenu publie ce que l’application sait déjà juger. L’API et la PWA ne changent pas.
- Une donnée absente laisse le jeu comme avant.

### Négatives

- Le joueur repose les coupes communes (*étymo · logie · iste* demande `[10]` puis `[5, 10]`, soit la coupe `10` deux fois).
- Quatre découpes au plus : un mot dont plus de deux parties se coupent garde sa seule découpe préférée.
- La justesse d’une découpe dépend de la saisie des compositions du contenu : une partie sans composition préférée ne se coupe pas.

## Critères de réévaluation

- Si des mots réels dépassent quatre découpes, relever le plafond de l’API plutôt que de les taire.
- Si les joueurs trouvent pénible de reposer les coupes communes, passer à un parcours qui garde les coupes déjà posées d’une découpe à l’autre.
