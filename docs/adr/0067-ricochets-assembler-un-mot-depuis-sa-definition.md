# ADR 0067 — Ricochets : assembler un mot depuis sa définition

- **Statut** : Proposé
- **Complète** : [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (un format de plus, le septième ; la séance, la maîtrise et l’absence de chronomètre ne changent pas) ; [ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md) (même étape de validation) ; [ADR 0007](0007-etat-et-economie-autoritaires.md) (la commande de réponse reste unique et sous autorité serveur)

## Contexte

Les formats des ricochets ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)) vont du sens vers la brique (*Brique d’un sens*) ou de la brique vers le sens (*Sens d’une brique*), et *Découper* part du mot pour retrouver ses morceaux. Aucun ne demande de **construire un mot** : le joueur reconnaît toujours une réponse déjà écrite, ou coupe un mot qu’on lui montre.

Or le geste central du jeu est la fusion : assembler des briques pour retrouver un mot. Les ricochets n’en gardent rien. Le format *Mot d’un sens littéral* s’en approche, mais le sens littéral (« description de la terre ») donne les morceaux presque mot à mot : il se résout par la glose, sans connaître la forme des briques. Le format « littéral ou actuel » a d’ailleurs été abandonné à la livraison de l’App.

Il faut un format où l’énoncé ne trahit pas les morceaux, et où le joueur produit le mot en choisissant ses briques.

## Décision

### Le format *Assembler*

- Le joueur voit la **définition actuelle** d’un mot qu’il a trouvé, jamais son sens littéral : « Science qui décrit la surface de la Terre. » La consigne est « Assemblez le mot. »
- Sous la définition, **deux colonnes** : à gauche les **préfixes**, à droite les **suffixes**. Une colonne propose **4 briques**, ou 2 ou 3 quand le joueur n’en connaît pas assez ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)).
- Le joueur touche **une brique de chaque colonne**. Entre les deux colonnes, un **assemblage** à deux emplacements vides (« ? + ? ») montre son choix : chaque brique choisie y prend sa place, et toucher une brique choisie la retire. Choisir une autre brique dans la même colonne remplace la précédente.
- **« Valider »** s’active quand les deux emplacements sont remplis. Rien n’est jugé ni envoyé avant : comme pour *Découper* ([ADR 0053](0053-decouper-glisser-une-lame-puis-valider.md)), un doigt qui dérape sur le second toucher ne donne pas une réponse définitive. C’est la seule autre exception à « un toucher répond ».
- Une brique s’affiche toujours comme une brique, avec son trait d’union (« géo- », « -graphie ») et la couleur de son type, **sans sa glose** : la glose rendrait la réponse déductible sans connaître le mot.

### Quels mots sont tirés

- Un mot est éligible si la **composition préférée** de sa carte compte **exactement deux briques**, la première écrite avec un trait d’union à droite (« géo- »), la seconde avec un trait d’union à gauche (« -graphie »). C’est la **position** qui décide de la colonne, pas le type : une racine écrite « géo- » se range dans les préfixes et garde la couleur de son type.
- Les mots à trois briques ou plus, et les mots à une seule brique, n’ont pas ce format : *Découper* les couvre.
- Le mot est une carte **trouvée** et non légendaire, comme pour les autres formats ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)). Sa définition actuelle est obligatoire pour tout mot découvrable : le format ne demande aucune donnée éditoriale de plus sur la définition.
- Une définition qui contient le mot lui-même, ou l’une de ses briques sous la même forme, n’est pas proposée : le validateur de contenu la signale, et le mot perd ce format tant qu’elle n’est pas reformulée.

### Les leurres

- Les leurres viennent **des briques que le joueur connaît**, jamais d’une brique inconnue, qui serait dévoilée, et jamais d’un mot inventé. Les préfixes en place dans la colonne de gauche, les suffixes dans celle de droite.
- Un leurre n’a **pas la même classe de sens** que la brique juste : deux affixes de même glose donneraient deux bonnes réponses.
- Aucune des paires possibles entre les deux colonnes, hors la bonne, ne forme un autre **mot du catalogue** : chaque colonne propose 4 briques, soit 16 paires, et une seule est juste. Un leurre qui compose un mot réel, trouvé ou non, est écarté.
- La bonne brique ne tombe pas toujours au même rang : les deux colonnes sont mélangées à chaque question.

### La réponse et le verdict

- La commande `ricochet` reçoit la paire de briques choisies, dans l’ordre (préfixe, suffixe), avec sa clé d’idempotence et la `version_etat` du client. Le serveur compare à la composition préférée : la réponse est juste si et seulement si les deux briques sont celles de la composition. Une seule réponse compte par question, quel que soit le nombre de briques posées puis retirées avant la validation.
- **Bonne réponse** : les deux briques s’emboîtent, le mot s’écrit en entier avec ses morceaux colorés, puis la définition reste affichée sous le mot.
- **Mauvaise réponse** : comme toute mauvaise réponse des ricochets ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)), sans rouge : les briques choisies passent en Sable avec un contour pointillé en Encre secondaire, secousse de 500 ms, rond barré et texte « Pas cette fois. ». Les deux bonnes briques s’emboîtent à leur place et le mot s’écrit. Le verdict dit en clair laquelle des deux briques choisies était juste (« Le début était juste. »), par le texte et par un signe, jamais par la couleur seule ; il n’y a pas de demi-point : la question est juste ou fausse.
- Mouvement réduit : l’emboîtement et la secousse sautent, le verdict affiche directement son état final.

