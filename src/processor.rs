use crate::traits::Processable;
use crate::data::MixedType;

pub fn process_data<T>(mut data: Vec<T>) -> Vec<T>
where
    T: Processable + Ord + Clone,
{
    data.sort_by(|a, b| {
        let ka = a.key();
        let kb = b.key();

        let na = ka.parse::<f64>();
        let nb = kb.parse::<f64>();

        match (na, nb) {
            (Ok(na), Ok(nb)) => na.partial_cmp(&nb).unwrap(),
            _ => ka.cmp(&kb),
        }
    });
    data
}

pub fn mixed_process_data(mut data: Vec<MixedType>) -> Vec<MixedType>
where
{
    data.sort_by(|a, b| {
        let ka = a.key();
        let kb = b.key();

        let na = ka.parse::<f64>();
        let nb = kb.parse::<f64>();

        match (na, nb) {
            (Ok(na), Ok(nb)) => na.partial_cmp(&nb).unwrap(),
            _ => ka.cmp(&kb),
        }
    });
    data
}