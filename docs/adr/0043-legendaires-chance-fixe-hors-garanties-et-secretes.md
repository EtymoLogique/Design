# ADR 0043 — Légendaires : une chance fixe, hors des garanties, toujours secrètes

- **Statut** : Proposé
- **Remplace partiellement** :
  - [ADR 0011](0011-plis-raretes-et-doublons.md), pour la garantie de nouveauté, qui ne vise plus les légendaires, et pour la ligne de chance propre à une légendaire connue ;
  - [ADR 0012](0012-briques-rationnees.md), pour le filet d’utilité, qui ne choisit jamais une légendaire ;
  - [ADR 0015](0015-codex-fascicules-et-legendaires.md), pour la ligne de chance d’une légendaire trouvée, pour la silhouette d’un mot légendaire une fois sa légendaire trouvée, et pour l’affichage des légendaires trouvées ;
  - [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md), pour la garantie et le filet de chaque fascicule, qui ne portent plus que sur ses briques non légendaires ;
  - [ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md), pour le poids des légendaires, remplacé par une chance fixe ;
  - [ADR 0040](0040-rarete-des-mots.md), pour la silhouette et la forme de rareté d’un mot légendaire encore inconnu.
- **Complète** : [ADR 0036](0036-chances-des-legendaires-toujours-affichees.md) (la ligne des légendaires n’affiche plus jamais 0 %) ; [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md) (un pli de jalon ne contient jamais de légendaire) ; [ADR 0004](0004-progression-atteignable.md) (le tutoriel et la réserve de départ n’en emploient jamais)

## Contexte

Les ADR 0015 et 0036 gardent secrets le nombre et la nature des légendaires d’un fascicule. Trois fuites les trahissent pourtant :

- **Le nombre se calcule.** Une légendaire pèse 2 et une commune 62 ([ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md)), et toutes les chances affichées sont exactes ([ADR 0036](0036-chances-des-legendaires-toujours-affichees.md)). La ligne des légendaires inconnues vaut donc leur nombre multiplié par la chance d’une commune, divisé par 31 : avec 6,2 % pour une commune et 0,4 % pour les légendaires inconnues, il en reste 31 × 0,4 / 6,2 = 2. Les chances par type trahissent aussi leur type, et la ligne de rareté leur nombre total.
- **La garantie de nouveauté les donne.** Elle vaut « tant qu’il reste des briques inconnues » ([ADR 0011](0011-plis-raretes-et-doublons.md)) : une fois toutes les autres briques connues, la dernière légendaire tombe au plus tard au 6ᵉ pli, et le message de garantie dit s’il en reste.
- **« Légendaire 0 % »** révèle un fascicule sans légendaire, ce que l’ADR 0036 avait accepté.

Les mots qui exigent une légendaire fuient aussi. Leur silhouette apparaît dès que leur légendaire est trouvée ([ADR 0015](0015-codex-fascicules-et-legendaires.md)). Au verso d’un affixe, ils entrent dans le compte des mots formés ; une fois trouvés, ils entrent dans la complétude du fascicule, dont le total augmente d’un d’un coup.

Sans la garantie, une légendaire de poids 2 ne serait presque jamais tirée : environ 0,2 % par pli, soit 14 % de chances en un mois pour un joueur assidu (75 plis). Or nous voulons qu’une fois son fascicule complet, le joueur assidu ait encore une vraie chasse à mener, sans jamais savoir combien de légendaires il lui reste.

## Décision

### Une chance fixe par pli

- Chaque pli a **1,2 % de chances** de donner une légendaire, la même dans tous les fascicules, quel que soit leur nombre. Les légendaires d’un fascicule se partagent cette chance à parts égales.
- Les 98,8 % restants se répartissent entre les briques non légendaires selon le poids de leur rareté : commune 62, peu commune 26, rare 10 ([ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md)).
- La chance est un paramètre d’équilibrage du pli, `chance_legendaire_pour_mille` (12 pour mille, soit 1,2 % ; [ADR 0009](0009-contenu-et-equilibrage.md)).

