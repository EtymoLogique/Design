# ADR 0061 — Wikidata fait référence, sans contribution de l’équipe

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0060](0060-wikidata-seule-source-de-verite.md), pour la possibilité de compléter Wikidata avant de publier un fait ; Wikidata seule source, les niveaux de confiance, la fermeture, l’attestation, la prononciation et l’outillage restent en vigueur
- **Complète** : [ADR 0002](0002-graphe-linguistique-editorial.md) (confiance `non_retenue`)

## Contexte

L’[ADR 0060](0060-wikidata-seule-source-de-verite.md) fait de Wikidata la seule source du catalogue, et ajoute : « L’équipe peut compléter Wikidata, puis publier le fait. » Le recoupement des dix fascicules de lancement (issue Content #32) laisse beaucoup de faits que Wikidata ne porte pas : 584 unités sans lexème, 228 relations sans déclaration « dérivé du lexème » (P5191), 707 compositions sans déclaration « combine les lexèmes » (P5238). Compléter Wikidata était la seule voie pour publier ce reste.

Le recoupement relève aussi des désaccords : Wikidata déclare une analyse, le catalogue en saisissait une autre (*tri-* du latin *tres* et non du grec τρεῖς, *Ökologie* de *öko-* et *-logie*, etc.). L’ADR 0060 ne dit pas laquelle l’emporte.

## Décision

### Aucune contribution

- L’équipe **ne contribue pas à Wikidata** : elle n’y crée ni ne modifie aucune entité, déclaration ni sens, et n’y signale pas d’erreur.
- Un fait que Wikidata ne porte pas **reste `brouillon`**, impubliable, tant que Wikidata ne le porte pas de lui-même. Une absence ne prouve rien contre le fait : il n’est pas passé en `non_retenue`.
- Une erreur probable de Wikidata (un sens sans rapport avec le lexème, par exemple) est notée dans `note_interne` du fait concerné, qui reste `brouillon`.

### Wikidata fait référence

- Quand Wikidata **contredit** le catalogue (il déclare une autre analyse du même fait), l’analyse de Wikidata l’emporte. L’analyse du catalogue passe en `non_retenue`, avec dans `note_interne` la déclaration Wikidata qui la contredit et la date de consultation.
- Un sens Wikidata plus étroit que la glose (*mener* : seulement « être en tête d’une course ») n’est pas une contradiction : la liste des sens de Wikidata est incomplète. La glose n’est pas réécrite d’après Wikidata ; sans sens qui la porte, elle reste `brouillon`.
- Réaligner une composition sur Wikidata quand cela change des briques ou des recettes reste une décision de game design, prise au cas par cas.

## Options envisagées

### Compléter Wikidata pour publier le reste

L’ADR 0060 le permettait : créer les lexèmes manquants, déclarer P5191, P5238 et la première attestation (P6684), signaler les erreurs. Écartée par décision de l’équipe (issue Content #32) : Wikidata est la référence, et le catalogue n’en est jamais la source.

### Garder les deux analyses en confiance `discutee`

Écartée : selon l’ADR 0060, `discutee` désigne deux analyses portées par Wikidata. Une analyse du catalogue que Wikidata ne porte pas n’est pas une analyse discutée.

## Conséquences

### Positives

- Une règle simple pour chaque désaccord : Wikidata l’emporte.
- Aucune entité de Wikidata ne dépend de l’équipe : la source reste indépendante du catalogue.

### Négatives

- Le reste du recoupement est impubliable sans échéance : il dépend du rythme de Wikidata, surtout pour le grec ancien et les éléments de composition français (*-logie*, *-phobie*…).
- Des analyses justes mais absentes de Wikidata disparaissent du jeu publié.
- Une erreur de Wikidata reste en place, et bloque les faits qui en dépendent.

## Critères de réévaluation

- Le critère de l’[ADR 0060](0060-wikidata-seule-source-de-verite.md) est atteint : plus du tiers des candidats d’un fascicule ne sont pas publiables faute de lexème Wikidata.
- Un fascicule ne peut plus réunir assez de mots publiables pour respecter sa taille.
