#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct JsonResume {
    basics: Option<Basics>,
    work: Option<Vec<Work>>,
    volunteer: Option<Vec<Volunteer>>,
    education: Option<Vec<Education>>,
    awards: Option<Vec<Award>>,
    publications: Option<Vec<Publication>>,
    skills: Option<Vec<Skill>>,
    languages: Option<Vec<Language>>,
    interests: Option<Vec<Interest>>,
    references: Option<Vec<Reference>>,
}

struct Basics {
    name: Option<String>,
    label: Option<String>,
    picture: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    summary: Option<String>,
    location: Option<Location>,
    profiles: Option<Vec<Profile>>,
}

struct Location {
    address: Option<String>,
    postalCode: Option<String>,
    city: Option<String>,
    countryCode: Option<String>,
    region: Option<String>,
}

struct Profile {
    network: Option<String>,
    username: Option<String>,
    url: Option<String>,
}

struct Work {
    company: Option<String>,
    position: Option<String>,
    website: Option<String>,
    startDate: Option<String>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}

struct Volunteer {
    organization: Option<String>,
    position: Option<String>,
    website: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}

struct Education {
    institution: Option<String>,
    area: Option<String>,
    studyType: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    gpa: Option<String>,
    courses: Option<Vec<String>>,
}

struct Award {
    title: Option<String>,
    date: Option<String>,
    awarder: Option<String>,
    summary: Option<String>,
}

struct Publication {
    name: Option<String>,
    publisher: Option<String>,
    releaseDate: Option<String>,
    website: Option<String>,
    summary: Option<String>,
}

struct Skill {
    name: Option<String>,
    level: Option<String>,
    keywords: Option<Vec<String>>,
}

struct Language {
    language: Option<String>,
    fluency: Option<String>,
}

struct Interest {
    name: Option<String>,
    keywords: Option<Vec<String>>,
}

struct Reference {
    name: Option<String>,
    reference: Option<String>,
}

