pub mod animal;

pub fn error_flag<T, E>(flag: &mut bool, Res: Result<T, E>) -> Result<T, E> {
    if Res.is_err() {
        *flag = true;
    }
    Res
}
