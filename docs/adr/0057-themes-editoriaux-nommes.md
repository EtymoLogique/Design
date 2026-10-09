# ADR 0057 — Pas de thème éditorial nommé dans le MVP

- **Statut** : Proposé
- **Portée** : couche ludique du catalogue et codex
- **Complète** : [ADR 0009](0009-contenu-et-equilibrage.md) (couche ludique), [ADR 0015](0015-codex-fascicules-et-legendaires.md) (codex), [ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md) (légendaires secrètes)

## Contexte

Aucun regroupement par domaine (médecine, navigation, astronomie) n’existe dans le catalogue. Le modèle de données laisse la porte ouverte : « Un regroupement éditorial nommé (« La terre ») reste possible plus tard, dans la couche ludique. » La piste des fascicules spéciaux thématiques ([potentiel d’évolution](../potentiel-evolution.md)) en aurait besoin.

Le codex regroupe déjà les cartes de trois façons :

- par **fascicule**, avec son nom (« Argile », « Lapidaire »), sa jaquette et sa complétude ([ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)) ;
- par **famille**, les mots formés sur une brique trouvée, déduits des recettes sans saisie ([codex](../codex.html)) ;
- par **langue**, avec la carte de chaque langue rencontrée.

Un thème serait un quatrième regroupement, saisi à la main, qui traverse les fascicules et les familles.

## Décision

### Aucun thème dans le MVP

- Le catalogue ne déclare **aucun thème** nommé, et le codex n’en affiche pas.
- Le nom d’un fascicule reste son seul titre éditorial ; il peut évoquer un domaine (« Lapidaire »), sans en faire une donnée.
- Les familles restent dérivées des recettes. Rien ne se saisit pour elles.

### Si un thème vient plus tard

Un nouvel ADR le décidera, avec la piste des fascicules spéciaux. Il respectera ces garde-fous :

- un thème est une donnée de la **couche ludique** ([ADR 0009](0009-contenu-et-equilibrage.md)), jamais un fait linguistique : il référence des mots publiés, jamais l’inverse ;
- il ne regroupe que des **mots**, pas des briques ni des langues, qui ont déjà leurs familles et leurs cartes ;
- il ne dévoile rien : un thème n’apparaît au codex qu’après la découverte d’un de ses mots, ses mots inconnus restent en silhouette, et un mot légendaire n’y figure ni dans la liste ni dans le compteur ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)) ;
- il ne crée ni rareté, ni récompense propre, ni pression de temps.

## Options envisagées

### Un thème par mot, dans la couche ludique, dès maintenant

Il nourrirait une vue « Thèmes » du codex. Écarté pour le MVP : aucune fonction ne le consomme encore, chaque mot demanderait un jugement de plus à saisir et à relire, et le codex compte déjà trois regroupements. Le coût éditorial passe avant l’apport.

### Un thème comme fait linguistique

Il vivrait à côté des sens, dans la couche des faits. Écarté : un domaine n’est pas une propriété étymologique attestée par une source de référence, et la couche linguistique ne doit pas dépendre du jeu ([ADR 0002](0002-graphe-linguistique-editorial.md), [ADR 0009](0009-contenu-et-equilibrage.md)).

### Déduire le thème des sens ou des briques

*-logie* ou *neuro-* suggèrent un domaine, mais pas assez sûrement : *astrologie* n’est pas une science, alors qu’*astronomie* en est une. Écarté : un regroupement faux serait pire que pas de regroupement.

## Conséquences

### Positives

- Aucun champ de plus à saisir, à relire ni à compiler.
- Le codex garde trois regroupements clairs, dont deux se calculent.
- Les garde-fous sont posés avant que les fascicules spéciaux n’en aient besoin.

### Négatives

- Un joueur curieux de la médecine ou de la navigation ne trouve pas de vue qui rassemble ces mots.
- Un futur fascicule spécial thématique demandera d’abord cet ADR complémentaire.

## Critères de réévaluation

- Un fascicule spécial thématique est décidé : définir le thème avec lui.
- Les retours de joueurs demandent un regroupement par domaine que les familles ne donnent pas.
- La recherche du codex ne suffit plus à retrouver les mots d’un même domaine.