| Chance d’une légendaire par pli | Au moins une en 25 plis (environ 10 jours) | Au moins une en 75 plis (un mois de joueur assidu) |
|---|---|---|
| 0,2 % (poids 2, une légendaire, sans garantie) | 5 % | 14 % |
| **1,2 % (retenu)** | **26 %** | **60 %** |
| 2 % | 40 % | 78 % |
| 3 % | 53 % | 90 % |
| 5 % | 72 % | 98 % |

### Ce que montre l’écran des plis

- Les quatre raretés, toujours ([ADR 0036](0036-chances-des-legendaires-toujours-affichees.md)) : la ligne « Légendaire » vaut 1,2 % dans tous les fascicules.
- **Une seule ligne pour toutes les légendaires du fascicule**, connues ou non : le diamant, « Légendaires » et 1,2 %, jamais leur nombre. Une légendaire trouvée ne retrouve pas de ligne propre : sa chance, 1,2 % divisé par leur nombre, le trahirait.
- Les chances par type (préfixes, suffixes) se calculent sans les légendaires : leur somme vaut 98,8 %.
- Chaque brique non légendaire garde sa chance exacte.

### Les garanties

- La **garantie de nouveauté** (6ᵉ pli, [ADR 0011](0011-plis-raretes-et-doublons.md), [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)) et le **filet d’utilité** (5ᵉ pli, [ADR 0012](0012-briques-rationnees.md)) ne choisissent que parmi les briques non légendaires du fascicule. Quand toutes ses briques non légendaires sont connues, la garantie de nouveauté s’arrête.
- Une **garantie lointaine** propre aux légendaires borne l’attente : dans chaque fascicule, le 100ᵉ pli d’affilée sans légendaire en donne une. Elle peut être déjà connue : c’est alors un doublon, qui ajoute un exemplaire ou de l’encre si la réserve est pleine ([ADR 0019](0019-encre-a-chaque-doublon.md), [ADR 0020](0020-deux-plis-en-attente-et-sabliers.md)). Son compteur est affiché.
- Seuls les plis ouverts avec l’énergie comptent pour les trois garanties, comme pour la garantie et le filet aujourd’hui ([ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md)).
- Aucun message ne dit que toutes les briques d’un fascicule sont connues, ni qu’il ne reste aucune légendaire. Par exemple : « Vous connaissez toutes les briques de ce fascicule, hors légendaires : chaque pli remplit votre réserve. »

### Au moins une légendaire par fascicule

- Chaque fascicule publie au moins une brique légendaire, employée par au moins une recette. Le validateur le vérifie avant toute publication.
- La ligne des légendaires n’affiche donc jamais 0 %, et ses 1,2 % ne disent rien de leur nombre.

### Les mots légendaires

- Un mot est légendaire quand sa recette la moins rare exige une légendaire ([ADR 0040](0040-rarete-des-mots.md)).
- **Avant d’être trouvé**, il n’a ni carte, ni silhouette, ni piste, ni forme de rareté, même quand sa légendaire est déjà trouvée. Il n’apparaît dans aucune liste de mots formés, au verso d’une brique, et n’est visé par aucun indice, aucune quête, aucun filet.
- **Il ne compte jamais**, trouvé ou non, dans aucune jauge : complétude d’un fascicule, familles, langues, verso d’une brique, maîtrise ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)), quêtes au long cours ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)). Un fascicule reste « complet » quand toutes ses cartes non légendaires sont trouvées ([ADR 0015](0015-codex-fascicules-et-legendaires.md)).
- **Une fois trouvé**, il a sa carte, rangée à part avec les briques légendaires trouvées. Il se révise comme les autres, sans entrer dans la jauge de maîtrise.

### Plis de jalon, tutoriel et réserve de départ

- Un pli de jalon ne contient jamais de légendaire : sa récompense est affichée d’avance ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)). Le validateur le vérifie.
- Le tutoriel, les graines ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)) et le cadeau de bienvenue ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)) n’emploient jamais de légendaire. Si *étymologie* ouvre le jeu ([game design](../game-design.md)), la brique *étymo-* n’est pas légendaire.

### Le compte des légendaires trouvées

