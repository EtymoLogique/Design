# ADR 0052 — Fusions trompeuses expliquées : trompe-l’œil et mauvais homographe

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0014](0014-sources-et-fascicules.md), pour la phrase « une simple ressemblance de chaînes ne forme pas une combinaison et n’appelle ni recette ni exclusion » : une ressemblance qui trompe vraiment peut désormais être déclarée, avec son explication (le reste de l’ADR 0014, dont les exclusions, reste en vigueur).
- **Complète** : [ADR 0003](0003-recettes-de-fusion.md) (résultats d’une tentative), [ADR 0013](0013-schema-des-briques.md) (homographes), [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) (ressources d’un fascicule)

## Contexte

Deux briques posées bout à bout s’écrivent parfois comme un mot qui n’en vient pas. Aujourd’hui le joueur reçoit alors un échec neutre ([ADR 0003](0003-recettes-de-fusion.md)) et ne sait pas si la piste est fausse ou s’il s’y prend mal. Il s’acharne, ou il abandonne : c’est une frustration, et une occasion manquée d’apprendre quelque chose sur le mot.

Deux situations très différentes se cachent derrière ce même échec :

1. **La fusion est fausse, définitivement.** Les lettres se suivent, mais le mot ne vient pas de ces briques : il a une autre origine, ou ce n’est pas un composé du tout. Aucune autre brique ne la sauve.
2. **La fusion est bonne, avec une autre brique.** Le joueur a posé la bonne forme au mauvais sens. Le cas type est *a-* : le préfixe privatif (« sans », *amoral*) et le préfixe « vers » (*amener*) sont deux unités, donc deux briques ([modèle de données](../modele-donnees.md)). Dire « fusion invalide » serait faux : la recette existe et reste possible.

La question ouverte « Homographes sur la table » du modèle de données porte sur le second cas ; l’[ADR 0014](0014-sources-et-fascicules.md) écartait le premier, faute de réponse de jeu à lui donner.

Contraintes à respecter : se tromper ne coûte rien ; le jeu ne dit jamais « presque » sans pouvoir le justifier ; un fascicule ne dévoile pas son contenu avant que le joueur ne le trouve ([ADR 0015](0015-codex-fascicules-et-legendaires.md), [ADR 0049](0049-date-du-prochain-fascicule-publique.md)) ; la base de progression ne copie aucun texte du catalogue et ne garde pas les propositions de fusion ([ADR 0010](0010-observabilite-et-vie-privee.md)).

## Décision

Une tentative qui n’est ni une recette, ni une exclusion, ni un « presque » d’ordre reçoit, quand c’est possible, une **explication**. Il y en a deux sortes, qui n’ont pas la même origine.

### 1. Mauvais homographe : calculé, rien à saisir

Si remplacer **une** des deux briques posées par un homographe (même langue, même nature, même forme affichée, autre unité) donne une recette exacte, la réponse est : « Bonne forme, autre brique. »

- Le compilateur du catalogue ajoute à chaque carte de brique la liste `homographes` (identifiants des autres briques de même langue, nature et forme normalisée). Le serveur et la PWA la lisent, sans recalculer la normalisation.
- Le message nomme la brique posée avec sa glose (« *a-* « sans » ne convient pas ici ») et dit qu’une autre brique s’écrit de la même façon. Il **ne nomme ni la glose ni l’identifiant de la bonne brique** : le joueur la cherche dans sa réserve ou dans son codex, où les deux *a-* se distinguent par leur glose.
- Seul le **remplacement d’une brique par un homographe qui donne une recette exacte** déclenche ce retour. Un homographe qui n’aboutirait à rien ne produit jamais de fausse lueur.
- La recette reste intacte : elle est indexée par **identifiants de briques**, jamais par forme. Poser *a-* « vers » avec la même seconde brique est une découverte normale.
- Le mot déjà découvert n’est pas rappelé (comme pour un « presque »).

### 2. Trompe-l’œil : une fausse fusion déclarée et expliquée

Un **trompe-l’œil** est un objet éditorial de la couche ludique ([ADR 0009](0009-contenu-et-equilibrage.md)), rattaché à un fascicule, qui déclare : « cette paire ordonnée de briques ressemble à un mot, mais ce mot n’en vient pas ».

