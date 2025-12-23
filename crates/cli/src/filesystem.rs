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

//you cannot access local memory that I know of, they way you would commiicate with
// a file system on a desktop,
#[cfg(feature = "wasm")]
pub fn makefs_wasm(vm: &mut VirtualMachine) {
    use std::convert::TryInto;

    let mut fs = vec![0u8; 65536];
    let mut start_block = 1;
    let mut file_index = 0;

    let user_bins: &[(&[u8], &str)] = &[
        (include_bytes!("../../../user/sh.bin"), "sh"),
        (include_bytes!("../../../user/ls.bin"), "ls"),
        (include_bytes!("../../../user/exit.bin"), "exit"),
        (include_bytes!("../../../user/hello.bin"), "hello"),
        (include_bytes!("../../../user/help.bin"), "help"),
    ];

    for (bin, name) in user_bins {
        let blocks_needed: usize = (bin.len() / 256) + 1;

        let block_start = start_block * 256;
        fs[block_start..block_start + bin.len()].copy_from_slice(bin);

        let zero_block_start = file_index * 16;
        fs[zero_block_start] = 1;
        fs[zero_block_start + 1] = start_block as u8;
        fs[zero_block_start + 2] = blocks_needed as u8;

        fs[zero_block_start + 3..zero_block_start + 11].fill(0);
        for (i, b) in name.as_bytes().iter().enumerate() {
            fs[zero_block_start + 3 + i] = *b;
        }

        start_block += blocks_needed;
        file_index += 1;
    }

    web_sys::console::log_1(
        &format!(
            "makefs_wasm: filesystem initialized, zero-block[0..{}]: {:?}",
            file_index * 16,
            &fs[0..file_index * 16]
        )
        .into(),
    );

    let boxed_fs: Box<[u8; 65536]> = fs.try_into().expect("Failed to convert FS to fixed size");
    vm.devices.disk().set(boxed_fs);
}
