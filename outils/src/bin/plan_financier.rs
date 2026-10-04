//! Calcule le plan financier d'ÉtymoLogique et régénère ses tableaux dans docs/plan-financier.md.
//!
//! Usage : cargo run -q --manifest-path outils/Cargo.toml --bin plan_financier
//!
//! Les tableaux sont insérés entre les marqueurs <!-- PLAN:START --> et <!-- PLAN:END -->.
//! Toutes les hypothèses sont en tête de fichier : modifiez-les ici, jamais dans le Markdown.
//! Montants en euros hors taxes, sauf les prix de la boutique (TTC, comme affichés au joueur).

use std::fs;
use std::path::Path;

const MOIS: [&str; 18] = [
    "oct. 2026",
    "nov. 2026",
    "déc. 2026",
    "janv. 2027",
    "févr. 2027",
    "mars 2027",
    "avr. 2027",
    "mai 2027",
    "juin 2027",
    "juil. 2027",
    "août 2027",
    "sept. 2027",
    "oct. 2027",
    "nov. 2027",
    "déc. 2027",
    "janv. 2028",
    "févr. 2028",
    "mars 2028",
];
const N: usize = MOIS.len();
const LANCEMENT: usize = 4; // M4 : bêta ouverte en production, avec les fascicules 1 et 2 (ADR 0048)
const FASCICULES_A_LA_BETA: usize = 2; // deux fascicules paraissent dès l'ouverture de la bêta, puis un par mois
#[allow(dead_code)]
const OUVERTURE: usize = 6; // M6 : ouverture officielle à quatre fascicules, Semaine de la langue française (ADR 0048)
const BOUTIQUE: usize = 7; // M7 : ouverture de la boutique de sabliers (ADR 0018, à accepter)

/// Valeur d'un mois dans une table `(mois, valeur)`, ou `defaut`.
fn au(table: &[(usize, f64)], m: usize, defaut: f64) -> f64 {
    table
        .iter()
        .find(|(k, _)| *k == m)
        .map_or(defaut, |(_, v)| *v)
}

// --- Équipe (€ HT ou chargés, par mois) --------------------------------------------------
const TJM_BACK: f64 = 650.0;
const TJM_FRONT: f64 = 550.0;
const TJM_MCO: f64 = 600.0;
const JOURS_BACK: &[(usize, f64)] = &[
    (1, 20.0),
    (2, 20.0),
    (3, 20.0),
    (4, 10.0),
    (5, 10.0),
    (6, 10.0),
]; // API Rust, comptes, boutique
const JOURS_FRONT: &[(usize, f64)] = &[
    (1, 20.0),
    (2, 20.0),
    (3, 20.0),
    (4, 10.0),
    (5, 8.0),
    (6, 8.0),
]; // PWA React
const JOURS_MCO: f64 = 6.0; // à partir de M7 : correctifs, mises à jour, sécurité, supervision
const JOURS_EVOL: f64 = 4.0; // à partir de M7 : évolutions (codex, accessibilité, fascicules spéciaux)
const PRODUIT: f64 = 4500.0; // direction produit et design (fondateur ou fondatrice), coût chargé
const DA: &[(usize, f64)] = &[(1, 2000.0), (2, 2000.0), (3, 2000.0)]; // direction artistique, jaquettes 1 à 3
const COMMUNAUTE: f64 = 1200.0; // à partir du lancement : support et communauté (3 j par mois)
const AUDIT: &[(usize, f64)] = &[(3, 3500.0), (12, 2000.0)]; // audit accessibilité et sécurité

// --- Éditorial -------------------------------------------------------------------------
const COUT_FASCICULE: f64 = 6500.0; // 25 mots : sourçage et rédaction, relecture savante, jaquette, silhouettes
const EDITO_AMORCE: &[(usize, f64)] = &[(1, 2000.0)]; // schéma, sources, catalogue de démonstration
const PREMIER_FASCICULE_PRODUIT: usize = 2; // un fascicule produit par mois dès M2

// --- Infrastructure et frais généraux --------------------------------------------------
const INFRA_DEV: f64 = 30.0;
const INFRA_FIXE: f64 = 50.0;
const INFRA_PAR_MAU: f64 = 0.005; // Scaleway serverless, fr-par (ADR 0024)
const OUTILS: f64 = 120.0;
const GENERAUX: f64 = 290.0; // expert-comptable, banque, assurance RC pro
const JURIDIQUE: &[(usize, f64)] = &[(1, 1400.0), (3, 2500.0), (6, 4000.0)]; // SAS et marque INPI ; CGU et RGPD ; revue boutique

