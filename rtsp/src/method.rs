

pub enum Method {
    Options,
    Describe,
    Setup,
    Play,
    Pause,
    Record,
    Announce,
    Teardown,
    GetParameter,
    SetParameter,
    Redirect
}

impl Method {
    // TODO: implement from_bytes: Parse from &[u8]

}