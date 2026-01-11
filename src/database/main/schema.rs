diesel::table! {
    grades (student, subject, grade, time) {
        student -> Uuid,
        subject -> Int4,
        grade -> Float4,
        time -> Time,
    }
}