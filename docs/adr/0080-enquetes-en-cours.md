# ADR 0080 — Enquêtes en cours : un indice acheté reste lisible

- **Statut** : Proposé
- **Portée** : écran de la table (I-09) et feuille des indices (I-12) de la PWA
- **Complète** : [ADR 0051](0051-indices-a-la-carte.md) (indices à la carte), qui ne dit pas où se relit un indice déjà payé

## Contexte

L’[ADR 0051](0051-indices-a-la-carte.md) vend trois indices par carte à trouver : silhouette, définition, sens littéral. La feuille des indices ne montre qu’une carte à la fois, celle que l’App juge la plus utile. Dès qu’elle passe à une autre carte, ce qui a été acheté pour la première ne se lit plus nulle part : jusqu’à 28 gouttes perdues de vue par carte.

L’[ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md) promettait qu’« un niveau acquis reste lisible ». L’ADR 0051 ne le redit pas et l’App ne le garantit plus. Le besoin a été retenu par le propriétaire sur la page de travail [pistes de rétention](../pistes-retention.html#enquetes-titre).

## Décision

### Une enquête

- Une **enquête** est une carte à trouver dont le joueur a acheté au moins un indice. Elle dure jusqu’à la découverte du mot.
- Une enquête ne dépend pas de la réserve : elle reste visible même quand le mot ne peut plus se former avec les briques du joueur.

### Où elle se lit

- Sous la table de fusion, un panneau « Enquêtes en cours » liste les enquêtes par numéro de carte croissant, une ligne repliée par enquête : « N° 003 · Silhouette, définition », avec en aperçu le dernier texte acheté.
- Dépliée, une enquête montre la silhouette, la définition et le sens littéral **achetés**, et rien d’autre.
- Au-delà de trois enquêtes, les suivantes se déplient d’un bouton.
- Le panneau ne vend rien : la feuille des indices reste le seul endroit où l’on achète.

### Ce qui ne se voit jamais

- Ni le mot, ni ses lettres, ni ses briques, ni sa langue, ni sa rareté, ni s’il se forme avec la réserve ([ADR 0051](0051-indices-a-la-carte.md)).
- Jamais un mot légendaire ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)), jamais une carte d’un fascicule verrouillé ([ADR 0069](0069-deblocage-progressif-des-fascicules.md)).

### Sans règle nouvelle

La PWA relit les indices obtenus que le serveur envoie déjà dans l’état. Aucune table, aucune commande, aucun contrat ne change.

## Options envisagées

### Rouvrir la feuille des indices sur une carte choisie

Un sélecteur de carte dans la feuille. Écartée : il faudrait choisir parmi des silhouettes, donc montrer la liste des cartes à trouver, ce que l’ADR 0051 évite.

### Montrer les enquêtes en tête de la feuille de réserve

Elles seraient sous les yeux au moment de choisir les briques. Écartée pour l’instant : la modale de réserve porte déjà la petite table, la consigne et les filtres. À reprendre si le panneau sous la table ne suffit pas.

### Relire les indices dans le codex

Sur la carte en silhouette. Écartée : le joueur cherche quand il est à la table, pas dans le codex, et la carte en silhouette ne doit rien montrer gratuitement.

## Conséquences

### Positives

- Un indice payé ne se perd plus : la promesse de l’ADR 0038 est tenue, sans règle nouvelle.
- Le joueur a des pistes en cours à poursuivre d’une séance à l’autre.

### Négatives

- Un panneau de plus sur l’écran de la table, qui allonge la page sur mobile.

## Critères de réévaluation

- Le panneau reste replié ou ignoré : le déplacer dans la feuille de réserve.
- Les joueurs accumulent plus de dix enquêtes : regrouper par fascicule.
