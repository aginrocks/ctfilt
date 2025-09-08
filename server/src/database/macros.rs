macro_rules! database_object {
    ($name:ident $(<$($gen:tt),*>)? { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name $(<$($gen),*>)? {
            $($field)*
        }
    };
}
