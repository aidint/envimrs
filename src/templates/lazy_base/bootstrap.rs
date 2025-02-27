use crate::get_config_dir;
use crate::templates::ensure_paths;
use std::fs;

pub fn deploy() {
    let init = include_bytes!("files/init.lua");
    let config = include_bytes!("files/lua/config/lazy.lua");

    let paths = [
        get_config_dir(),
        get_config_dir().join("lua"),
        get_config_dir().join("lua").join("config"),
    ];

    ensure_paths(&paths);

    match fs::write(get_config_dir().join("init.lua"), init) {
        Ok(_) => (),
        Err(e) => panic!("Writing init.lua: File system error: {e}"),
    };

    match fs::write(
        get_config_dir().join("lua").join("config").join("lazy.lua"),
        config,
    ) {
        Ok(_) => (),
        Err(e) => panic!("Writing lua/config/lazy.lua: File system error: {e}"),
    };
}
