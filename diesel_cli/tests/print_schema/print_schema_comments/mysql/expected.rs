// @generated automatically by Diesel CLI.

diesel::table! {
    /// table comment
    with_comments (id) {
        /// column comment
        id -> Integer,
        /// The `column_without_comment` column of the `with_comments` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        column_without_comment -> Nullable<Integer>,
    }
}