# ADR 0033 — Gestes de la table de fusion : un toucher pose, chaque brique garde sa case

- **Statut** : Accepté
- **Remplace partiellement** : [ADR 0001](0001-pwa-responsive.md), pour la sélection suivie de « Ajouter à la table » ; [ADR 0032](0032-fusion-de-deux-briques.md), pour la troisième brique qui remplace celle de l'emplacement visé
- **Complète** : [ADR 0032](0032-fusion-de-deux-briques.md) (deux emplacements), la [démonstration de la table](../jeu.html) et le [mouvement](../mouvement.html)

## Contexte

L'[ADR 0001](0001-pwa-responsive.md) prévoyait sur mobile un geste en deux temps : toucher une brique pour la sélectionner, puis « Ajouter à la table ». Les maquettes cernaient la brique sélectionnée d'un anneau Corail. L'[ADR 0032](0032-fusion-de-deux-briques.md) fixe deux emplacements, et remplace la brique de l'emplacement visé quand la table est pleine.

À l'essai de l'application, le double geste paraissait lent et l'anneau Corail se confondait avec une action. Le remplacement silencieux effaçait une hypothèse en cours sans que le joueur l'ait voulu. Et quand on retirait la première brique, la seconde glissait à sa place : l'ordre, qui fait partie de l'hypothèse, changeait tout seul.

## Décision

### Poser et retirer

- **Un toucher ou un clic sur une brique de la réserve la pose** dans la première case libre, sur mobile comme sur ordinateur. Il n'y a plus de sélection ni de bouton « Ajouter à la table ». Le glisser-déposer pose sur la case visée.
- **Table pleine : rien ne se passe.** Une troisième brique n'en remplace aucune. Seul un dépôt explicite sur une case pleine la remplace.
- **Chaque brique garde sa case.** Toucher une brique posée la rend à la réserve et laisse sa case vide ; l'autre brique ne bouge pas. Glisser une brique sur l'autre case les échange, ou la déplace si cette case est vide. Au clavier : flèches pour échanger, Suppr pour vider.
- **Fusionner** est le bouton principal, sur toute la largeur, suivi de **Vider**. Le retour de la tentative s'écrit dessous, en ligne.

### Retours et mouvement

- **Aucun anneau Corail sur une brique**, posée ou non. Le focus clavier est un anneau Encre.
- **Découverte** (après la réponse du serveur, jamais avant) : les deux briques se rejoignent, deux ondes et des confettis (700 ms), puis la fiche s'ouvre. Un « −1 » s'envole une seule fois des briques utilisées.
- **Refus doux** : échec, « presque », combinaison écartée ou réserve insuffisante secouent brièvement la table (500 ms, [mouvement](../mouvement.html)), sans perte. Un mot déjà découvert ne secoue pas : sa fiche est rappelée.
- **« Continuer » après une découverte** : la carte se réduit et vole jusqu'à l'onglet Codex, qui la reçoit d'un rebond (620 ms, comme le vol de carte). Mouvement réduit : la fiche se ferme simplement.

### Ce qui quitte l'écran de la table

- Les **découvertes récentes** ne s'affichent plus sous la table : le codex et son compteur Corail les portent seuls.

## Options envisagées

### Garder la sélection puis « Ajouter à la table »

Écartée : deux gestes pour un, et un anneau Corail qui ressemble à une action. Le toucher direct reste accessible : la brique est un bouton, et le clavier garde ses raccourcis.

### Remplacer la dernière brique quand la table est pleine

Écartée : l'hypothèse change sans que le joueur l'ait demandé.

### Tasser les briques quand on en retire une

Écartée : l'ordre des briques fait partie de l'hypothèse ; il ne doit changer que par un geste du joueur.

## Conséquences

### Positives

- une brique se pose en un geste, identique au doigt et à la souris ;
- l'hypothèse posée ne change que par un geste explicite ;
- la table ne porte que ce qui sert à la fusion.

### Négatives

- un toucher involontaire pose une brique : un autre toucher la retire, sans rien consommer ;
- le remplacement d'une brique exige un glisser-déposer.

## Critères de réévaluation

- Les tests montrent des poses involontaires fréquentes au toucher.
- Les joueurs cherchent à remplacer une brique d'un toucher sans y parvenir.
