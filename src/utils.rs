use std::sync::Arc;

// pub trait ToArc {
//     type Target;
//     fn to_arc(self) -> Arc<Self::Target>;
// }

// impl<T: Into<String>> ToArc for T {
//     type Target = str;
//     fn to_arc(self) -> Arc<Self::Target> {
//         Arc::from(self.into())
//     }
// }