// --- Marketing (€ HT) -------------------------------------------------------------------
// (achat média, relations presse et influence, contenus et outils)
const MARKETING: [(f64, f64, f64); N] = [
    (0.0, 0.0, 1500.0),
    (0.0, 0.0, 1500.0),
    (0.0, 1500.0, 3000.0),
    // La bêta ouvre sans achat média ; le budget de lancement passe à l'ouverture officielle, en mars (ADR 0048).
    (0.0, 0.0, 2000.0),
    (3500.0, 1500.0, 1000.0),
    (10500.0, 6500.0, 1000.0),
    (2000.0, 1000.0, 800.0),
    (2000.0, 1000.0, 800.0),
    (2000.0, 1000.0, 800.0),
    (1500.0, 500.0, 800.0),
    (1500.0, 500.0, 800.0),
    (3500.0, 1500.0, 1000.0),
    (2500.0, 1000.0, 800.0),
    (2500.0, 1000.0, 800.0),
    (4000.0, 1500.0, 1000.0),
    (3000.0, 1000.0, 800.0),
    (2500.0, 1000.0, 800.0),
    (4500.0, 2500.0, 1000.0),
];
// Joueurs apportés par la presse, la liste d'attente et les temps forts, hors bouche-à-oreille.
// M4 : la liste d'attente rejoint la bêta ; M6 : presse de l'ouverture officielle (2 000) et Semaine de la langue française (2 000).
const TEMPS_FORTS: &[(usize, f64)] = &[
    (4, 1500.0),
    (5, 800.0),
    (6, 4000.0),
    (12, 1000.0),
    (15, 800.0),
    (18, 2000.0),
];

// --- Rétention mensuelle d'une cohorte : part encore active k mois après son arrivée -----
const RETENTION: [f64; 8] = [1.0, 0.30, 0.20, 0.16, 0.14, 0.12, 0.11, 0.10];
const DECLIN: f64 = 0.95; // au-delà, chaque mois garde 95 % du précédent (un fascicule par mois)

// --- Boutique de sabliers --------------------------------------------------------------
const TVA: f64 = 0.20;
const STRIPE_TAUX: f64 = 0.015;
const STRIPE_FIXE: f64 = 0.25; // cartes européennes
const PANIER_TTC: f64 = 3.50; // achat moyen : entre le lot de 12 (1,99 €) et celui de 36 (4,99 €)
const MONTEE: &[(usize, f64)] = &[(BOUTIQUE, 0.5)]; // le premier mois, la moitié du régime

/// Coût d'un joueur payé, bouche-à-oreille, payeurs / MAU, dépense mensuelle TTC par payeur.
#[derive(Clone, Copy)]
struct Scenario {
    cpa: f64,
    viral: f64,
    conversion: f64,
    arppu: f64,
}

const SCENARIOS: [(&str, Scenario); 3] = [
    (
        "prudent",
        Scenario {
            cpa: 3.50,
            viral: 0.07,
            conversion: 0.008,
            arppu: 3.50,
        },
    ),
    (
        "central",
        Scenario {
            cpa: 2.50,
            viral: 0.10,
            conversion: 0.015,
            arppu: 4.50,
        },
    ),
    (
        "favorable",
        Scenario {
            cpa: 1.80,
            viral: 0.14,
            conversion: 0.025,
            arppu: 6.00,
        },
    ),
];

fn retention(k: usize) -> f64 {
    RETENTION
        .get(k)
        .copied()
        .unwrap_or_else(|| RETENTION[7] * DECLIN.powf((k - RETENTION.len() + 1) as f64))
}

struct Structure {
    produit: f64,
    jours_back: &'static [(usize, f64)],
    jours_front: &'static [(usize, f64)],
    jours_mco: f64,
    jours_evol: f64,
    communaute: f64,
    cout_fascicule: f64,
}

// Structure de coûts allégée : fondateur ou fondatrice non rémunéré·e qui code une partie du front,
// un seul prestataire back, MCO resserrée, fascicules intégrés en interne.
const ALLEGEE: Structure = Structure {
    produit: 0.0,
    jours_back: &[
        (1, 15.0),
        (2, 15.0),
        (3, 15.0),
        (4, 6.0),
        (5, 6.0),
        (6, 6.0),
    ],
    jours_front: &[
        (1, 10.0),
        (2, 10.0),
        (3, 10.0),
        (4, 4.0),
        (5, 4.0),
        (6, 4.0),
    ],
    jours_mco: 4.0,
    jours_evol: 2.0,
    communaute: 600.0,
    cout_fascicule: 4500.0,
};
const COMPLETE: Structure = Structure {
    produit: PRODUIT,
    jours_back: JOURS_BACK,
    jours_front: JOURS_FRONT,
    jours_mco: JOURS_MCO,
    jours_evol: JOURS_EVOL,
    communaute: COMMUNAUTE,
    cout_fascicule: COUT_FASCICULE,
};

