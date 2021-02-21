mod syntax_node;
mod wit;

pub use crate::wit::YggdrasilHost;

wit_bindgen::generate!({

    world: "host",
});

export!(YggdrasilHost);
