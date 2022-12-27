use person_gen::gen::{Person, Locale, Gender};

fn main() {
    let locale = Locale::US;
    let gender = Gender::Male;

    let person = Person::new(locale, gender).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    let first_name = person.generate_first_name();
    let last_name = person.generate_last_name();

    println!("{first_name} {last_name}");
}
