# ADR 0032 — Une fusion réunit exactement deux briques

- **Statut** : Accepté
- **Remplace partiellement** : [ADR 0003](0003-recettes-de-fusion.md), pour le nombre d'emplacements d'une recette ; [ADR 0014](0014-sources-et-fascicules.md), pour le périmètre de la règle de fermeture
- **Complète** : [ADR 0013](0013-schema-des-briques.md) (tables `recette` et `emplacement`)

## Contexte

L'[ADR 0003](0003-recettes-de-fusion.md) décrit une recette comme une liste ordonnée d'emplacements, sans en fixer le nombre. La vision du jeu laissait la question ouverte (« combien de briques une recette peut-elle utiliser sans devenir illisible ? ») et les maquettes montraient un rail de trois cases au plus, alors que la démonstration jouable n'en a que deux. La règle de fermeture de l'[ADR 0014](0014-sources-et-fascicules.md) dépend de ce nombre : elle couvre toute suite de briques « jusqu'au nombre maximal de briques sur la table ».

Sans limite fixe, le nombre de combinaisons à vérifier croît très vite, la table devient dense sur mobile et le retour « presque » perd sa lisibilité.

## Décision

- La table de fusion a **exactement deux emplacements**, numérotés 1 et 2. Il n'est pas possible de fusionner trois briques ou davantage en une fois.
- **Fusionner** n'est actif que lorsque les deux emplacements sont remplis. Une seule brique ne se fusionne pas.
- Une troisième brique ne s'ajoute jamais à la table : elle remplace celle de l'emplacement visé (le deuxième par défaut sur mobile), qui revient à la réserve sans rien consommer.
- Une recette a exactement deux emplacements. Chacun accepte une ou plusieurs briques (variantes admises, ADR 0003). Le serveur refuse toute tentative qui ne contient pas exactement deux briques.
- Une découverte consomme donc toujours deux exemplaires : un de chaque brique.
- **Fermeture** : la règle de l'ADR 0014 porte sur les **paires ordonnées** de briques publiées. Une composition de trois parties ou plus n'est pas jouable telle quelle et n'entre pas dans la fermeture.
- Un mot plus long se formera en cascade, deux par deux, quand un mot découvert pourra redevenir une brique (*biologie*, puis *biologie* + *-iste*), comme le décrit le [potentiel d'évolution](../potentiel-evolution.md).
- L'indice de niveau 2, « nombre d'ingrédients », n'apprend plus rien : il devient **« nature des deux briques »** (préfixe, suffixe ou mot), dans l'ordre des emplacements.

## Options envisagées

### Trois briques au plus

Écartée : le rail de trois cases est serré sur un écran de 390 px, le nombre de combinaisons à fermer passe du carré au cube du nombre de briques, et un « presque » sur trois briques est difficile à formuler sans donner la solution.

### Nombre libre, fixé par chaque recette

Écartée : le joueur ne sait jamais quand sa proposition est complète, et la fermeture n'a plus de borne vérifiable.

## Conséquences

### Positives

- geste simple et identique sur mobile et ordinateur : deux cases, un « + », un bouton ;
- fermeture bornée et vérifiable : pour n briques publiées, n × n paires ordonnées au plus ;
- retour « presque » lisible : ordre inversé, ou une seule des deux briques juste ;
- coût d'une découverte constant et prévisible pour l'équilibrage des plis.

### Négatives

- les mots de trois éléments ou plus ne sont pas jouables dans le MVP, sauf à les analyser en deux parties dans une composition synchronique relue ;
- l'indice « nature des deux briques » en dit un peu plus que l'ancien « nombre d'ingrédients » ;
- la cascade par mots-ingrédients reste à concevoir.

## Critères de réévaluation

- Des mots essentiels à un fascicule ne peuvent s'analyser en deux parties sans trahir leur histoire.
- Les tests montrent que les joueurs cherchent spontanément à poser une troisième brique et ne comprennent pas le refus.
