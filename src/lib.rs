pub mod core {
    pub mod content;
    pub mod bloom;
    pub mod tables;
}

pub mod routing {
    pub mod protocol;
    pub mod router;
    pub mod multicast;
}

pub mod apps {
    pub mod healthcare {
        pub mod dicom;
    }
}