### Ce qui ne change pas

- Une séance compte toujours 5 questions, sans chronomètre ni note. *Assembler* entre dans le mélange des formats, jamais deux fois de suite. Une séance en compte **un seul** : il demande plus de gestes que les autres.
- Niveaux, échéances, maîtrise ([ADR 0065](0065-ricochets-la-maitrise-se-perd-et-se-regagne.md)), quêtes ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)) : une réponse à *Assembler* compte comme n’importe quelle autre réponse à la carte du mot.
- Le format est un candidat de plus dans le tirage : si aucun mot du joueur n’est éligible, ou s’il n’a pas assez de briques connues pour au moins deux choix par colonne, il n’est pas proposé.

### Accessibilité

- Chaque colonne est un groupe de boutons (`role="radiogroup"`, flèches pour se déplacer, Entrée ou Espace pour choisir) ; l’assemblage annonce son contenu (« Préfixe : géo-. Suffixe : pas encore choisi. »). Cibles de 44 px au moins.
- Sur mobile, les deux colonnes restent côte à côte : deux colonnes de briques de 44 px de haut tiennent sur 360 px de large, l’assemblage se pose sous elles, au-dessus de « Valider ».

### Données et serveur

- Le serveur ne connaît aucun texte du catalogue. Il lui faut, pour chaque mot éligible, les deux identifiants de sa composition préférée (`assemblage: [brq_…, brq_…]`) et, pour chaque brique, sa position (préfixe ou suffixe) et sa classe de sens, déjà publiés pour les autres formats. Le manifeste `ricochets` de chaque fascicule les porte ; l’ajout est rétrocompatible (l’API d’abord, la PWA ensuite).
- Le contrat gagne un format `assembler` et une réponse à deux briques ; aucune table n’est créée : le niveau de la carte du mot suffit.
- L’exclusion des paires qui forment un mot réel se calcule à chaque tirage, sur les assemblages de tous les mots du catalogue publiés dans les manifestes : le serveur ne lit toujours aucun texte. Chaque brique porte sa colonne (`position`) dans le manifeste.

## Options envisagées

### Montrer le sens littéral comme énoncé

Écartée : « description de la terre » donne *géo-* et *-graphie* par leurs gloses. Le joueur n’aurait pas à connaître les formes des briques, seulement à traduire. La définition actuelle ne dit rien des morceaux ; c’est à lui de les retrouver.

### Un toucher sur la seconde brique répond

Écartée : comme l’ADR 0053 pour *Découper*, la réponse partirait au dernier toucher, sans appel. Avec deux touchers à faire, le risque est double (on se trompe d’abord de colonne, puis de brique). Un bouton « Valider » coûte un toucher de plus mais supprime les réponses définitives par erreur.

### Laisser le joueur écrire le mot au clavier

Écartée : le clavier, les accents et les variantes de forme (*géographie*, *geographie*) rendent la réponse fragile au toucher, et le joueur n’a pas à produire une orthographe : les briques sont l’objet du jeu.

### Une seule colonne de briques mêlées

Écartée : préfixes et suffixes mêlés donneraient trop de combinaisons à parcourir et ôteraient une information que le joueur utilise déjà à la table de fusion, la place de la brique dans le mot.

### Proposer aussi les mots de trois briques ou plus

Écartée pour l’instant : une troisième colonne (ou une colonne de racines) rend l’écran lourd sur mobile, et ces mots ont déjà *Découper*. À reprendre si la part de mots éligibles reste trop faible (voir les critères).

### Montrer la glose de chaque brique

Écartée : l’énoncé et les gloses suffiraient à déduire le mot par élimination, sans jamais se souvenir de la forme des briques.

## Conséquences

### Positives

- Les ricochets retrouvent le geste de la fusion : assembler des briques pour former un mot, mais sans rien consommer.
- Le joueur retient la **forme** des briques et leur place (« géo- » devant, « -graphie » derrière), ce que ni *Sens d’une brique* ni *Mot d’un sens littéral* n’exercent.
- L’énoncé ne trahit rien : la définition actuelle est la donnée la plus fiable du catalogue, et elle existe déjà pour chaque mot découvrable.
- Aucune table de plus ; les données de jugement sont une paire d’identifiants par mot.

### Négatives

- Un format de plus à dessiner, à tester sur mobile et à maintenir, avec une étape de validation comme *Découper*.
- Les mots à trois briques et plus n’y ont pas accès : la part de cartes concernées dépend du catalogue.
- Le validateur de contenu doit vérifier deux choses de plus : la définition ne contient pas le mot, et aucune paire de leurres ne forme un mot du catalogue.
- Quatre briques par colonne exigent du joueur qu’il en connaisse au moins deux de chaque position pour que le format ait un sens : en début de partie, il est rare.

## Critères de réévaluation

- Si moins de 40 % des cartes de mots trouvées sont éligibles à *Assembler* sur le fascicule de lancement, étudier les mots à trois briques (une colonne de racines, ou deux emplacements fixes pour les briques déjà posées).
- Si la part de réponses fausses à *Assembler* dépasse de plus de 15 points celle des autres formats après un mois d’usage, réduire une colonne à 3 briques.
- Si le joueur retire presque toujours une brique juste après l’avoir posée, revoir la place de l’assemblage et la lisibilité de « Valider ».
- Si des définitions trahissent encore le mot malgré le validateur, ajouter un drapeau éditorial par mot plutôt qu’un contrôle de texte.
