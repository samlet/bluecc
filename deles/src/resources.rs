mod position;
mod status_item;

pub trait Resource {
    const KIND: &'static str;
    const NAME: &'static str;
}

#[cfg(test)]
mod lib_tests {
    use super::*;
}


