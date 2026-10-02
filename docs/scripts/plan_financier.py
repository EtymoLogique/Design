#!/usr/bin/env python3
"""Calcule le plan financier d'ÉtymoLogique et régénère ses tableaux dans docs/plan-financier.md.

Usage : python3 docs/scripts/plan_financier.py

Les tableaux sont insérés entre les marqueurs <!-- PLAN:START --> et <!-- PLAN:END -->.
Toutes les hypothèses sont en tête de fichier : modifiez-les ici, jamais dans le Markdown.
Montants en euros hors taxes, sauf les prix de la boutique (TTC, comme affichés au joueur).
"""
from pathlib import Path

DOCS = Path(__file__).resolve().parent.parent
PAGE = DOCS / "plan-financier.md"

MOIS = [
    "oct. 2026", "nov. 2026", "déc. 2026", "janv. 2027", "févr. 2027", "mars 2027",
    "avr. 2027", "mai 2027", "juin 2027", "juil. 2027", "août 2027", "sept. 2027",
    "oct. 2027", "nov. 2027", "déc. 2027", "janv. 2028", "févr. 2028", "mars 2028",
]
N = len(MOIS)
LANCEMENT = 4          # M4 : bêta ouverte en production, avec les fascicules 1 et 2 (ADR 0048)
FASCICULES_A_LA_BETA = 2  # deux fascicules paraissent dès l'ouverture de la bêta, puis un par mois
OUVERTURE = 6          # M6 : ouverture officielle à quatre fascicules, Semaine de la langue française (ADR 0048)
BOUTIQUE = 7           # M7 : ouverture de la boutique de sabliers (ADR 0018, à accepter)

# --- Équipe (€ HT ou chargés, par mois) --------------------------------------------------
TJM_BACK, TJM_FRONT, TJM_MCO = 650, 550, 600
JOURS_BACK = {1: 20, 2: 20, 3: 20, 4: 10, 5: 10, 6: 10}   # API Rust, comptes, boutique
JOURS_FRONT = {1: 20, 2: 20, 3: 20, 4: 10, 5: 8, 6: 8}    # PWA React
JOURS_MCO = 6          # à partir de M7 : correctifs, mises à jour, sécurité, supervision
JOURS_EVOL = 4         # à partir de M7 : évolutions (codex, accessibilité, fascicules spéciaux)
PRODUIT = 4500         # direction produit et design (fondateur ou fondatrice), coût chargé
DA = {1: 2000, 2: 2000, 3: 2000}                          # direction artistique, jaquettes 1 à 3
COMMUNAUTE = 1200      # à partir du lancement : support et communauté (3 j par mois)
AUDIT = {3: 3500, 12: 2000}                               # audit accessibilité et sécurité

# --- Éditorial -------------------------------------------------------------------------
COUT_FASCICULE = 6500  # 25 mots : sourçage et rédaction, relecture savante, jaquette, silhouettes
EDITO_AMORCE = {1: 2000}                                  # schéma, sources, catalogue de démonstration
PRODUCTION_FASCICULES = range(2, N + 1)                   # un fascicule produit par mois dès M2

# --- Infrastructure et frais généraux --------------------------------------------------
INFRA_DEV = 30
INFRA_FIXE, INFRA_PAR_MAU = 50, 0.005                     # Scaleway serverless, fr-par (ADR 0024)
OUTILS = 120
GENERAUX = 290         # expert-comptable, banque, assurance RC pro
JURIDIQUE = {1: 1400, 3: 2500, 6: 4000}                   # SAS et marque INPI ; CGU et RGPD ; revue boutique

# --- Marketing (€ HT) -------------------------------------------------------------------
# (achat média, relations presse et influence, contenus et outils)
MARKETING = {
    1: (0, 0, 1500), 2: (0, 0, 1500), 3: (0, 1500, 3000),
    # La bêta ouvre sans achat média ; le budget de lancement passe à l'ouverture officielle, en mars (ADR 0048).
    4: (0, 0, 2000), 5: (3500, 1500, 1000), 6: (10500, 6500, 1000),
    7: (2000, 1000, 800), 8: (2000, 1000, 800), 9: (2000, 1000, 800),
    10: (1500, 500, 800), 11: (1500, 500, 800), 12: (3500, 1500, 1000),
    13: (2500, 1000, 800), 14: (2500, 1000, 800), 15: (4000, 1500, 1000),
    16: (3000, 1000, 800), 17: (2500, 1000, 800), 18: (4500, 2500, 1000),
}
# Joueurs apportés par la presse, la liste d'attente et les temps forts, hors bouche-à-oreille.
# M4 : la liste d'attente rejoint la bêta ; M6 : presse de l'ouverture officielle (2 000) et Semaine de la langue française (2 000).
TEMPS_FORTS = {4: 1500, 5: 800, 6: 4000, 12: 1000, 15: 800, 18: 2000}