| Champ | Contenu |
|---|---|
| `id` | `tpl_` + 8 caractères, opaque ([ADR 0005](0005-pipeline-de-contenu.md)). |
| `fascicule_id` | Le fascicule qui l’active. Ses deux briques doivent être publiées dans ce fascicule ou un précédent. |
| `combinaison` | Exactement deux briques, ordonnées ([ADR 0032](0032-fusion-de-deux-briques.md)). La clé est la paire d’**identifiants de briques**, jamais une forme : un trompe-l’œil sur *a-* « sans » ne touche pas *a-* « vers ». |
| `mot_id` | Le mot que les lettres épellent, quand il est dans le catalogue. Facultatif : *semoule* n’a pas à être un mot du jeu pour qu’on y pense. |
| `motif` | Vocabulaire contrôlé : `autre_origine` (le mot vient d’un autre étymon), `mot_simple` (emprunt en bloc ou mot non composé), `autre_decoupage` (le mot se coupe ailleurs). Il pilote l’affichage et le contrôle de qualité, jamais le texte. |
| `explication_courte` | Une phrase, 140 caractères au plus, par locale. Ne contient jamais la graphie du `mot_id`. |
| `explication` | Quelques phrases, par locale : le mot, sa vraie origine. Rédigée, jamais copiée d’une référence. |
| `fait` | Confiance, état de relecture, sources, `note_interne` : l’affirmation « ce mot ne vient pas de ces briques » est un fait sourcé comme les autres. |

**Retour de jeu.** Un trompe-l’œil prend le pas sur l’échec neutre. Le joueur lit :

- toujours, l’`explication_courte` (« Les lettres se suivent, mais ce mot n’est pas fait de ces deux briques. L’un de ses morceaux a une autre histoire. ») ;
- l’`explication` complète (qui nomme le mot) **seulement si le `mot_id` est au codex du joueur, ou s’il est nul**. Sinon, le mot reste caché : le trompe-l’œil ne révèle pas l’existence d’un mot encore à trouver. Dès que le mot est découvert, son verso gagne une section « À ne pas confondre » qui liste ses trompe-l’œil avec leur explication complète.

Rien n’est consommé, rien n’est récompensé : on n’engrange rien à se tromper exprès.

### Priorité de résolution

`resoudre` ([ADR 0003](0003-recettes-de-fusion.md)) applique, dans l’ordre : recette exacte (découverte, connue), exclusion, **trompe-l’œil**, « presque » d’ordre, **mauvais homographe**, échec neutre. Un trompe-l’œil et un mauvais homographe ne coexistent jamais sur une même paire : le validateur le refuse (voir ci-dessous), ce qui garde un seul mécanisme par cas.

Les trompe-l’œil ne sont pas des exclusions. Une exclusion ([ADR 0014](0014-sources-et-fascicules.md)) est une combinaison **attestée** que l’équipe choisit de ne pas publier (archaïque, offensante). Un trompe-l’œil est une combinaison **non attestée** qu’on explique.

### Données du catalogue (dépôt Content)

- Dossier `ludique/trompe-l-oeil/`, un fichier JSON par objet, schéma `trompe-l-oeil.schema.json`, identifiant `tpl_[0-9a-z]{8}`.
- L’artefact les publie avec les ressources du fascicule (à côté des recettes et des exclusions) ; `note_interne` et `etat_relecture` restent privés (`CHAMPS_PRIVES`). Comme les recettes, ils sont lisibles par tous : le secret tient à l’interface.
- Nouveaux contrôles de `valider`, un code `domaine.regle` chacun, un test, une ligne du README :
  - `trompe_l_oeil.paire` : deux briques publiées au plus tard dans `fascicule_id` ;
  - `trompe_l_oeil.doublon` : au plus un par paire ordonnée ;
  - `trompe_l_oeil.recette` : ni la paire ni son inverse ne sont une recette exacte (variantes comprises), et la paire n’est pas une exclusion ;
  - `trompe_l_oeil.homographe_jouable` : aucun remplacement d’une brique par un homographe ne donne une recette (ce cas est calculé, voir 1) ;
  - `trompe_l_oeil.origine` : si `mot_id` est renseigné, sa composition ou sa chaîne étymologique ne mobilise pas à la fois les deux unités des briques ;
  - `trompe_l_oeil.divulgation` : l’`explication_courte` ne contient pas la graphie du `mot_id` (comparaison normalisée) ;
  - `trompe_l_oeil.sources` : au moins une source ; en publication, état `relu`.
- Un rapport non bloquant `trompe-l-oeil-candidats` liste les paires de briques dont la concaténation (forme normalisée) est la forme d’une unité du catalogue sans recette ni trompe-l’œil, pour aider à repérer les pièges probables. Il ne publie rien.

### Contrat et données du joueur (dépôt App)

- `IssueFusion::Echec` reste `{ "type": "echec" }` et gagne un champ **facultatif** `explication` (`#[serde(default)]`) : `{ "nature": "homographe", "rang": 1 }` ou `{ "nature": "trompe_l_oeil", "trompe_l_oeil_id": "tpl_…" }`. Une PWA qui ne connaît pas ce champ affiche l’échec neutre actuel : le changement est rétrocompatible et suit l’ordre « API d’abord, PWA ensuite ».
- `etymo_domaine::fusion::resoudre` reçoit les trompe-l’œil et les homographes ; `prevoir` (`web/src/services/jeu.ts`) change avec lui, tests miroirs compris. Le serveur fait foi : s’il répond autrement que la prévisualisation, la PWA affiche sa réponse.
- **Aucune table de progression nouvelle.** Les propositions de fusion ne laissent aucune ligne ([modèle de données, § 7](../modele-donnees.md)) ; la télémétrie compte seulement un résultat de plus par type d’explication (`homographe`, `trompe_l_oeil`), sans identifiant de joueur ([ADR 0010](0010-observabilite-et-vie-privee.md)).
- Aucune énergie, aucun exemplaire, aucune encre n’est touché.

