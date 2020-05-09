use rayon::iter::ParallelBridge;
use rayon::prelude::*;

use text_translator::*;

const YANDEX_API_KEY: &str =
    "trnsl.1.1.20200507T202428Z.5e03932d06f63e6a.6ca69498c3b22bff94f6eda9ad8c21b4c3320078";
const ENGINE: Engine = Engine::Api(api::Translator::Yandex {
    key: YANDEX_API_KEY,
});
const TEXT: &str = "Hello, my name is Naruto Uzumaki!\nI love noodles and fights.";

#[test]
fn api_single_translate() {
    let res = ENGINE.translate(
        TEXT.to_string(),
        InputLanguage::Defined(Language::English),
        Language::French,
    );

    res.unwrap();
}

#[test]
fn api_single_translate_automatic_language() {
    let res = ENGINE.translate(TEXT.to_string(), InputLanguage::Automatic, Language::French);

    res.unwrap();
}

#[test]
fn api_translate_all_languages() {
    static mut FAILED_TASKS: u32 = 0;

    Language::iterator().par_bridge().for_each(|language| {
        let res = ENGINE.translate(TEXT.to_string(), InputLanguage::Automatic, language.clone());

        match res {
            Ok(translation) => println!("Translated to {:?} : {:#?}", language, translation),
            Err(err) => {
                println!("Could not translate to {:?} : {:#?}", language, err);
                unsafe { FAILED_TASKS += 1 }
            }
        }
    });

    assert_eq!(0, unsafe { FAILED_TASKS })
}

#[test]
fn api_translate_long_text() {
    const LONG_TEXT: &str = r#"Bannis ! bannis ! bannis ! c'est là la destinée.
Ce qu'apporté le flux sera dans la journée
Repris par le reflux.
Les jours mauvais fuiront sans qu'on sache leur nombre,
Et les peuples joyeux et se penchant sur l'ombre
Diront : Cela n'est plus !

Les temps heureux luiront, non pour la seule France,
Mais pour tous. On verra dans cette délivrance,
Funeste au seul passé,
Toute l'humanité chanter, de fleurs couverte,
Comme un maître qui rentre en sa maison déserte
Dont on l'avait chassé.

Les tyrans s'éteindront comme des météores.
Et, comme s'il naissait de la nuit deux aurores
Dans le même ciel bleu,
Nous vous verrous sortir de ce gouffre où nous sommes,
Mêlant vos deux rayons, fraternité des hommes,
Paternité de Dieu !

Oui, je vous le déclare, oui, je vous le répète,
Car le clairon redit ce que dit la trompette,
Tout sera paix et jour !
Liberté ! plus de serf et plus de prolétaire !
Ô sourire d'en haut ! ô du ciel pour la terre
Majestueux amour !

L'arbre saint du Progrès, autrefois chimérique,
Croîtra, couvrant l'Europe et couvrant l'Amérique,
Sur le passé détruit,
Et, laissant l'éther pur luire à travers ses branches,
Le jour, apparaîtra plein de colombes blanches,
Plein d'étoiles, la nuit.

Et nous qui serons morts, morts dans l'exil peut-être,
Martyrs saignants, pendant que les hommes, sans maître,
Vivront, plus fiers, plus beaux,
Sous ce grand arbre, amour des cieux qu'il avoisine,
Nous nous réveillerons pour baiser sa racine
Au fond de nos tombeaux !


Victor Hugo - Les Châtiments "#;

    let res = ENGINE.translate(
        LONG_TEXT.to_string(),
        InputLanguage::Automatic,
        Language::English,
    );

    match res {
        Ok(translation) => println!("Translated to English : {}", translation),
        Err(err) => {
            println!("Could not translate to English : {:#?}", err);
        }
    }
}