# --- Rétention mensuelle d'une cohorte : part encore active k mois après son arrivée -----
RETENTION = [1.0, 0.30, 0.20, 0.16, 0.14, 0.12, 0.11, 0.10]
DECLIN = 0.95          # au-delà, chaque mois garde 95 % du précédent (un fascicule par mois)

# --- Boutique de sabliers --------------------------------------------------------------
TVA = 0.20
STRIPE_TAUX, STRIPE_FIXE = 0.015, 0.25                    # cartes européennes
PANIER_TTC = 3.50      # achat moyen : entre le lot de 12 (1,99 €) et celui de 36 (4,99 €)
MONTEE = {BOUTIQUE: 0.5}                                  # le premier mois, la moitié du régime

SCENARIOS = {
    # coût d'un joueur payé, bouche-à-oreille, payeurs / MAU, dépense mensuelle TTC par payeur
    "prudent": dict(cpa=3.50, viral=0.07, conversion=0.008, arppu=3.50),
    "central": dict(cpa=2.50, viral=0.10, conversion=0.015, arppu=4.50),
    "favorable": dict(cpa=1.80, viral=0.14, conversion=0.025, arppu=6.00),
}


def retention(k: int) -> float:
    if k < len(RETENTION):
        return RETENTION[k]
    return RETENTION[-1] * DECLIN ** (k - len(RETENTION) + 1)


# Structure de coûts allégée : fondateur ou fondatrice non rémunéré·e qui code une partie du front,
# un seul prestataire back, MCO resserrée, fascicules intégrés en interne.
ALLEGEE = dict(
    produit=0, jours_back={1: 15, 2: 15, 3: 15, 4: 6, 5: 6, 6: 6},
    jours_front={1: 10, 2: 10, 3: 10, 4: 4, 5: 4, 6: 4},
    jours_mco=4, jours_evol=2, communaute=600, cout_fascicule=4500,
)
COMPLETE = dict(
    produit=PRODUIT, jours_back=JOURS_BACK, jours_front=JOURS_FRONT,
    jours_mco=JOURS_MCO, jours_evol=JOURS_EVOL, communaute=COMMUNAUTE, cout_fascicule=COUT_FASCICULE,
)


def simuler(cpa: float, viral: float, conversion: float, arppu: float, c: dict = COMPLETE) -> list[dict]:
    lignes, cohortes = [], []
    for m in range(1, N + 1):
        media, rp, contenu = MARKETING.get(m, (0, 0, 0))
        nouveaux = 0.0
        if m >= LANCEMENT:
            mau_prec = lignes[-1]["mau"] if lignes else 0
            nouveaux = media / cpa + viral * mau_prec + TEMPS_FORTS.get(m, 0)
        cohortes.append(nouveaux)
        mau = sum(c * retention(m - 1 - i) for i, c in enumerate(cohortes))

        payeurs = 0.0
        if m >= BOUTIQUE:
            payeurs = mau * conversion * MONTEE.get(m, 1.0)
        ca_ttc = payeurs * arppu
        ca_ht = ca_ttc / (1 + TVA)
        transactions = ca_ttc / PANIER_TTC
        frais_paiement = ca_ttc * STRIPE_TAUX + transactions * STRIPE_FIXE

        dev = (c["jours_back"].get(m, 0) * TJM_BACK + c["jours_front"].get(m, 0) * TJM_FRONT
               + (c["jours_mco"] + c["jours_evol"]) * TJM_MCO * (m >= 7))
        equipe = dev + c["produit"] + DA.get(m, 0) + c["communaute"] * (m >= LANCEMENT) + AUDIT.get(m, 0)
        edito = c["cout_fascicule"] * (m in PRODUCTION_FASCICULES) + EDITO_AMORCE.get(m, 0)
        infra = (INFRA_FIXE + INFRA_PAR_MAU * mau) if m >= LANCEMENT else INFRA_DEV
        generaux = OUTILS + GENERAUX + JURIDIQUE.get(m, 0)
        marketing = media + rp + contenu
        couts = equipe + edito + infra + generaux + marketing + frais_paiement

        lignes.append(dict(
            m=m, mois=MOIS[m - 1], nouveaux=nouveaux, mau=mau, payeurs=payeurs,
            ca_ttc=ca_ttc, ca_ht=ca_ht, frais_paiement=frais_paiement,
            dev=dev, equipe=equipe, edito=edito, infra=infra, generaux=generaux,
            media=media, rp=rp, contenu=contenu, marketing=marketing,
            couts=couts, resultat=ca_ht - couts,
        ))
    cumul = 0.0
    for l in lignes:
        cumul += l["resultat"]
        l["cumul"] = cumul
    return lignes


