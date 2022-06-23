mod person;

use person::{Nationality, Person};

fn main() {
    let men: Vec<Person> = vec![
        Person::male("Alex")
            .characteristics(1.80, 80, Nationality::Russian)
            .preferences((1.50, 1.85), (48, 70), Nationality::NoMatter),
        Person::male("Bob")
            .characteristics(1.70, 71, Nationality::Russian)
            .preferences((1.45, 1.70), (48, 75), Nationality::Chinese),
        Person::male("Terry")
            .characteristics(1.95, 102, Nationality::Russian)
            .preferences((1.65, 1.80), (55, 70), Nationality::NoMatter),
        Person::male("Kevin")
            .characteristics(1.87, 98, Nationality::Russian)
            .preferences((1.55, 1.75), (50, 65), Nationality::British),
    ];

    let women: Vec<Person> = vec![
        Person::female("Fiona")
            .characteristics(1.70, 65, Nationality::British)
            .preferences((1.70, 1.90), (80, 100), Nationality::NoMatter),
        Person::female("Julia")
            .characteristics(1.58, 50, Nationality::British)
            .preferences((1.65, 1.90), (70, 90), Nationality::NoMatter),
        Person::female("Kate")
            .characteristics(1.60, 55, Nationality::Chinese)
            .preferences((1.70, 2.00), (80, 95), Nationality::Chinese),
        Person::female("Maria")
            .characteristics(1.78, 69, Nationality::Chinese)
            .preferences((1.90, 2.20), (86, 110), Nationality::Chinese),
    ];
}
