use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pageable {
    pub start: Option<u32>,
    pub size: Option<u32>,
}

#[macro_export]
macro_rules! apply_pageable {
    ($select:expr, $page:expr) => {{
        let limit = $page.size.unwrap_or(50);
        let offset = $page.start.unwrap_or(0) * limit;
        $select.limit(limit.into()).offset(offset.into())
    }};
}
