/// Represents a version of the HTTP spec.
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Rtp);

impl Version {
    /// `RTP/1.0`
    pub const RTP_10: Version = Version(Rtp::Rtp10);

    /// `RTP/2.0`
    pub const RTP_20: Version = Version(Rtp::Rtp20);
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Rtp {
    Rtp10,
    Rtp20,
}
