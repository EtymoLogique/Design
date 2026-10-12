# ADR 0065 — Ricochets : la maîtrise se perd sur une erreur et se regagne

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md), pour la maîtrise « définitive » (la carte « reste » maîtrisée)
- **Complète** : [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (le choix des cartes d’une séance) ; [ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md) (les légendaires restent hors de la jauge)

## Contexte

L’[ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) dit qu’une carte maîtrisée « reste » maîtrisée et revient « tous les 7 jours au plus ». Il ne dit pas ce qui arrive quand le joueur se trompe sur elle, ni si elle entre dans le choix des cartes d’une séance. Le modèle de données la rend même définitive (`maitrisee_le` ne s’efface plus).

Une maîtrise qu’aucune erreur ne peut défaire ne mesure rien : un joueur qui a oublié une carte la garde dans sa jauge. Et une carte maîtrisée qui ne revient jamais n’est plus révisée, ce qui contredit le pilier Transmission.

## Décision

### Une carte maîtrisée reste dans le tirage

- Une carte maîtrisée est **à revoir** quand sa prochaine révision est atteinte, au plus 7 jours après la précédente. Elle ne sort jamais définitivement du tirage.
- Une séance de 5 questions choisit ses cartes dans cet ordre : les cartes à revoir non maîtrisées, puis les découvertes récentes, puis les cartes maîtrisées à revoir. Elle compte **2 cartes maîtrisées au plus**, pour que les cartes en cours d’apprentissage restent prioritaires. Faute de mieux, une séance se complète avec ce qu’elle trouve : elle ne reste jamais vide.

### Une erreur fait descendre d’une marche

- Une mauvaise réponse à une carte à revoir la fait descendre d’**un niveau**, y compris quand elle était maîtrisée : de maîtrisée (niveau 3, trois bonnes réponses) elle passe au niveau 2. Elle revient le lendemain.
- Elle perd alors sa maîtrise : la jauge du fascicule baisse (« 18 / 24 » devient « 17 / 24 ») et le signe de maîtrise disparaît de la carte. Aucun signe particulier ne la remplace : une carte qui a perdu sa maîtrise est une carte à revoir comme une autre, sans reproche.
- La carte regagne la maîtrise par le chemin habituel : elle remonte au niveau 3, puis trois bonnes réponses sur trois jours différents. Le compte `bonnes_niveau_3` repart de 0 ; il n’y a pas de raccourci.
- Les paliers déjà atteints d’une quête au long cours le restent : une récompense atteinte ne se retire pas ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)).

### Inchangé

- Une mauvaise réponse ne coûte rien (ni exemplaire, ni énergie, ni encre) et s’affiche en Gris encre, jamais en rouge.
- Une réponse à une carte qui n’est pas à revoir ne change aucun niveau.
- Les légendaires restent hors de la jauge de maîtrise.
- Le codex n’affiche aucun nombre de cartes à revoir.

### Données et serveur

- `ricochet.maitrisee_le` redevient nullable et réversible : il est vidé quand la carte perd sa maîtrise, rempli quand elle la regagne. Une carte est maîtrisée si et seulement si `maitrisee_le` est rempli.
- La commande `ricochet` applique la règle côté serveur, qui fait foi ; le client suit la même règle pour choisir les cartes d’une séance.
- Le récapitulatif de fin de séance annonce une maîtrise perdue sans la dramatiser : « *géologie* revient demain. »

## Options envisagées

### Garder la maîtrise définitive

Écartée : la jauge cesserait de refléter ce que le joueur sait, et la carte ne reviendrait plus.

### Retomber au niveau 0 ou 1 après une erreur

Écartée : une seule erreur annulerait des semaines de révision, et l’erreur pèserait plus que le reste, contre l’esprit d’un jeu où se tromper ne coûte rien.

### Garder les trois bonnes réponses après une perte

Écartée : la carte redeviendrait maîtrisée sur une seule bonne réponse, le jour même d’une erreur : la maîtrise ne prouverait plus rien.

### Un signe particulier pour une carte qui a perdu sa maîtrise

Écartée : un signe de plus à dessiner, et un reproche de plus sur la carte. Le signe de maîtrise qui disparaît suffit.

### Aucun plafond de cartes maîtrisées par séance

Écartée : un joueur qui a beaucoup de cartes maîtrisées ne verrait plus que ces cartes, au détriment des découvertes récentes.

## Conséquences

### Positives

- La jauge de maîtrise reste honnête : elle baisse quand le joueur oublie.
- Les cartes maîtrisées continuent d’être révisées, sans prendre la place des cartes en cours d’apprentissage.
- Aucune règle nouvelle à apprendre : le niveau descend d’une marche, comme pour toute carte.

### Négatives

- Une jauge qui baisse peut décevoir : le ton du récapitulatif doit rester sans reproche.
- Les cartes maîtrisées doivent être comptées comme un état courant, plus comme un acquis : une migration du champ `maitrisee_le` des joueurs existants n’est pas nécessaire (les valeurs remplies restent valides).
- Quêtes au long cours sur la maîtrise : leur progression suit l’état courant, leurs paliers atteints restent acquis.

## Critères de réévaluation

- Beaucoup de joueurs perdent des maîtrises sans les regagner : revoir la marche (ou le nombre de bonnes réponses à refaire).
- Les séances paraissent répétitives : revoir le plafond de 2 cartes maîtrisées.
- Les tests montrent que la baisse de jauge est vécue comme une punition.
