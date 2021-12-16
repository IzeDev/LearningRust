use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::fmt;

fn from() {
    struct Ssn {
        year: i32,
        month: i32,
        day: i32,
        extras: String,
    }
    impl From<String> for Ssn {
        fn from(text: String) -> Self {
            let year: i32 = (&text[0..4]).parse().unwrap();
            let month: i32 = (&text[4..6]).trim_start_matches("0").parse().unwrap();
            let day: i32 = (&text[6..8]).trim_start_matches("0").parse().unwrap();
            let extras: String = (&text[8..12]).parse().unwrap();
            Ssn {
                year: year,
                month: month,
                day: day,
                extras: extras,
            }
        }
    }

    let my_ssno = Ssn::from(String::from("199209254177"));
    // We get the into-implementation for free
    let my_other: Ssn = String::from("199209254177").into();

    println!(
        "Year: {0}, month: {1}, day: {2}, extras: {3}.",
        my_ssno.year, my_ssno.month, my_ssno.day, my_ssno.extras
    );

    println!(
        "Year: {0}, month: {1}, day: {2}, extras: {3}.",
        my_other.year, my_other.month, my_other.day, my_other.extras
    );
}

fn try_from() {
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    let my_odd_numb = EvenNumber::try_from(13);
    let my_even_numb = EvenNumber::try_from(12);
}

fn converting_to_string() {
    struct Ssn {
        year: i32,
        month: i32,
        day: i32,
        extras: String,
    }

    impl fmt::Display for Ssn {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Social security number: {0}-{1}-{2}-{3}",
                self.year, self.month, self.day, self.extras
            )
        }
    }

    let my_ssno = Ssn {
        year: 1992,
        month: 9,
        day: 25,
        extras: String::from("4177"),
    };

    println!("{}", my_ssno.to_string());
}

fn main() {
    from();
    try_from();
    converting_to_string();
}
