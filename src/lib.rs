#![allow(unused_variables)]
#![allow(dead_code)]

pub mod widgets;
pub mod event;
pub mod state;
pub mod window;
pub mod widget_graph;
pub mod style;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