def eur(x: float) -> str:
    """Montant arrondi à la centaine (à la dizaine sous 1 000 €), séparateur de milliers français."""
    pas = 10 if abs(x) < 1000 else 100
    v = int(round(x / pas) * pas)
    s = f"{abs(v):,}".replace(",", " ")
    return ("−" if v < 0 else "") + s


def nb(x: float) -> str:
    v = int(round(x / 10.0) * 10) if x >= 100 else int(round(x))
    return f"{v:,}".replace(",", " ")


def somme(lignes, cle, a, b):
    return sum(l[cle] for l in lignes if a <= l["m"] <= b)


def tableau_mensuel(lignes, a, b, titre_col="Mois") -> str:
    rows = [l for l in lignes if a <= l["m"] <= b]
    entetes = [titre_col] + [f"M{l['m']}<br>{l['mois']}" for l in rows] + ["Total"]
    postes = [
        ("Développement", "dev"),
        ("Produit, design, communauté, audits", None),
        ("Éditorial (fascicules)", "edito"),
        ("Infrastructure", "infra"),
        ("Frais généraux et juridique", "generaux"),
        ("Achat média", "media"),
        ("Relations presse et influence", "rp"),
        ("Contenus et outils marketing", "contenu"),
        ("Frais de paiement", "frais_paiement"),
        ("**Total des coûts**", "couts"),
        ("Chiffre d’affaires HT (sabliers)", "ca_ht"),
        ("**Résultat du mois**", "resultat"),
        ("**Trésorerie cumulée**", "cumul"),
    ]
    out = ["| " + " | ".join(entetes) + " |", "|" + "---|" * len(entetes)]
    for label, cle in postes:
        if cle is None:
            vals = [l["equipe"] - l["dev"] for l in rows]
        else:
            vals = [l[cle] for l in rows]
        total = "" if cle == "cumul" else eur(sum(vals))
        cells = [eur(v) for v in vals]
        if label.startswith("**"):
            cells = [f"**{c}**" for c in cells]
            total = f"**{total}**" if total else ""
        out.append("| " + " | ".join([label] + cells + [total]) + " |")
    return "\n".join(out)


def tableau_audience(lignes, a, b) -> str:
    rows = [l for l in lignes if a <= l["m"] <= b]
    out = ["| Mois | Fascicules parus | Nouveaux joueurs | Joueurs actifs du mois (MAU) | Payeurs | CA TTC |",
           "|---|---|---|---|---|---|"]
    for l in rows:
        parus = max(0, l["m"] - LANCEMENT + FASCICULES_A_LA_BETA) if l["m"] >= LANCEMENT else 0
        out.append(f"| M{l['m']} {l['mois']} | {parus} | {nb(l['nouveaux'])} | {nb(l['mau'])} | "
                   f"{nb(l['payeurs'])} | {eur(l['ca_ttc'])} |")
    return "\n".join(out)


