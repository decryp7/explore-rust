use std::ops::Deref;
use std::sync::Arc;
use crate::build_version::BuildVersion;
use crate::publisher::{Event, Publisher, Subscription};

mod publisher;
mod build_version;

fn main() {
    let mut publisher = Publisher::default();
    let subscription = Arc::new(Subscription::new(Box::new(|version| {
        println!("{}", version);
    })));

    publisher.subscribe(Event::LatestVersion, subscription.clone());
    publisher.notify(Event::LatestVersion, BuildVersion::default());
    publisher.unsubscribe(Event::LatestVersion, subscription.clone());
    publisher.notify(Event::LatestVersion, BuildVersion::default());
}
