use memvid_core::Memvid;

pub fn create(filepath: &str) -> String {
    match Memvid::create(&filepath) {
        Ok(_) => {
            format!("Memory file created successfully at: {}", filepath)
        }
        Err(e) => {
            format!("Failed to create memory file at {}: {:?}", filepath, e)
        }
    }
}

pub fn open(filepath: &str) -> Result<Memvid, String> {
    match Memvid::open(&filepath) {
        Ok(memvid) => Ok(memvid),
        Err(e) => Err(format!(
            "Failed to open memory file at {}: {:?}",
            filepath, e
        )),
    }
}

pub fn put_bytes(
    data: &[u8],
    options: Option<crate::types::PutOptions>,
) -> Result<crate::types::FrameId, String> {
    let put_options = if let Some(opts) = options {
        opts.to_memvid_options()
    } else {
        memvid_core::PutOptions::default()
    };

    let mut memvid = match open("memvid.mvid") {
        Ok(memvid) => memvid,

        Err(e) => Err(format!("Failed to open memvid file: {:?}", e))?,
    };

    match memvid.put_bytes_with_options(data, put_options) {
        Ok(frame_id) => {
            memvid
                .commit()
                .map_err(|e| format!("Failed to commit changes: {:?}", e))?;
            Ok(frame_id)
        }
        Err(e) => Err(format!("Failed to put bytes into memory: {:?}", e)),
    }
}
