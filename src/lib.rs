//use allsorts;
//use nom;

pub mod language;
pub mod script;

use pest::{self, Parser};
#[macro_use] extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../fea.pest"]
pub struct FEAParser;

#[test]
fn test_feaparser() {
    let test = r#"@lol = [Qol Mol @lol];
    languagesystem DFLT dflt;
    language DEU required;
    include(te\)st);
    # include(te)st); would fail
    include (lol);

    anonymous jig { @lol = [lol]; {@Q = [Q R S T];} name ";}{"; {}; "}"; } jig;
    anonymous jjig { } jjig;
    anonymous jjig {} jjig;
    anonymous FIVE { {} {} {} } FIVE;
    # This'd be invalid FEA syntax due to tag mismatch. It's up to struct builder to check this! Not possible in a grammar.
    anonymous LAST {
        It's the end of the world as we know it
        And I feel fine!
    } FRST;

    name 0x3 0x1 0x411;

    feature mark {

    } mark;

    feature liga {
        featureNames {
            name 0x3 0x1 0x411 "Feature description for MS Platform, script Unicode, language Japanese";
            name "b";
        };
        name "C";
    } liga;

    table GDEF {
        LigatureCaretByPos lol 0;
    } GDEF;

    table head {
        FontRevision 0.0;
    } head;

    table OS/2 {
    FSType 4;
    Panose 2 15 0 0 2 2 8 2 9 4;
    TypoAscender 800;
    TypoDescender -200; # Note that TypoDescender is negative for descent below the baseline.
    winAscent 832;
    winDescent 321; # Note that winDescent is positive for descent below the baseline.
    UnicodeRange
        0   # Basic Latin
        1   # Latin-1 Supplement
        9   # Cyrillic
        55  # CJK Compatibility
        59  # CJK Unified Ideographs
        60  # Private Use Area
        ;
    CodePageRange
        1252    # Latin 1
        1251    # Cyrillic
        932     # JIS/Japan
        ;
    XHeight 400;
    CapHeight 600;
    WeightClass 800;
    WidthClass 3;
    Vendor "ADBE";
    FamilyClass 0x0805;  # Class 8 (Sans-serif), Subclass 5 (Neo-grotesque Gothic)
} OS/2;

feature aalt {
    featureNames {
        name "Fancy Q's";
    };
    lookup aalt_1 {
        sub Q from [Q.ss01 Q.ss02 Q.ss03];
    } aalt_1;
} aalt;


variation rvrn heavy {
    lookup symbols_heavy;
    lookup letters_heavy;
} rvrn;

anchorDef 120 120 ANCHOR_1;
anchorDef 120 -20 contourpoint 5 ANCHOR_2;
valueRecordDef -10 FIRST_KERN;
valueRecordDef <0 0 20 0> SECOND_KERN;

feature liga {
    sub A by B;
    sub @A by @B;
    sub B by A B C;
    sub f f by f_f;
    subtable;
    sub f i by f_i;
    sub f l by f_l;
    sub f l' lookup test;
    sub f l' by y;
    sub Q by NULL;
    lookup inside_lu {sub \NULL by NULL;}inside_lu;
} liga;
# comment ça va
#
    "#;
    let ast = FEAParser::parse(Rule::file, test);
    //eprintln!("{:?}", ast);
    use pest_ascii_tree::{self, into_ascii_tree};
    eprintln!("{}", into_ascii_tree(ast.unwrap()).unwrap());
}
