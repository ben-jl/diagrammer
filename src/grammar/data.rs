use std::{collections::HashMap, hash::Hash};

use ndarray::{ArrayBase,s, ArrayView, Axis, CowArray, IxDyn, RawData};

pub trait Dataset<'a, S : Sized, L> {
    fn backing_array(&self) -> &CowArray<'a, S, IxDyn>;
    fn get_labels(&self) -> Vec<&DatasetLabel<L>>;
    fn get_by_label(&self, label: &DatasetLabel<L>) -> &CowArray<'a, S, IxDyn>;
} 

#[derive(Debug,Clone, PartialEq, PartialOrd)]
pub struct DatasetLabel<T> {
    axis: Axis,
    labels: Vec<T>
}

#[derive(Debug)]
pub struct DatasetBase<'a, S : Sized, T> {
    labels: HashMap<Axis, DatasetLabel<T>>,
    raw_data: CowArray<'a, S, IxDyn>
}

impl <'a, S, FC> From<FC> for DatasetBase<'a, S, String> where FC : Into<CowArray<'a, S, IxDyn>> {
    fn from(data: FC) -> Self {
        let raw_data = data.into();
        let mut labels : HashMap<Axis, DatasetLabel<String>> = HashMap::new();
        for (a_ix, ad) in raw_data.axes().enumerate() {
            if let Some(ls) = labels.get_mut(&Axis(a_ix)) {
                ls.labels.push(format!("q_{}", a_ix));
            } else {
                let ds = DatasetLabel { axis: Axis(a_ix), labels: vec![format!("q_{}", a_ix)]};
                labels.insert(Axis(a_ix), ds);
            }
        }
        DatasetBase { raw_data, labels }
    }
}

impl <'a,  S : Sized, L> Dataset<'a, S, L> for DatasetBase<'a, S, L> {
    fn backing_array(&self) -> &CowArray<'a, S, IxDyn> {
        &self.raw_data
    }

    fn get_labels(&self) -> Vec<&DatasetLabel<L>> {
        self.labels.iter().map(|e| e.1).collect()
    }

    fn get_by_label(&self, label: &DatasetLabel<L>) -> &CowArray<'a, S, IxDyn> {

        unimplemented!()
    }
}