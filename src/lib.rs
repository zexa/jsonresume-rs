#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct JsonResume {
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

pub struct Basics {
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

pub struct Location {
    address: Option<String>,
    postalCode: Option<String>,
    city: Option<String>,
    countryCode: Option<String>,
    region: Option<String>,
}

pub struct Profile {
    network: Option<String>,
    username: Option<String>,
    url: Option<String>,
}

pub struct Work {
    company: Option<String>,
    position: Option<String>,
    website: Option<String>,
    startDate: Option<String>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}

pub struct Volunteer {
    organization: Option<String>,
    position: Option<String>,
    website: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}

pub struct Education {
    institution: Option<String>,
    area: Option<String>,
    studyType: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    gpa: Option<String>,
    courses: Option<Vec<String>>,
}

pub struct Award {
    title: Option<String>,
    date: Option<String>,
    awarder: Option<String>,
    summary: Option<String>,
}

pub struct Publication {
    name: Option<String>,
    publisher: Option<String>,
    releaseDate: Option<String>,
    website: Option<String>,
    summary: Option<String>,
}

pub struct Skill {
    name: Option<String>,
    level: Option<String>,
    keywords: Option<Vec<String>>,
}

pub struct Language {
    language: Option<String>,
    fluency: Option<String>,
}

pub struct Interest {
    name: Option<String>,
    keywords: Option<Vec<String>>,
}

pub struct Reference {
    name: Option<String>,
    reference: Option<String>,
}

