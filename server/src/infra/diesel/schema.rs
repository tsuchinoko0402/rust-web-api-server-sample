table! {
    pokemon (no) {
        no -> Int4,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Array<Text>,
    }
}
