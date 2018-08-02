use codegen::bindings::{Blob, BlobTrigger, Http, HttpTrigger, Queue, QueueTrigger, TimerTrigger};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    InOut,
    Out,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Binding {
    Context,
    HttpTrigger(HttpTrigger),
    Http(Http),
    TimerTrigger(TimerTrigger),
    QueueTrigger(QueueTrigger),
    Queue(Queue),
    BlobTrigger(BlobTrigger),
    Blob(Blob),
}

impl Binding {
    pub fn name(&self) -> Option<&str> {
        match self {
            Binding::Context => None,
            Binding::HttpTrigger(b) => Some(&b.name),
            Binding::Http(b) => Some(&b.name),
            Binding::TimerTrigger(b) => Some(&b.name),
            Binding::QueueTrigger(b) => Some(&b.name),
            Binding::Queue(b) => Some(&b.name),
            Binding::BlobTrigger(b) => Some(&b.name),
            Binding::Blob(b) => Some(&b.name),
        }
    }

    pub fn is_context(&self) -> bool {
        match self {
            Binding::Context => true,
            _ => false,
        }
    }

    pub fn is_trigger(&self) -> bool {
        match self {
            Binding::HttpTrigger(_)
            | Binding::TimerTrigger(_)
            | Binding::QueueTrigger(_)
            | Binding::BlobTrigger(_) => true,
            Binding::Context | Binding::Http(_) | Binding::Queue(_) | Binding::Blob(_) => false,
        }
    }
}
