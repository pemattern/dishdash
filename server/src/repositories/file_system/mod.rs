mod images;

use std::{path::PathBuf, sync::LazyLock};

pub use images::Images;

static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./data"));
