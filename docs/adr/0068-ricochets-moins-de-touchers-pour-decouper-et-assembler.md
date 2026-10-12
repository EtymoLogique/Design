# ADR 0068 — Ricochets : la lame n’attend jamais sur une coupe, et la seconde brique répond

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0067](0067-ricochets-assembler-un-mot-depuis-sa-definition.md), pour l’étape « Valider » d’*Assembler* et l’option « Un toucher sur la seconde brique répond », écartée par cet ADR et retenue ici ; [ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md), pour la place de la lame à l’arrivée et après chaque coupe
- **Complète** : [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (« un toucher répond » s’étend à *Assembler*) ; [ADR 0007](0007-etat-et-economie-autoritaires.md) (la commande de réponse reste unique et sous autorité serveur)

## Contexte

Deux retours de jeu sur les ricochets.

*Découper* ([ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md)) : la lame arrive vers 40 % du mot. Les mots à deux briques se coupent souvent près de ce point (*géo|graphie*, *bio|logie*) : la lame est déjà sur la bonne jointure, et le joueur valide sans avoir cherché. Le format ne mesure plus rien.

*Assembler* ([ADR 0067](0067-ricochets-assembler-un-mot-depuis-sa-definition.md)) : après deux briques choisies, un toucher de plus sur « Valider ». En usage, ce toucher est vécu comme une formalité : le joueur a déjà choisi, et l’assemblage montre sa réponse dès la première brique posée.

## Décision

### *Découper* : la lame arrive sur un leurre

- À l’arrivée de la question, la lame se pose sur une jointure **tirée au hasard, uniformément**, parmi les jointures libres qui **ne sont pas une coupe attendue**. Toutes les jointures du mot sont possibles, des bords comme du milieu.
- Après chaque coupe posée, la lame repart de la même façon : une jointure libre tirée au hasard, jamais une coupe attendue.
- Quand aucune jointure libre n’est un leurre (un mot de deux lettres, ou la dernière jointure libre est la seule coupe restante), la lame se pose sur une jointure libre quelconque.
- Le tirage se fait dans le client, sans rien envoyer : la place de la lame n’entre pas dans la réponse.

### *Assembler* : la seconde brique répond

- Il n’y a plus de bouton « Valider ». Dès que les deux emplacements de l’assemblage sont remplis, la réponse part et le verdict s’affiche.
- Tant qu’un seul emplacement est rempli, toucher la brique posée la retire, et choisir une autre brique de la même colonne la remplace.
- Le pied de la séance dit « Touchez un préfixe et un suffixe. » jusqu’à la réponse, comme « Touchez la bonne réponse. » pour les autres formats.
- La commande ne change pas : la paire (préfixe, suffixe), avec sa clé d’idempotence et la `version_etat` du client ; le serveur fait foi.

## Options envisagées

### Garder la lame vers 40 % du mot, mais la décaler si elle tombe sur la coupe

Écartée : la lame resterait près du milieu, à une lettre de la bonne jointure. Le joueur apprendrait vite qu’un seul pas suffit.

### Tirer la lame sur toutes les jointures, coupe comprise

Écartée : avec un mot de huit à dix lettres, une fois sur huit environ la lame serait déjà en place. C’est le défaut qu’on corrige.

### Garder « Valider » pour *Assembler*

Écartée : l’[ADR 0067](0067-ricochets-assembler-un-mot-depuis-sa-definition.md) craignait un doigt qui dérape sur le second toucher. Les deux colonnes sont distinctes, les cibles font 44 px, et la première brique posée se retire encore : le risque est faible, et le toucher de plus pèse sur chaque question.

## Conséquences

### Positives

- *Découper* demande toujours au moins un geste vers la bonne jointure : le format mesure de nouveau la connaissance du mot.
- *Assembler* passe de trois touchers à deux.

### Négatives

- Une seconde brique touchée par erreur donne une réponse définitive, sans appel.
- La place de la lame n’est plus la même d’une séance à l’autre pour un même mot : un test de l’interface doit fixer le hasard.

## Critères de réévaluation

- Si la part de réponses fausses à *Assembler* augmente de plus de 10 points après le retrait de « Valider », sur un mois d’usage, revoir l’espacement des colonnes ou revenir à une étape de validation.
- Si les joueurs signalent une lame « perdue » sur les mots longs (douze lettres et plus), la tirer à deux jointures au moins de la coupe plutôt que n’importe où.
