struct NewApp {
    ApiPaths: Vec<(String, HashMap<String, Action>)>,
    pub ActivePaths: Vec<(&String, &HashMap<&String, &Action>)>,
    pub SelectedPath: (&String, &HashMap<&String, &Action>),
    pub SelectedAction: (&String, &Action),
}