#[derive(Debug, Default)]
struct Ligne {
    m: usize,
    nouveaux: f64,
    mau: f64,
    payeurs: f64,
    ca_ttc: f64,
    ca_ht: f64,
    frais_paiement: f64,
    dev: f64,
    equipe: f64,
    edito: f64,
    infra: f64,
    generaux: f64,
    media: f64,
    rp: f64,
    contenu: f64,
    couts: f64,
    resultat: f64,
    cumul: f64,
}

/// Somme compensée, comme `sum()` sur des flottants en Python 3.12 et suivants.
fn somme(valeurs: impl IntoIterator<Item = f64>) -> f64 {
    let (mut haut, mut bas) = (0.0f64, 0.0f64);
    for x in valeurs {
        let t = haut + x;
        bas += if haut.abs() >= x.abs() {
            (haut - t) + x
        } else {
            (x - t) + haut
        };
        haut = t;
    }
    if bas != 0.0 && bas.is_finite() {
        haut + bas
    } else {
        haut
    }
}

fn si(condition: bool) -> f64 {
    condition as u8 as f64
}

fn simuler(s: Scenario, c: &Structure) -> Vec<Ligne> {
    let (mut lignes, mut cohortes) = (Vec::<Ligne>::new(), Vec::new());
    for m in 1..=N {
        let (media, rp, contenu) = MARKETING[m - 1];
        let mut nouveaux = 0.0;
        if m >= LANCEMENT {
            let mau_prec = lignes.last().map_or(0.0, |l| l.mau);
            nouveaux = media / s.cpa + s.viral * mau_prec + au(TEMPS_FORTS, m, 0.0);
        }
        cohortes.push(nouveaux);
        let mau = somme(
            cohortes
                .iter()
                .enumerate()
                .map(|(i, c)| c * retention(m - 1 - i)),
        );

        let payeurs = if m >= BOUTIQUE {
            mau * s.conversion * au(MONTEE, m, 1.0)
        } else {
            0.0
        };
        let ca_ttc = payeurs * s.arppu;
        let ca_ht = ca_ttc / (1.0 + TVA);
        let transactions = ca_ttc / PANIER_TTC;
        let frais_paiement = ca_ttc * STRIPE_TAUX + transactions * STRIPE_FIXE;

        let dev = au(c.jours_back, m, 0.0) * TJM_BACK
            + au(c.jours_front, m, 0.0) * TJM_FRONT
            + (c.jours_mco + c.jours_evol) * TJM_MCO * si(m >= 7);
        let equipe = dev
            + c.produit
            + au(DA, m, 0.0)
            + c.communaute * si(m >= LANCEMENT)
            + au(AUDIT, m, 0.0);
        let edito =
            c.cout_fascicule * si(m >= PREMIER_FASCICULE_PRODUIT) + au(EDITO_AMORCE, m, 0.0);
        let infra = if m >= LANCEMENT {
            INFRA_FIXE + INFRA_PAR_MAU * mau
        } else {
            INFRA_DEV
        };
        let generaux = OUTILS + GENERAUX + au(JURIDIQUE, m, 0.0);
        let marketing = media + rp + contenu;
        let couts = equipe + edito + infra + generaux + marketing + frais_paiement;

        lignes.push(Ligne {
            m,
            nouveaux,
            mau,
            payeurs,
            ca_ttc,
            ca_ht,
            frais_paiement,
            dev,
            equipe,
            edito,
            infra,
            generaux,
            media,
            rp,
            contenu,
            couts,
            resultat: ca_ht - couts,
            cumul: 0.0,
        });
    }
    let mut cumul = 0.0;
    for l in &mut lignes {
        cumul += l.resultat;
        l.cumul = cumul;
    }
    lignes
}