def synthese(res) -> str:
    cols = list(res)
    out = ["| Indicateur | " + " | ".join(c[0].upper() + c[1:] for c in cols) + " |", "|---" * (len(cols) + 1) + "|"]

    def ligne(label, f):
        out.append("| " + " | ".join([label] + [f(res[s]) for s in cols]) + " |")

    ligne("Joueurs actifs en M6 (mars 2027)", lambda l: nb(l[5]["mau"]))
    ligne("Joueurs actifs en M18 (mars 2028)", lambda l: nb(l[17]["mau"]))
    ligne("Joueurs acquis sur 18 mois", lambda l: nb(sum(x["nouveaux"] for x in l)))
    ligne("Payeurs en M18", lambda l: nb(l[17]["payeurs"]))
    ligne("CA HT M7 à M18", lambda l: eur(somme(l, "ca_ht", 7, 18)) + " €")
    ligne("CA HT mensuel en M18", lambda l: eur(l[17]["ca_ht"]) + " €")
    ligne("Coûts M1 à M6", lambda l: eur(somme(l, "couts", 1, 6)) + " €")
    ligne("Coûts M7 à M18", lambda l: eur(somme(l, "couts", 7, 18)) + " €")
    ligne("Résultat M7 à M18", lambda l: eur(somme(l, "resultat", 7, 18)) + " €")
    ligne("Trésorerie cumulée au plus bas (besoin de financement)",
          lambda l: eur(min(x["cumul"] for x in l)) + " €")
    ligne("Coûts mensuels en M18", lambda l: eur(l[17]["couts"]) + " €")
    ligne("Couverture des coûts par la boutique en M18",
          lambda l: f"{round(100 * l[17]['ca_ht'] / l[17]['couts'])} %")

    def point_mort(l):
        cout = l[17]["couts"] - l[17]["frais_paiement"]
        s = SCENARIOS_BY_ID[id(l)]
        net_par_payeur = s["arppu"] / (1 + TVA) - s["arppu"] * STRIPE_TAUX - s["arppu"] / PANIER_TTC * STRIPE_FIXE
        return nb(cout / net_par_payeur / s["conversion"])

    ligne("MAU nécessaires pour couvrir les coûts de M18", point_mort)
    return "\n".join(out)


SCENARIOS_BY_ID: dict[int, dict] = {}


def main() -> None:
    res = {}
    for nom, p in SCENARIOS.items():
        res[nom] = simuler(**p)
        SCENARIOS_BY_ID[id(res[nom])] = p
    central = res["central"]
    p = SCENARIOS["central"]
    res["central, structure allégée"] = simuler(**p, c=ALLEGEE)
    SCENARIOS_BY_ID[id(res["central, structure allégée"])] = p
    allegee = res["central, structure allégée"]

    parties = [
        "### Synthèse des scénarios\n",
        synthese(res),
        "\n\n### Phase 1 : développement, de M1 à M3 (scénario central)\n",
        tableau_mensuel(central, 1, 3),
        "\n\n### Phase 2 : bêta ouverte et ouverture officielle, de M4 à M6 (scénario central)\n",
        tableau_mensuel(central, 4, 6),
        "\n\n### Audience des six premiers mois (scénario central)\n",
        tableau_audience(central, 4, 6),
        "\n\n### Phase 3 : MCO et fascicules, de M7 à M12 (scénario central)\n",
        tableau_mensuel(central, 7, 12),
        "\n\n### Phase 3 : MCO et fascicules, de M13 à M18 (scénario central)\n",
        tableau_mensuel(central, 13, 18),
        "\n\n### Audience et boutique, de M7 à M18 (scénario central)\n",
        tableau_audience(central, 7, 18),
        "\n\n### Variante : structure allégée, de M1 à M6 (scénario central)\n",
        tableau_mensuel(allegee, 1, 6),
        "\n\n### Variante : structure allégée, de M7 à M18 (scénario central)\n",
        tableau_mensuel(allegee, 7, 18),
        "\n",
    ]
    bloc = "\n".join(parties)
    texte = PAGE.read_text(encoding="utf-8")
    debut, fin = "<!-- PLAN:START -->", "<!-- PLAN:END -->"
    avant, reste = texte.split(debut)
    _, apres = reste.split(fin)
    PAGE.write_text(f"{avant}{debut}\n\n{bloc}\n{fin}{apres}", encoding="utf-8")
    print(f"{PAGE.relative_to(DOCS.parent)} mis à jour")


if __name__ == "__main__":
    main()
