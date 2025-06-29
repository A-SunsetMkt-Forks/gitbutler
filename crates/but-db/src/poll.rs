use crate::DbHandle;
use bitflags::bitflags;

bitflags! {
    /// What kind of data to listen to
    #[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
    pub struct ItemKind: u8 {
        const Actions = 1 << 0;
        const Workflows = 1 << 1;
        const Assignments = 1 << 2;
    }
}

impl DbHandle {
    /// Register polling at `interval` for any `kind` of data and return a channel to be informed about the changes
    /// for the respective kind.
    /// Drop the receiver for the polling to stop.
    /// Note that this opens a new connection.
    ///
    /// ### Shortcoming
    ///
    /// The current implementation will send a change event the first time it starts up.
    pub fn poll_changes(
        &self,
        kind: ItemKind,
        interval: std::time::Duration,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<anyhow::Result<ItemKind>>> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut this = DbHandle::new_at_url(&self.url)?;
        std::thread::Builder::new()
            .name("Gitbutler-DB-watcher".into())
            .spawn(move || {
                let mut prev_assignments = Vec::new();
                let mut prev_workflows = Vec::new();
                let mut prev_actions = Vec::new();
                'outer: loop {
                    std::thread::sleep(interval);
                    for to_check in ItemKind::all().iter() {
                        let send_result = if kind & to_check == ItemKind::Actions {
                            let res = this.butler_actions().list(0, i64::MAX);
                            match res {
                                Ok((_num_items, items)) => {
                                    if items != prev_actions {
                                        prev_actions = items;
                                        tx.send(Ok(ItemKind::Actions))
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => tx.send(Err(e)),
                            }
                        } else if kind & to_check == ItemKind::Workflows {
                            let res = this.workflows().list(0, i64::MAX);
                            match res {
                                Ok((_num_items, items)) => {
                                    if items != prev_workflows {
                                        prev_workflows = items;
                                        tx.send(Ok(ItemKind::Workflows))
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => tx.send(Err(e)),
                            }
                        } else if kind & to_check == ItemKind::Assignments {
                            let res = this.hunk_assignments().list_all();
                            match res {
                                Ok(items) => {
                                    if items != prev_assignments {
                                        prev_assignments = items;
                                        tx.send(Ok(ItemKind::Assignments))
                                    } else {
                                        continue;
                                    }
                                }
                                Err(e) => tx.send(Err(e)),
                            }
                        } else {
                            eprintln!("BUG: didn't implement a branch for {to_check:?}");
                            break 'outer;
                        };
                        if send_result.is_err() {
                            break 'outer;
                        }
                    }
                }
            })?;
        Ok(rx)
    }
}