fn milliers(v: i64) -> String {
    let chiffres = v.abs().to_string();
    let mut out = String::new();
    for (i, c) in chiffres.chars().enumerate() {
        if i > 0 && (chiffres.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    if v < 0 { format!("-{out}") } else { out }
}

/// Montant arrondi à la centaine (à la dizaine sous 1 000 €), séparateur de milliers français.
fn eur(x: f64) -> String {
    let pas = if x.abs() < 1000.0 { 10.0 } else { 100.0 };
    let v = ((x / pas).round_ties_even() * pas) as i64;
    let s = milliers(v.abs());
    if v < 0 { format!("−{s}") } else { s }
}

fn nb(x: f64) -> String {
    let v = if x >= 100.0 {
        (x / 10.0).round_ties_even() * 10.0
    } else {
        x.round_ties_even()
    };
    milliers(v as i64)
}

fn somme_mois(lignes: &[Ligne], cle: fn(&Ligne) -> f64, a: usize, b: usize) -> f64 {
    somme(lignes.iter().filter(|l| (a..=b).contains(&l.m)).map(cle))
}

/// Libellé d'une ligne de tableau et valeur du mois.
type Poste = (&'static str, fn(&Ligne) -> f64);

fn tableau_mensuel(lignes: &[Ligne], a: usize, b: usize) -> String {
    let rangees: Vec<&Ligne> = lignes.iter().filter(|l| (a..=b).contains(&l.m)).collect();
    let mut entetes = vec!["Mois".to_string()];
    entetes.extend(
        rangees
            .iter()
            .map(|l| format!("M{}<br>{}", l.m, MOIS[l.m - 1])),
    );
    entetes.push("Total".into());
    let postes: [Poste; 13] = [
        ("Développement", |l| l.dev),
        ("Produit, design, communauté, audits", |l| l.equipe - l.dev),
        ("Éditorial (fascicules)", |l| l.edito),
        ("Infrastructure", |l| l.infra),
        ("Frais généraux et juridique", |l| l.generaux),
        ("Achat média", |l| l.media),
        ("Relations presse et influence", |l| l.rp),
        ("Contenus et outils marketing", |l| l.contenu),
        ("Frais de paiement", |l| l.frais_paiement),
        ("**Total des coûts**", |l| l.couts),
        ("Chiffre d’affaires HT (sabliers)", |l| l.ca_ht),
        ("**Résultat du mois**", |l| l.resultat),
        ("**Trésorerie cumulée**", |l| l.cumul),
    ];
    let mut out = vec![
        format!("| {} |", entetes.join(" | ")),
        format!("|{}", "---|".repeat(entetes.len())),
    ];
    for (libelle, cle) in postes {
        let valeurs: Vec<f64> = rangees.iter().map(|l| cle(l)).collect();
        let mut total = if libelle == "**Trésorerie cumulée**" {
            String::new()
        } else {
            eur(somme(valeurs.iter().copied()))
        };
        let mut cellules: Vec<String> = valeurs.iter().map(|v| eur(*v)).collect();
        if libelle.starts_with("**") {
            cellules = cellules.iter().map(|c| format!("**{c}**")).collect();
            if !total.is_empty() {
                total = format!("**{total}**");
            }
        }
        out.push(format!(
            "| {libelle} | {} | {total} |",
            cellules.join(" | ")
        ));
    }
    out.join("\n")
}

fn tableau_audience(lignes: &[Ligne], a: usize, b: usize) -> String {
    let mut out = vec![
        "| Mois | Fascicules parus | Nouveaux joueurs | Joueurs actifs du mois (MAU) | Payeurs | CA TTC |".to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ];
    for l in lignes.iter().filter(|l| (a..=b).contains(&l.m)) {
        let parus = if l.m >= LANCEMENT {
            l.m + FASCICULES_A_LA_BETA - LANCEMENT
        } else {
            0
        };
        out.push(format!(
            "| M{} {} | {parus} | {} | {} | {} | {} |",
            l.m,
            MOIS[l.m - 1],
            nb(l.nouveaux),
            nb(l.mau),
            nb(l.payeurs),
            eur(l.ca_ttc)
        ));
    }
    out.join("\n")
}

fn synthese(resultats: &[(String, Scenario, Vec<Ligne>)]) -> String {
    let colonnes: Vec<String> = resultats
        .iter()
        .map(|(nom, _, _)| nom[..1].to_uppercase() + &nom[1..])
        .collect();
    let mut out = vec![
        format!("| Indicateur | {} |", colonnes.join(" | ")),
        format!("{}|", "|---".repeat(colonnes.len() + 1)),
    ];
    let mut ligne = |libelle: &str, f: &dyn Fn(&Scenario, &[Ligne]) -> String| {
        let cellules: Vec<String> = resultats.iter().map(|(_, s, l)| f(s, l)).collect();
        out.push(format!("| {libelle} | {} |", cellules.join(" | ")));
    };
    ligne("Joueurs actifs en M6 (mars 2027)", &|_, l| nb(l[5].mau));
    ligne("Joueurs actifs en M18 (mars 2028)", &|_, l| nb(l[17].mau));
    ligne("Joueurs acquis sur 18 mois", &|_, l| {
        nb(somme(l.iter().map(|x| x.nouveaux)))
    });
    ligne("Payeurs en M18", &|_, l| nb(l[17].payeurs));
    ligne("CA HT M7 à M18", &|_, l| {
        eur(somme_mois(l, |x| x.ca_ht, 7, 18)) + " €"
    });
    ligne("CA HT mensuel en M18", &|_, l| eur(l[17].ca_ht) + " €");
    ligne("Coûts M1 à M6", &|_, l| {
        eur(somme_mois(l, |x| x.couts, 1, 6)) + " €"
    });
    ligne("Coûts M7 à M18", &|_, l| {
        eur(somme_mois(l, |x| x.couts, 7, 18)) + " €"
    });
    ligne("Résultat M7 à M18", &|_, l| {
        eur(somme_mois(l, |x| x.resultat, 7, 18)) + " €"
    });
    ligne(
        "Trésorerie cumulée au plus bas (besoin de financement)",
        &|_, l| eur(l.iter().map(|x| x.cumul).fold(f64::INFINITY, f64::min)) + " €",
    );
    ligne("Coûts mensuels en M18", &|_, l| eur(l[17].couts) + " €");
    ligne("Couverture des coûts par la boutique en M18", &|_, l| {
        format!(
            "{} %",
            (100.0 * l[17].ca_ht / l[17].couts).round_ties_even() as i64
        )
    });
    ligne(
        "MAU nécessaires pour couvrir les coûts de M18",
        &|s, l| {
            let cout = l[17].couts - l[17].frais_paiement;
            let net_par_payeur =
                s.arppu / (1.0 + TVA) - s.arppu * STRIPE_TAUX - s.arppu / PANIER_TTC * STRIPE_FIXE;
            nb(cout / net_par_payeur / s.conversion)
        },
    );
    out.join("\n")
}

fn main() {
    let mut resultats: Vec<(String, Scenario, Vec<Ligne>)> = SCENARIOS
        .iter()
        .map(|(nom, s)| (nom.to_string(), *s, simuler(*s, &COMPLETE)))
        .collect();
    let p = SCENARIOS[1].1;
    resultats.push(("central, structure allégée".into(), p, simuler(p, &ALLEGEE)));
    let (central, allegee) = (&resultats[1].2, &resultats[3].2);

    let parties = [
        "### Synthèse des scénarios\n".to_string(),
        synthese(&resultats),
        "\n\n### Phase 1 : développement, de M1 à M3 (scénario central)\n".into(),
        tableau_mensuel(central, 1, 3),
        "\n\n### Phase 2 : bêta ouverte et ouverture officielle, de M4 à M6 (scénario central)\n"
            .into(),
        tableau_mensuel(central, 4, 6),
        "\n\n### Audience des six premiers mois (scénario central)\n".into(),
        tableau_audience(central, 4, 6),
        "\n\n### Phase 3 : MCO et fascicules, de M7 à M12 (scénario central)\n".into(),
        tableau_mensuel(central, 7, 12),
        "\n\n### Phase 3 : MCO et fascicules, de M13 à M18 (scénario central)\n".into(),
        tableau_mensuel(central, 13, 18),
        "\n\n### Audience et boutique, de M7 à M18 (scénario central)\n".into(),
        tableau_audience(central, 7, 18),
        "\n\n### Variante : structure allégée, de M1 à M6 (scénario central)\n".into(),
        tableau_mensuel(allegee, 1, 6),
        "\n\n### Variante : structure allégée, de M7 à M18 (scénario central)\n".into(),
        tableau_mensuel(allegee, 7, 18),
        "\n".into(),
    ];
    let bloc = parties.join("\n");
    let page = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/plan-financier.md");
    let texte = fs::read_to_string(&page).expect("lecture de docs/plan-financier.md");
    let (debut, fin) = ("<!-- PLAN:START -->", "<!-- PLAN:END -->");
    let (avant, reste) = texte.split_once(debut).expect("marqueur PLAN:START");
    let (_, apres) = reste.split_once(fin).expect("marqueur PLAN:END");
    fs::write(&page, format!("{avant}{debut}\n\n{bloc}\n{fin}{apres}"))
        .expect("écriture de docs/plan-financier.md");
    println!("docs/plan-financier.md mis à jour");
}
