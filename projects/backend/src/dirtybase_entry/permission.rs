pub(crate) enum AppPermission {
    Full,
    PlaceTip,
    Chat,
    ManageChat,
}

impl AsRef<str> for AppPermission {
    fn as_ref(&self) -> &str {
        match self {
            Self::Full => "*",
            Self::PlaceTip => "place-tip",
            Self::Chat => "chat",
            Self::ManageChat => "manage:chat",
        }
    }
}
