# ADR 0053 — Découper : glisser une lame, puis valider

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md), pour le format Découper : quatre découpages proposés, option « Découper en touchant le mot » écartée, et « un toucher répond » appliqué à ce format
- **Complète** : [ADR 0007](0007-etat-et-economie-autoritaires.md) (la commande de réponse reste unique et sous autorité serveur)

## Contexte

L’[ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) décrit le format Découper comme un choix entre quatre découpages d’un mot, et écarte le découpage « en touchant le mot » : sur mobile, les lettres sont trop étroites pour une cible de 44 px, et un mot en trois parties a plusieurs coupes justes.

Le client livre pourtant une variante : chaque jointure du mot est un bouton invisible, et la dernière coupe envoie la réponse. Au toucher, elle pose trois problèmes :

- la cible fait environ 20 px de large, la moitié des 44 px que le design système exige ;
- l’écartement du mot au survol n’existe pas au doigt : le joueur ne voit rien avant de couper, et le doigt cache la jointure visée ;
- la réponse part à la dernière coupe, sans appel : un doigt qui dérape donne une réponse fausse et définitive.

Découper est le seul format où le joueur produit la réponse au lieu de la reconnaître. Il faut garder ce geste et le rendre fiable au toucher.

## Décision

### Le geste

Sur un pointeur grossier (écran tactile) :

- **Une lame** se pose sur le mot dès l’arrivée de la question, en pointillé : la coupe n’existe pas encore. Toute la largeur du mot est une zone de glissement ; la lame se cale sur la jointure la plus proche du doigt, jamais entre deux lettres.
- **Une poignée** de 44 × 44 px, en forme de brique, Corail, se trouve sous le mot, au bout de la lame : le doigt ne cache aucune lettre.
- **Le mot s’écarte** à l’endroit de la lame, de 0,2 em de chaque côté. L’écart suit la lame de jointure en jointure.
- **Les boutons ‹ et ›** (chacun 44 px au moins) avancent la lame d’une jointure libre ; au clavier, les flèches ← et → font de même. Chaque glissement a donc une alternative.
- **« Couper ici »** pose la coupe : l’écart devient fixe, un trait plein en Encre le marque, et la lame repart vers la jointure libre suivante. Deux coupes ne se posent jamais au même endroit.
- **« Reprendre »** retire la dernière coupe, sans limite et sans pénalité, tant que la réponse n’est pas validée.
- **« Valider »** prend la place de « Couper ici » quand toutes les coupes sont posées : leur nombre est celui des segments du mot moins un. La consigne ne change pas (« Où ce mot se coupe-t-il ? », « Coupez ce mot en 3 morceaux. »).

### La réponse

- Rien n’est envoyé ni jugé avant « Valider ». La commande de réponse est unique : la liste ordonnée des coupes, avec sa clé d’idempotence et la `version_etat` connue du client ; le serveur fait foi.
- Le verdict garde son déroulé : le mot s’écarte aux coupes, un contour de brique se trace autour de chaque morceau, puis la brique se remplit de la couleur de son type. Une brique garde son trait d’union.
- Une coupe fausse se lit sans nouvelle teinte, comme toute mauvaise réponse des ricochets : morceaux en Sable, contour pointillé en Encre secondaire, secousse de 500 ms, rond barré et texte « Pas cette fois. ». Jamais de rouge. La bonne découpe est donnée au verdict, avec les briques élémentaires.
- Une seule réponse compte par question, quel que soit le nombre de coupes posées puis reprises avant la validation.

### Pointeur fin (souris)

- La lame suit le survol ; la poignée et les boutons ‹ › sont masqués.
- Un clic pose la coupe, comme aujourd’hui. Le bouton principal n’apparaît qu’une fois toutes les coupes posées, avec « Valider » : le verdict ne dépend pas de l’appareil.

### Ce qui ne change pas

- Les autres formats gardent « un toucher répond, sans bouton Valider » : seul Découper a une étape de validation.
- Les données employées (les segments de la composition préférée), la maîtrise, les quêtes et l’absence de chronomètre ne changent pas.
- Mouvement réduit : l’écart et la lame sautent d’une jointure à l’autre sans transition, et le verdict affiche directement son état final.

## Options envisagées

### Garder le découpage en touchant directement la jointure

C’est la variante livrée aujourd’hui. Écartée : une cible de 20 px, aucun aperçu au doigt, et une réponse définitive au dernier toucher.

### Revenir aux quatre découpages proposés (ADR 0044)

Écartée : le joueur reconnaît un découpage au lieu de le produire, et les leurres, qui déplacent une coupe d’une ou deux lettres, se devinent à l’œil sans connaître les briques. Le geste de découper est ce que le format veut apprendre.

### Une loupe au-dessus du doigt

Écartée : elle ajoute un second objet à lire, alors que l’écart du mot montre déjà les deux morceaux à l’endroit visé, et elle demande de gérer sa position près des bords de l’écran.

### Valider à la dernière coupe, sans bouton

Écartée : c’est le défaut de la variante livrée. Un bouton « Valider » coûte un toucher de plus, mais supprime les réponses définitives par erreur.

## Conséquences

### Positives

- La cible de glissement est le mot entier : l’objection de taille de l’ADR 0044 tombe.
- Le joueur voit les deux morceaux avant de couper, et peut se reprendre : se tromper de geste ne coûte rien, comme se tromper de réponse.
- Les formes viennent du design système : la poignée est une brique, la lame en pointillé dit « ce qui manque », le trait plein est un fait.
- Aucun changement côté serveur : la réponse reste la liste des coupes.

### Négatives

- Une étape de plus que dans les autres formats, donc une question un peu plus longue.
- Deux modes d’interaction (tactile, souris) à tester et à maintenir.
- La mesure des jointures dépend de la mise en page du mot : les ligatures et les accents demandent des tests.

## Critères de réévaluation

- Si la part de réponses fausses du format Découper sur mobile reste supérieure de plus de 15 points à celle des autres formats après un mois d’usage, reprendre la précision du geste (pas de la lame, taille de la poignée).
- Si plus d’une séance sur dix est abandonnée sur une question Découper, étudier un retour à des propositions de découpages pour les mots de plus de dix lettres.
- Si les joueurs reprennent presque toujours la première coupe, la lame se pose mal au départ : revoir sa position initiale.
