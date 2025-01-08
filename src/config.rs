pub mod cli {
    pub fn get_options() -> Vec<(
        &'static str,
        &'static str,
        Vec<(&'static str, char, bool, bool)>,
    )> {
        const REQUIRED: bool = true;
        const OPTIONAL: bool = false;
        const TAKES: bool = true;
        const NOTAKES: bool = false;

        vec![
            // (
            //     "example",
            //     "Example description",
            //     vec![
            //         ("option", 'o', REQUIRED, NOTAKES),
            //         ("option2", 'u', OPTIONAL, TAKES),
            //     ]
            // )
        ]
    }
}