#[test]
fn api_translate_too_long_text() {
    let mut too_long_text = String::from(
        r#"
I

Temps futurs ! vision sublime !
Les peuples sont hors de l'abîme.
Le désert morne est traversé.
Après les sables, la pelouse ;
Et la terre est comme une épouse,
Et l'homme est comme un fiancé !

Dès à présent l'oeil qui s'élève
Voit distinctement ce beau rêve
Qui sera le réel un jour ;
Car Dieu dénoûra toute chaîne,
Car le passé s'appelle haine
Et l'avenir se nomme amour !

Dès à présent dans nos misères
Germe l'hymen des peuples frères ;
Volant sur nos sombres rameaux,
Comme un frelon que l'aube éveille,
Le progrès, ténébreuse abeille,
Fait du bonheur avec nos maux.

Oh ! voyez ! la nuit se dissipe.
Sur le monde qui s'émancipe,
Oubliant Césars et Capets,
Et sur les nations nubiles,
S'ouvrent dans l'azur, immobiles,
Les vastes ailes de la paix !

Ô libre France enfin surgie !
Ô robe blanche après l’orgie !
Ô triomphe après les douleurs !
Le travail bruit dans les forges,
Le ciel rit, et les rouges-gorges
Chantent dans l’aubépine en fleurs !

La rouille mord les hallebardes.
De vos canons, de vos bombardes
Il ne reste pas un morceau
Qui soit assez grand, capitaines,
Pour qu’on puisse prendre aux fontaines
De quoi faire boire un oiseau.

Les rancunes sont effacées ;
Tous les coeurs, toutes les pensées,
Qu’anime le même dessein,
Ne font plus qu’un faisceau superbe ;
Dieu prend pour lier cette gerbe
La vieille corde du tocsin.

Au fond des cieux un point scintille.
Regardez, il grandit, il brille,
Il approche, énorme et vermeil.
Ô République universelle,
Tu n’es encor que l’étincelle,
Demain tu seras le soleil !

II

Fêtes dans les cités, fêtes dans les campagnes !
Les cieux n’ont plus d’enfers, les lois n’ont plus de bagnes.
Où donc est l’échafaud ? ce monstre a disparu.
Tout renaît. Le bonheur de chacun est accru
De la félicité des nations entières.
Plus de soldats l’épée au poing, plus de frontières,
Plus de fisc, plus de glaive ayant forme de croix.
L’Europe en rougissant dit : – Quoi ! j’avais des rois !
Et l’Amérique dit. – Quoi ! j’avais des esclaves !
Science, art, poésie, ont dissous les entraves
De tout le genre humain. Où sont les maux soufferts ?
Les libres pieds de l’homme ont oublié les fers.
Tout l’univers n’est plus qu’une famille unie.
Le saint labeur de tous se fond en harmonie
Et la société, qui d’hymnes retentit,
Accueille avec transport l’effort du plus petit
L’ouvrage du plus humble au fond de sa chaumière
Emeut l’immense peuple heureux dans la lumière
Toute l’humanité dans sa splendide ampleur
Sent le don que lui fait le moindre travailleur ;
Ainsi les verts sapins, vainqueurs des avalanches,
Les grands chênes, remplis de feuilles et de branches,
Les vieux cèdres touffus, plus durs que le granit,
Quand la fauvette en mai vient y faire son nid,
Tressaillent dans leur force et leur hauteur superbe,
Tout joyeux qu’un oiseau leur apporte un brin d’herbe.

Radieux avenir ! essor universel !
Epanouissement de l’homme sous le ciel !

III

Ô proscrits, hommes de l’épreuve,
Mes compagnons vaillants et doux,
Bien des fois, assis près du fleuve,
J’ai chanté ce chant parmi vous ;

Bien des fois, quand vous m’entendîtes,
Plusieurs m’ont dit : « Perds ton espoir.
Nous serions des races maudites,
Le ciel ne serait pas plus noir !

» Que veut dire cette inclémence ?
Quoi ! le juste a le châtiment !
La vertu s’étonne et commence
A regarder Dieu fixement.

» Dieu se dérobe et nous échappe.
Quoi donc ! l’iniquité prévaut !
Le crime, voyant où Dieu frappe,
Rit d’un rire impie et dévot.

» Nous ne comprenons pas ses voies.
Comment ce Dieu des nations
Fera-t-il sortir tant de joies
De tant de désolations ?

» Ses desseins nous semblent contraires
A l’espoir qui luit dans tes yeux… »
- Mais qui donc, ô proscrits, mes frères,
Comprend le grand mystérieux ?

Qui donc a traversé l’espace,
La terre, l’eau, l’air et le feu,
Et l’étendue où l’esprit passe ?
Qui donc peut dire : « J’ai vu Dieu !

» J’ai vu Jéhova ! je le nomme !
Tout à l’heure il me réchauffait.
Je sais comment il a fait l’homme,
Comment il fait tout ce qu’il fait !

» J’ai vu cette main inconnue
Qui lâche en s’ouvrant l’âpre hiver,
Et les tonnerres dans la nue,
Et les tempêtes sur la mer,

» Tendre et ployer la nuit livide ;
Mettre une âme dans l’embryon ;
Appuyer dans l’ombre du vide
Le pôle du septentrion ;

» Amener l’heure où tout arrive ;
Faire au banquet du roi fêté
Entrer la mort, ce noir convive
Qui vient sans qu’on l’ait invité ;

» Créer l’araignée et sa toile,
Peindre la fleur, mûrir le fruit,
Et, sans perdre une seule étoile,
Mener tous les astres la nuit ;

» Arrêter la vague à la rive ;
Parfumer de roses l’été ;
Verser le temps comme une eau vive
Des urnes de l’éternité ;

» D’un souffle, avec ses feux sans nombre,
Faire, dans toute sa hauteur,
Frissonner le firmament sombre
Comme la tente d’un pasteur ;

» Attacher les globes aux sphères
Par mille invisibles liens…
Toutes ces choses sont très claires.
Je sais comment il fait ! j’en viens ! »

Qui peut dire cela ? personne.
Nuit sur nos coeurs ! nuit sur nos yeux !
L’homme est un vain clairon qui sonne.
Dieu seul parle aux axes des cieux.
"#,
    );

    too_long_text.push_str(&too_long_text.clone());
    too_long_text.push_str(&too_long_text.clone());

    let res = ENGINE.translate(too_long_text, InputLanguage::Automatic, Language::English);

    match res {
        Ok(_) => {
            panic!("should have been error 413: MaxTextSizeExceeded (translation did not fail)")
        }
        Err(Error::YandexAPIError(api::YandexError::MaxTextSizeExceeded)) => {
            println!("API failed on too long text")
        }
        Err(err) => panic!(
            "should have been error 413: MaxTextSizeExceeded (translation failed with {:?})",
            err
        ),
    }
}
