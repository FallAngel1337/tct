use crate::cmd::ProgResult;
use crate::man::MANUALS;
use std::{
        fs, 
        io::{self, stdout, Write},
        path::Path
};

// TODO: Create a uninstall funciotion
pub fn install_manpages<T: AsRef<Path>, U: AsRef<Path>>(from: T, to: U) -> io::Result<()> {
    fs::create_dir_all(&to)?;
    
    for entry in fs::read_dir(from)? {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                install_manpages(entry.path(), to.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
            }
    };

    Ok(())
}

pub fn man(cmds: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    cmds.iter().skip(1).for_each(|cmd| {
        if MANUALS.contains_key(cmd.as_str()) {
            out.write_all(fs::read_to_string(MANUALS[cmd.as_str()]).unwrap().as_bytes())
                .expect("Cannot show manual");
        }
    });
    Ok(())
}