- Un compteur, le diamant suivi du nombre (« ◆ 2 »), accompagne chaque jauge de complétude : en tête du codex, sur chaque fascicule du codex, sur le choix du fascicule de l’écran des plis et sur le sélecteur de la page Quêtes.
- Il compte les briques **et** les mots légendaires trouvés. Son nom accessible les détaille : « 2 légendaires trouvées : 1 brique, 1 mot ».
- Jamais de total ni de barre : seulement ce qui a été trouvé.
- Il s’affiche même à 0 : puisque chaque fascicule a au moins une légendaire, « ◆ 0 » ne trahit rien et, à côté de « complet », invite à la chasse.
- Le toucher n’affiche plus, dans le codex, que les légendaires trouvées.

## Options envisagées

### Garder le poids de 2 et la garantie de nouveauté

Écartée : le nombre de légendaires se calcule à partir des chances affichées, et la dernière tombe au plus tard au 6ᵉ pli.

### Garder le poids de 2 sans garantie

Écartée : environ 0,2 % par pli, 14 % de chances en un mois pour un joueur assidu. La légendaire deviendrait presque introuvable.

### Arrondir ou masquer les chances des légendaires

Écartée : une chance affichée n’est jamais arrondie de manière trompeuse, et toutes les probabilités du tirage restent affichées ([ADR 0036](0036-chances-des-legendaires-toujours-affichees.md)).

### Une ligne par légendaire connue

Écartée : sa chance, 1,2 % divisé par le nombre de légendaires, trahirait ce nombre.

### Une garantie lointaine qui ne donne qu’une légendaire nouvelle

Écartée : quand elle cesserait de s’appliquer, le joueur saurait qu’il ne lui en reste aucune.

### Une chance de 2 %, de 3 % ou de 5 %

Écartées : à 3 %, un joueur assidu a une chance sur deux d’en tirer une en dix jours et neuf sur dix sur un mois ; à 5 %, la chasse se termine trop vite. À 1,2 %, il a environ une chance sur quatre en dix jours et six sur dix sur un mois : la légendaire reste une chasse longue, et la garantie au 100ᵉ pli la borne.

### Masquer le compte des légendaires trouvées tant qu’il vaut 0

Écartée : avec au moins une légendaire par fascicule, « 0 » ne révèle rien, et le compteur rend visible la chasse de fin de mois.

## Conséquences

### Positives

- Le nombre et la nature des légendaires restent secrets, même pour qui fait le calcul.
- Une fois son fascicule complet, le joueur assidu a encore une chasse réelle, sans attente sans fin grâce à la garantie au 100ᵉ pli.
- Les mots légendaires deviennent de vraies surprises : rien ne les annonce, et le joueur les déduit du sens de la brique légendaire.
- Le joueur garde le compte de ses légendaires, à côté de chaque jauge.

### Négatives

- La chance d’une légendaire connue n’est plus affichée seule : seule la chance de l’ensemble l’est. La transparence de groupe est à faire valider juridiquement, comme le prévoit déjà l’ADR 0015.
- Chaque fascicule doit publier au moins une légendaire, et ses mots : une charge éditoriale de plus.
- Environ 30 % des joueurs devront attendre la garantie lointaine pour obtenir une légendaire : la probabilité de n’en tirer aucune en 99 plis est de 0,988 puissance 99, soit 30 %.
- 1,2 % des plis vont aux légendaires : le rythme des fascicules est à resimuler ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)).
- La démonstration et l’App sont à corriger. L’App compte aujourd’hui les mots légendaires au verso des affixes et dans la complétude des fascicules, et n’affiche les légendaires trouvées qu’à partir d’une, briques seulement (dépôt App : `web/src/services/codex.ts`, `web/src/composants/Codex.tsx`).

## Critères de réévaluation

- Moins de la moitié des joueurs assidus trouvent une légendaire dans le mois de son fascicule : relever la chance.
- La garantie lointaine se déclenche pour plus de 10 % des joueurs : la chance est trop basse.
- Une règle de transparence sur les coffres à butin exige la chance de chaque objet.
- Les tests montrent que « ◆ 0 » à côté de « complet » est ressenti comme une frustration plutôt que comme une invitation.
