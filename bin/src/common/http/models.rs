macro_rules! auto {
    ($input_name:ident , $output_name:ident , { $($field_name:ident : $field_ty:ty),+ }) => {
        #[derive(Clone, Debug, Serialize, Deserialize, GraphQLInputObject)]
        pub struct $input_name {
            $(
                pub $field_name: $field_ty
            )*
        }

        #[derive(Clone, Debug, Serialize, Deserialize, GraphQLObject)]
        pub struct $input_name {
            $(
                pub $field_name: $field_ty
            )*
        }
    }
}

auto!(Foo, Bar, {
    baz: i32
})
