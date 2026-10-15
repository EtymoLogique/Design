# ADR 0036 — La chance des légendaires est toujours affichée

- **Statut** : Accepté
- **Remplace partiellement** : [ADR 0015](0015-codex-fascicules-et-legendaires.md), pour ce que l’écran des plis laisse deviner des légendaires d’un fascicule
- **Complète** : [ADR 0011](0011-plis-raretes-et-doublons.md) (transparence des chances)

## Contexte

L’ADR 0015 garde le suspense sur les légendaires : une légendaire inconnue n’a ni silhouette ni place dans les compteurs, et les légendaires inconnues d’un pli forment une seule ligne, sans leur nombre.

L’écran des plis affiche les chances par rareté. Un fascicule sans légendaire affiche donc « Légendaire 0 % », ce qui révèle qu’il n’en contient aucune. Masquer la ligne dans ce cas éviterait de le révéler, mais l’écran n’afficherait plus toutes les probabilités du tirage. Or aucun texte français trouvé n’impose cet affichage pour un jeu gratuit. Les obligations connues viennent des magasins d’applications (Apple, Google), des engagements des consoliers (2019) et de PEGI (objets aléatoires payants classés PEGI 16 depuis juin 2026). Le choix repose sur la loyauté envers le joueur et la préparation de la boutique ; le conseil juridique dira si un texte s’applique.

## Décision

- L’écran des plis affiche **toujours la chance des quatre raretés**, légendaire comprise, **même quand elle vaut 0 %**.
- Le suspense porte sur le **nombre** et la **nature** des légendaires : une légendaire inconnue n’a toujours ni silhouette, ni place dans les compteurs, ni ligne propre dans les chances de chaque brique ; les légendaires inconnues restent regroupées en une seule ligne, sans leur nombre (ADR 0015).
- Qu’un fascicule n’ait aucune légendaire devient lisible. C’est accepté.

## Options envisagées

### Masquer la ligne des légendaires quand il n’y en a aucune

Écartée : l’écran n’afficherait plus toutes les probabilités du tirage, ce qu’exigent les magasins d’applications et PEGI, et ce que la loyauté envers le joueur demande.

### Afficher une chance arrondie ou une fourchette

Écartée : une chance affichée n’est jamais arrondie de manière trompeuse (ADR 0011).

## Conséquences

### Positives

- Les probabilités affichées sont complètes et exactes, loyales envers le joueur, conformes aux exigences des magasins et de PEGI, prêtes pour une boutique.
- Le nombre et la nature des légendaires restent secrets.

### Négatives

- Un joueur sait qu’un fascicule ne contient aucune légendaire quand la ligne affiche 0 %.

## Critères de réévaluation

- Le conseil juridique cite un texte, ou le cadre légal ou les règles des magasins changent sur l’affichage des probabilités.
