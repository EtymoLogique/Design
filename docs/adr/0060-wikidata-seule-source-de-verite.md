# ADR 0060 — Wikidata, seule source de vérité du catalogue

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0014](0014-sources-et-fascicules.md), pour les sources (Wiktionnaire en repérage, références savantes qui attestent, rôles `reference` et `reperage`) ; le fascicule, la complétude et l'exclusion déclarée restent en vigueur
- **Modifie** : [ADR 0055](0055-prononciation-des-mots.md) (origine des transcriptions), [ADR 0056](0056-date-de-premiere-attestation.md) (origine de la date), [ADR 0058](0058-langues-d-emprunt.md) (la chaîne passe par une langue « d'après Wikidata »)
- **Complète** : [ADR 0002](0002-graphe-linguistique-editorial.md) (sources et confiance), [ADR 0017](0017-licences.md) (licences)

## Contexte

L'[ADR 0014](0014-sources-et-fascicules.md) répartit les sources en deux rôles : des références savantes qui attestent (TLFi, Académie française, Littré, Gaffiot, Ernout-Meillet, Bailly, LSJ, Chantraine) et le Wiktionnaire, qui repère. Ces références ne sont pas libres, souvent payantes ou consultables seulement en ligne. Chaque fait demande une consultation humaine entrée par entrée, que ni un agent ni un script ne peut refaire de façon vérifiable.

Wikidata couvre les langues, et, par ses lexèmes, les lemmes, leur langue, leurs formes, leurs étymologies (dérivation, combinaison de lexèmes) et, parfois, la première attestation d'un mot. Ses données sont CC0 : aucune contrainte de licence ne pèse sur le catalogue. Elles sont interrogeables par SPARQL, donc vérifiables et rejouables. Un serveur MCP les expose aux agents qui rédigent le contenu.

## Décision

### Une seule source de vérité

- **Wikidata est la seule source** d'un fait du catalogue : existence et forme d'une unité, langue, étymon, relation, composition, première attestation, transcription.
- Les références savantes (TLFi, Académie française, Littré, Gaffiot, Ernout-Meillet, Bailly, LSJ, Chantraine) et le Wiktionnaire (y compris Wiktextract et kaikki.org) **ne sont plus des sources** : ni pour attester, ni pour repérer, ni pour vérifier. Aucun import du Wiktionnaire n'est conservé ni comparé.
- Le rôle de source (`reference` ou `reperage`) disparaît : il n'y a qu'un rôle. Une source du catalogue désigne une entité Wikidata (item `Q…` ou lexème `L…`).
- Chaque lien entre un fait et sa source indique **l'identifiant de l'entité consultée** (`Q…` ou `L…`, éventuellement l'identifiant de l'énoncé) et la **date de la consultation**, qui tient lieu d'édition.
- Les gloses, définitions, sens littéraux et explications restent **rédigés par l'équipe**. Aucun libellé Wikidata n'est copié par défaut ; la licence CC0 l'autoriserait, mais les textes du jeu sont éditoriaux.

### Niveaux de confiance

| Confiance | Exigence |
|---|---|
| Établie | Une déclaration Wikidata affirme le fait et porte elle-même une référence. |
| Probable | Une déclaration Wikidata affirme le fait, sans référence. |
| Discutée | Wikidata porte deux analyses en désaccord (deux déclarations, ou deux lexèmes). Chaque analyse est enregistrée. |
| Non retenue | Jamais publiée comme solution. |

- **Sans déclaration Wikidata, pas de fait publié.** Une absence dans Wikidata ne prouve rien contre un fait, mais elle l'empêche d'être publié. L'équipe peut compléter Wikidata, puis publier le fait.
- Le fait naît `brouillon` ; seule une relecture humaine le passe à `relu` ([ADR 0002](0002-graphe-linguistique-editorial.md)). La relecture rejoue la requête.

### Fermeture

La règle de fermeture de l'[ADR 0014](0014-sources-et-fascicules.md) devient : pour toute suite ordonnée de briques publiées, **si Wikidata porte un lexème formé de ces éléments** (propriété « combine les lexèmes » ou « dérivé du lexème »), ce mot est publié avec sa recette, ou écarté par une exclusion déclarée. La recherche des candidats est une requête SPARQL sur les lexèmes, sans import du Wiktionnaire.

### Attestation et prononciation

- La **première attestation** ([ADR 0056](0056-date-de-premiere-attestation.md)) est lue dans la déclaration « first attested from » du lexème, avec sa précision. Un lexème sans cette déclaration n'a pas d'attestation affichée.
- Les **transcriptions** ([ADR 0055](0055-prononciation-des-mots.md)) sont lues dans les formes du lexème. Une forme sans transcription n'en a pas.

### Agent et outillage

Seul l'agent qui génère les nouveaux mots interroge Wikidata, par le serveur MCP `wikidata` (skill `nouveau-mot`). Le validateur du dépôt de contenu ne fait aucun appel réseau : il vérifie la forme des sources (identifiant, date), pas leur véracité.

## Options envisagées

### Garder les références savantes en plus de Wikidata

Écartée : deux sources de vérité se contredisent, et la décision de l'emporter ne se prend pas sans humain. L'équipe veut une seule règle, rejouable.

### Garder le Wiktionnaire en repérage

Écartée : un repérage hors Wikidata ouvre une seconde porte d'entrée aux faits, et il faudrait en conserver les imports datés.

### Wikidata en repérage, références savantes qui attestent

C'était l'état de l'[ADR 0014](0014-sources-et-fascicules.md) en esprit. Écartée : elle laisse le goulot de la consultation humaine et ne se vérifie pas par machine.

## Conséquences

### Positives

- Une seule règle de preuve, rejouable par requête SPARQL, sans licence ni accès payant.
- Un agent peut établir, citer et revérifier un fait seul.
- Plus d'import du Wiktionnaire à archiver ni à comparer d'un fascicule à l'autre.
- Les corrections de l'équipe profitent à Wikidata.

### Négatives

- **La couverture de Wikidata borne le catalogue.** Les lexèmes latins et surtout grecs anciens sont incomplets ; des mots et des étymons seront impubliables tant que Wikidata ne les porte pas.
- Wikidata est ouvert : une déclaration peut changer ou être vandalisée après publication. Le catalogue publié est un instantané ; la date de consultation et la relecture humaine sont les garde-fous.
- Les faits déjà saisis d'après le TLFi ou le Bailly doivent être recoupés avec Wikidata ou rétrogradés en `brouillon`.
- L'ADR 0056 perd la précision éditoriale du TLFi ; les dates seront moins nombreuses.

### Chantiers découlant

- Schéma `source` : un seul rôle, `localisation` = identifiant Wikidata, date de consultation. Règle de publication `publication.source_reference` adaptée.
- Faits existants et sources `src_…` du dépôt de contenu à migrer ([ADR 0005](0005-pipeline-de-contenu.md)).

## Critères de réévaluation

- Plus du tiers des candidats d'un fascicule ne sont pas publiables faute de lexème Wikidata.
- Une déclaration Wikidata utilisée par un fait publié change ou est contestée plus d'une fois par fascicule.
- Wikidata change de licence ou son serveur MCP n'est plus maintenu.
