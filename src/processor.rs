use crate::traits::Processable;

pub fn process_data<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Processable + Ord + Clone,
{
    data.sort_by_key(|item| item.key());
    data
}