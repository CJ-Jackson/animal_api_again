pub mod animal;

pub fn error_flag<T, E>(flag: &mut bool, res: Result<T, E>) -> Result<T, E> {
    if res.is_err() {
        *flag = true;
    }
    res
}
