use split_paragraphs::ParagraphsExt;

#[test]
fn line() {
    let text = "Hello world";
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("Hello world"));
    assert_eq!(paragraphs.next(), None);
}

#[test]
fn line2() {
    let text = "Hello world\n";
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("Hello world"));
    assert_eq!(paragraphs.next(), None);
}

#[test]
fn line3() {
    let text = "\n \nHello world\n\t\n\r";
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("Hello world"));
    assert_eq!(paragraphs.next(), None);
}

#[test]
fn line4() {
    let text = "
\u{0009}\n
\u{000A}\n
\u{000B}\n
\u{000C}\n
\u{000D}\n
\u{0020}\n
\u{0085}\n
\u{00A0}\n
\u{1680}\n
\u{2000}\n
\u{2001}\n
\u{2002}\n
\u{2003}\n
\u{2004}\n
\u{2005}\n
\u{2006}\n
\u{2007}\n
\u{2008}\n
\u{2009}\n
\u{200A}\n
\u{2028}\n
\u{2029}\n
\u{202F}\n
\u{205F}\n
\u{3000}\n
Hello world";
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("Hello world"));
    assert_eq!(paragraphs.next(), None);
}

#[test]
fn test1() {
    let text = "This is the first paragraph.\n \nThis is the second paragraph.\n\nThis is the third paragraph, which\nhas multiple lines.\n \n \nThis is the fourth paragraph";
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("This is the first paragraph."));
    assert_eq!(paragraphs.next(), Some("This is the second paragraph."));
    assert_eq!(
        paragraphs.next(),
        Some("This is the third paragraph, which\nhas multiple lines.")
    );
    assert_eq!(paragraphs.next(), Some("This is the fourth paragraph"));
}

#[test]
fn test1_raw() {
    let text = r#"This is the first paragraph.
 
This is the second paragraph.

This is the third paragraph, which
has multiple lines.
 
 
This is the fourth paragraph"#;
    let mut paragraphs = text.paragraphs();
    assert_eq!(paragraphs.next(), Some("This is the first paragraph."));
    assert_eq!(paragraphs.next(), Some("This is the second paragraph."));
    assert_eq!(
        paragraphs.next(),
        Some(
            r#"This is the third paragraph, which
has multiple lines."#
        )
    );
    assert_eq!(paragraphs.next(), Some("This is the fourth paragraph"));
}

const RESULTS: [&str; 13] = [
    "A",
    "B",
    "C",
    "D",
    "E\n  - 1\n  - 2",
    "F",
    "G",
    "H",
    "I\n  - 1\n    - a\n    - b\n  - 2",
    "J",
    "K",
    "L",
    "M",
];

const TEXT: &str = "
\u{0009}\u{2004}\nA\n
\u{000A}\u{2005}\nB\n
\u{000B}\u{2006}\nC\n
\u{000C}\u{2007}\nD\n
\u{000D}\u{2008}\nE\n  - 1\n  - 2\n
\u{0020}\u{2009}\nF\n
\u{0085}\u{200A}\nG\n
\u{00A0}\u{2028}\nH\n
\u{1680}\u{2029}\nI\n  - 1\n    - a\n    - b\n  - 2\n
\u{2000}\u{202F}\nJ\n
\u{2001}\u{205F}\nK\n
\u{2002}\u{3000}\nL\n
\u{2003} \t\r\nM\n";

#[test]
fn test_double_ended_iterator() {
    assert!(RESULTS.len() < 32);

    for i in 0..1u32 << RESULTS.len() {
        let mut paragraphs = TEXT.paragraphs();
        let mut results = RESULTS.into_iter();

        for j in 0..RESULTS.len() {
            if i & (1 << j) != 0 {
                assert_eq!(paragraphs.next(), results.next());
            } else {
                assert_eq!(paragraphs.next_back(), results.next_back());
            }
        }
    }
}
