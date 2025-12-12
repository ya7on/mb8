use mb8::vm::VirtualMachine;
use std::path::PathBuf;

pub fn makefs(user: Vec<PathBuf>, vm: &mut VirtualMachine) {
    let mut fs = vec![0u8; 65536];
    let mut blocks = 1;
    let mut files = 0;
    for path in user {
        let Ok(data) = std::fs::read(&path) else {
            continue;
        };
        let Ok(name) = path.file_stem().ok_or("Failed to get file name") else {
            continue;
        };

        let size = (data.len() / 256) + 1;

        // Add to zero block
        let zero_block_start = files * 16;
        fs[zero_block_start] = 1;
        fs[zero_block_start + 1] = blocks;
        fs[zero_block_start + 2] = size as u8;

        let chars = name.as_encoded_bytes();
        if chars.len() > 8 {
            eprintln!(
                "Error: File name {} is too long. Max 8 characters allowed.",
                name.to_string_lossy()
            );
            return;
        }
        for (i, c) in chars.iter().enumerate() {
            fs[zero_block_start + 3 + i] = *c;
        }

        let block_start = blocks as usize * 256;
        for (i, d) in data.iter().enumerate() {
            fs[block_start + i] = *d;
        }

        blocks += size as u8;
        files += 1;
    }

    let Ok(fs) = fs.try_into() else {
        eprintln!("Failed to convert file system");
        return;
    };
    vm.devices.disk().set(fs);
}
