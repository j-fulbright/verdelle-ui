/// Visual variant of a [`crate::Toast`]. The set matches the types of shadcn/ui's Sonner
/// toasts, except that `Destructive` is Sonner's `error` (named as Alert and Button name it);
/// Sonner's `loading` is not provided.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Destructive,
}

impl ToastVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            ToastVariant::Default => "default",
            ToastVariant::Success => "success",
            ToastVariant::Info => "info",
            ToastVariant::Warning => "warning",
            ToastVariant::Destructive => "destructive",
        }
    }
}