### Interface

- Le retour s’affiche sous la table, au même endroit que le « presque » : même gabarit, ton sobre, Encre sur le fond de la carte, jamais de rouge d’erreur. La couleur n’est pas le seul signal : un libellé précède l’explication (« Bonne forme, autre brique », « Fausse piste »).
- **Petites briques cliquables dans le message.** Les briques citées s’affichent comme des briques en ligne (couleur de leur type), chacune étant un bouton d’au moins 44 px de zone tactile qui ouvre sa carte. Dans « Bonne forme, autre brique », la brique posée et la forme (*a-*) sont cliquables ; toucher la forme ouvre la réserve filtrée sur celle-ci, où les deux *a-* se distinguent par leur glose, sans désigner la bonne. Dans « Fausse piste », les deux briques posées sont cliquables.
- Trompe-l’œil : l’explication courte, puis « Pourquoi ? » ouvre l’explication complète si elle est lisible. Un seul bouton Corail par écran : « Pourquoi ? » est neutre.
- Vocabulaire : « trompe-l’œil » et « fausse piste » sont réservés à ce retour ; ni « erreur », ni « invalide », ni « faux ami ».

## Options envisagées

### Stocker, pour chaque joueur, les fusions invalides qu’il a tentées

Écartée : les propositions de fusion ne laissent aucune ligne dans la base de progression, par principe de vie privée, et l’historique d’un joueur n’apprend rien que le catalogue n’enseigne déjà. Les explications sont publiques, attachées au mot, et se relisent dans son verso. Un « carnet des fausses pistes » par joueur pourra être réévalué (voir ci-dessous).

### Étendre les exclusions à toutes les fausses fusions

Écartée : une exclusion décrit un mot attesté qu’on retient, avec deux raisons possibles ; un trompe-l’œil décrit l’inverse, un mot qui n’est pas formé ainsi. Les confondre brouillerait le contrôle de qualité (un fait attesté contre un fait réfuté) et le retour de jeu.

### Écrire les explications des mauvais homographes à la main

Écartée : la liste des homographes se déduit du catalogue, et le modèle de données impose déjà que deux homographes se distinguent par leur glose. Un texte saisi pour chaque paire ne dirait rien de plus que « ce n’est pas ce *a-* », et il faudrait le maintenir à chaque fascicule.

### Révéler la bonne brique quand le joueur se trompe d’homographe

Écartée : ce serait donner la solution. On dit seulement « autre brique, même forme », comme le « presque » d’ordre ne dit pas la recette.

### Un échec neutre partout, avec une page d’aide générale

Écartée : elle ne répond pas au joueur qui vient d’essayer *a-* et qui ne comprend pas pourquoi ça ne marche pas.

## Conséquences

### Positives

- Le joueur sait si sa piste est fausse ou s’il est tout près, au lieu de s’acharner ou d’abandonner.
- Une frustration devient un fait de langue appris : le joueur lit pourquoi *para-* de *parasol* n’est pas celui du grec.
- Le cas des homographes (*a-* et les autres) est traité sans saisie éditoriale, par construction.
- Aucune table, aucune donnée personnelle ni dépense nouvelle : le changement tient au catalogue et à `resoudre`.
- Rétrocompatible : une ancienne PWA affiche un échec neutre.

### Négatives

- Un travail éditorial de plus : repérer les paires piégeuses, rédiger deux textes sourcés par trompe-l’œil, les relire.
- Le « presque » d’un homographe dit qu’une autre brique existe, ce qui informe un peu sur le catalogue.
- Un trompe-l’œil dont le `mot_id` est inconnu du joueur reste vague (« un de ses morceaux a une autre histoire ») : l’explication complète attend la découverte.
- `resoudre` et `prevoir` gagnent deux règles à garder en miroir.

## Critères de réévaluation

- Plus de 30 % des échecs d’une semaine tombent sur une paire qu’aucun trompe-l’œil ni homographe n’explique : élargir la liste des candidats.
- Un trompe-l’œil dont l’explication courte reste lue sans que le joueur ouvre « Pourquoi ? » : fusionner les deux textes.
- Des joueurs redemandent les explications déjà lues : étudier un carnet des fausses pistes, en pesant la règle « aucune proposition de fusion stockée ».
- Une explication jugée spoiler par les testeurs : ne plus afficher l’explication courte tant que le mot n’est pas trouvé.
