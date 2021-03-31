struct SemanticRoles{
    pub causers: Option<RoleMarker>,
    pub patient: Option<String>,
    pub range: Option<String>,
    pub starting_point: Option<String>,
    pub end_point: Option<String>,
    pub adverbial: Option<String>,
    pub beneficiary: Option<String>,
    pub condition: Option<String>,
    pub coordinated_arguments: Option<String>,
    pub degree: Option<String>,
    pub direction: Option<String>,
    pub discourse_marker: Option<String>,
    pub extend: Option<String>,
    pub frequency: Option<String>,
    pub locative: Option<String>,
    pub manner: Option<String>,
    pub purpose_or_reason: Option<String>,
    pub quantity: Option<String>,
    pub temporal: Option<String>,
    pub topic: Option<String>,
}

struct RoleMarker{
    pub start: u32,
    pub end: u32,
}


