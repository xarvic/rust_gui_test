#![allow(unused_variables)]
#![allow(dead_code)]

pub mod widgets;
pub mod event;
pub mod state;
pub mod window;
pub(crate) mod widget_graph;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
