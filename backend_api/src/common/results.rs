pub async fn merge_result_future<T, FUT>(fut: FUT) -> T
where
    FUT: Future<Output = Result<T, T>>,
{
    fut.await.unwrap_or_else(|err| err)
}
